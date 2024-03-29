use std::cmp::Ordering;

pub struct Description {
    count: usize,
    sum: f64,
    min: f64,
    max: f64,
    mean: f64,
    stddev: f64,
    lowerq: f64, // Lower Quartile
    median: f64, // Median
    upperq: f64  // Upper Quartile
}

impl Description {
    #[allow(clippy::too_many_arguments)]
    pub fn new(count: usize,
               sum: f64,
               min: f64,
               max: f64,
               mean: f64,
               stddev: f64,
               lowerq: f64,
               median: f64,
               upperq: f64) -> Self {
        Description {
            count,
            sum,
            min,
            max,
            mean,
            stddev,
            lowerq,
            median,
            upperq
        }
    }

    pub fn from_vec<Entry>(data: &Vec<Entry>, extract: impl Fn(&Entry) -> f64) -> Self {
        let mut values: Vec<f64> = data.iter().map(&extract).collect();
        values.sort_by(|l, r| {
            if      l < r { Ordering::Less }
            else if l > r { Ordering::Greater }
            else          { Ordering::Equal }
        });

        let mut count: usize = 0;
        let mut sum: f64 = 0.0;
        let mut min: f64 = 0.0;
        let mut max: f64 = 0.0;

        for value_ref in values.iter() {
            let value = *value_ref;
            if count > 0 {
                count += 1;
                sum += value;
                if value < min { min = value; }
                if value > max { max = value; }
            }
            else {
                count = 1;
                sum = value;
                min = value;
                max = value;
            }
        }

        let mean = if count > 0 { sum / count as f64 } else { 0.0 };

        let mut variance = 0.0;
        for entry in data {
            variance += (extract(entry) - mean).powf(2.0);
        }

        if count > 1 {
            variance /= (count - 1) as f64;
        }

        let (low, med, upp) = Self::calc_quartiles(&values);

        Description::new(
            count,
            sum,
            min,
            max,
            mean,
            variance.sqrt(),
            low,
            med,
            upp)
    }

    #[inline(always)]
    pub fn count(&self) -> usize {
        self.count
    }

    #[inline(always)]
    pub fn sum(&self) -> f64 {
        self.sum
    }

    #[inline(always)]
    pub fn min(&self) -> f64 {
        self.min
    }

    #[inline(always)]
    pub fn max(&self) -> f64 {
        self.max
    }

    #[inline(always)]
    pub fn mean(&self) -> f64 {
        self.mean
    }

    #[inline(always)]
    pub fn stddev(&self) -> f64 {
        self.stddev
    }

    #[inline(always)]
    pub fn lower_quartile(&self) -> f64 {
        self.lowerq
    }

    #[inline(always)]
    pub fn median(&self) -> f64 {
        self.median
    }

    #[inline(always)]
    pub fn upper_quartile(&self) -> f64 {
        self.upperq
    }

    fn calc_median(values: &[f64]) -> f64 {
        // Assume values are ordered
        let size = values.len();
        if size == 0 {
            return 0.0;
        }

        let idx = size / 2;
        if size % 2 == 0 {
            (values[idx] + values[idx - 1]) / 2.0
        }
        else {
            values[idx]
        }
    }

    fn calc_quartiles(values: &Vec<f64>) -> (f64, f64, f64) {
        // Assume values are ordered
        let size = values.len();
        let idx = size / 2;

        let med = Self::calc_median(values);
        let upp = Self::calc_median(&values[idx..size]);
        let low = if size % 2 == 0 {
            Self::calc_median(&values[0..idx])
        } else {
            Self::calc_median(&values[0..(idx + 1)])
        };

        (low, med, upp)
    }
}

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_description() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let desc = Description::from_vec(&data, |e| *e);
        assert_eq!(desc.count(), 5);
        assert!(value_eql(desc.sum(), 15.0));
        assert!(value_eql(desc.min(), 1.0));
        assert!(value_eql(desc.max(), 5.0));
        assert!(value_eql(desc.mean(), 3.0));
        assert!(value_eql(desc.stddev(), 1.581139));
        assert!(value_eql(desc.lower_quartile(), 2.0));
        assert!(value_eql(desc.median(), 3.0));
        assert!(value_eql(desc.upper_quartile(), 4.0));
    }

    #[test]
    fn test_description2() {
        let data = vec![0.78182178, 0.34128316, 0.76575515, 0.03832678, 0.86000713,
                        0.55843009, 0.52630449, 0.34965383, 0.64174317, 0.86802848];
        let desc = Description::from_vec(&data, |e| *e);
        assert_eq!(desc.count(), 10);
        assert!(value_eql(desc.sum(), 5.731354));
        assert!(value_eql(desc.min(), 0.038327));
        assert!(value_eql(desc.max(), 0.868028));
        assert!(value_eql(desc.mean(), 0.573135));
        assert!(value_eql(desc.stddev(), 0.268068));
        assert!(value_eql(desc.lower_quartile(), 0.3496538));
        assert!(value_eql(desc.median(), 0.600086));
        assert!(value_eql(desc.upper_quartile(), 0.78182178));
    }

    #[test]
    fn test_description_entry() {
        struct Entry {
            pub value: f64,
        }
        let data = vec![Entry{value: 1.0},
                        Entry{value: 2.0},
                        Entry{value: 3.0},
                        Entry{value: 4.0},
                        Entry{value: 5.0}];
        let desc = Description::from_vec(&data, |e| e.value);
        assert_eq!(desc.count(), 5);
        assert!(value_eql(desc.sum(), 15.0));
        assert!(value_eql(desc.min(), 1.0));
        assert!(value_eql(desc.max(), 5.0));
        assert!(value_eql(desc.mean(), 3.0));
        assert!(value_eql(desc.stddev(), 1.581139));
        assert!(value_eql(desc.lower_quartile(), 2.0));
        assert!(value_eql(desc.median(), 3.0));
        assert!(value_eql(desc.upper_quartile(), 4.0));
    }

    fn value_eql(lhs: f64, rhs: f64) -> bool {
        (lhs - rhs).abs() < 0.000001
    }
}
