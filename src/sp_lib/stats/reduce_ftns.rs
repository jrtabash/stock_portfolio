use std::error::Error;

pub type ReduceResult = Result<f64, Box<dyn Error>>;

pub fn reduce<Entry>(entries: &[Entry],
                     init: f64,
                     ftn: impl Fn(f64, &Entry) -> ReduceResult) -> ReduceResult {
    let mut ret = init;
    for entry in entries {
        ret = ftn(ret, entry)?;
    }
    Ok(ret)
}

#[inline(always)]
pub fn min<Entry>(entries: &[Entry],
                  extract: impl Fn(&Entry) -> f64) -> ReduceResult {
    reduce(&entries[1..],
           extract(&entries[0]),
           |ret, entry| {
               let value = extract(entry);
               Ok(if value < ret { value } else { ret })
           })
}

#[inline(always)]
pub fn max<Entry>(entries: &[Entry],
                  extract: impl Fn(&Entry) -> f64) -> ReduceResult {
    reduce(&entries[1..],
           extract(&entries[0]),
           |ret, entry| {
               let value = extract(entry);
               Ok(if value > ret { value } else { ret })
           })
}

#[inline(always)]
pub fn sum<Entry>(entries: &[Entry],
                  extract: impl Fn(&Entry) -> f64) -> ReduceResult {
    reduce(entries,
           0.0,
           |ret, entry| Ok(ret + extract(entry)))
}

#[inline(always)]
pub fn mean<Entry>(entries: &[Entry],
                   extract: impl Fn(&Entry) -> f64) -> ReduceResult {
    Ok(sum(entries, extract)? / entries.len() as f64)
}

pub fn variance<Entry>(entries: &[Entry],
                       extract: impl Fn(&Entry) -> f64) -> ReduceResult {
    let avg = mean(entries, &extract)?;
    let mut variance = reduce(entries,
                              0.0,
                              |v, e| Ok(v + (extract(e) - avg).powf(2.0)))?;
    if entries.len() > 1 {
        variance = variance / (entries.len() - 1) as f64;
    }
    Ok(variance)
}

#[inline(always)]
pub fn stddev<Entry>(entries: &[Entry],
                     extract: impl Fn(&Entry) -> f64) -> ReduceResult {
    Ok(variance(entries, extract)?.sqrt())
}

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::datastore::history::History;

    #[test]
    fn test_reduce() {
        let data = get_data();
        let entries = data.entries();
        let result = reduce(entries, 0.0, |r, e| Ok(r + e.open)).unwrap();
        assert_eq!(result, 53.75);
    }

    #[test]
    fn test_reduce_empty() {
        let data = History::new("BARR");
        let entries = data.entries();
        let result = reduce(entries, 0.0, |r, e| Ok(r + e.open)).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_min() {
        let data = get_data();
        let entries = data.entries();
        assert_eq!(min(entries, |e| e.close).unwrap(), 10.00);
        assert_eq!(min(entries, |e| e.volume as f64).unwrap(), 800.0);
    }

    #[test]
    fn test_max() {
        let data = get_data();
        let entries = data.entries();
        assert_eq!(max(entries, |e| e.close).unwrap(), 11.25);
        assert_eq!(max(entries, |e| e.volume as f64).unwrap(), 1200.0);
    }

    #[test]
    fn test_sum() {
        let data = get_data();
        let entries = data.entries();
        assert_eq!(sum(entries, |e| e.open).unwrap(), 53.75);
        assert_eq!(sum(entries, |e| e.volume as f64).unwrap(), 5000.0);
    }

    #[test]
    fn test_mean() {
        let data = get_data();
        let entries = data.entries();
        assert_eq!(mean(entries, |e| e.open).unwrap(), 10.75);
        assert_eq!(mean(entries, |e| e.volume as f64).unwrap(), 1000.0);
    }

    #[test]
    fn test_variance() {
        let data = get_data();
        let entries = data.entries();
        assert!(value_eql(variance(entries, |e| e.open).unwrap(), 0.156250));
        assert!(value_eql(variance(entries, |e| e.volume as f64).unwrap(), 25000.0));
    }

    #[test]
    fn test_stddev() {
        let data = get_data();
        let entries = data.entries();
        assert!(value_eql(stddev(entries, |e| e.open).unwrap(), 0.395285));
        assert!(value_eql(stddev(entries, |e| e.volume as f64).unwrap(), 158.113883));
    }

    fn get_data() -> History {
        History::parse_csv(
            "FOOO",
            "2021-10-04,10.50,11.00,10.00,10.75,10.75,1000\n\
             2021-10-05,10.75,11.25,10.25,11.00,11.00,1100\n\
             2021-10-06,11.00,11.50,10.50,11.25,11.25,1200\n\
             2021-10-07,11.25,11.25,9.75,10.25,10.25,900\n\
             2021-10-08,10.25,10.75,9.50,10.00,10.00,800").unwrap()
    }

    fn value_eql(lhs: f64, rhs: f64) -> bool {
        (lhs - rhs).abs() < 0.000001
    }
}
