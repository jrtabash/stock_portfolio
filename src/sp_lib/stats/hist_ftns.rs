use std::error::Error;
use crate::datastore::history::{History, HistoryEntry, Price};
use crate::util::datetime::LocalDate;
use crate::util::price_type::price_zero;

pub type DatePriceList = Vec<(LocalDate, Price)>;

// Volume Weighted Average Price
pub fn entries_vwap(entries: &[HistoryEntry]) -> Result<Price, Box<dyn Error>> {
    let mut notional: Price = 0.0;
    let mut volume: u64 = 0;
    for h in entries {
        notional += h.adj_close * h.volume as Price;
        volume += h.volume;
    }
    if volume == 0 {
        return Err(format!("entries_vwap: Cannot divide by zero total volume").into())
    }
    Ok(notional / volume as Price)
}

#[inline(always)]
pub fn hist_vwap(hist: &History) -> Result<Price, Box<dyn Error>> {
    entries_vwap(hist.entries())
}

// Moving Volume Weighted Average Price
pub fn entries_mvwap(entries: &[HistoryEntry], days: usize) -> Result<DatePriceList, Box<dyn Error>> {
    if days < 1 {
        return Err(format!("entries_mvwap: days < 1").into())
    }
    if days > entries.len() {
        return Err(format!("entries_mvwap: days > len").into())
    }

    let base = days - 1;
    let size = entries.len();

    let mut notional: Price = 0.0;
    let mut volume: u64 = 0;
    for i in 0..base {
        notional += entries[i].adj_close * entries[i].volume as Price;
        volume += entries[i].volume;
    }

    let mut prices: DatePriceList = Vec::with_capacity(size - base);
    for i in base..size {
        notional += entries[i].adj_close * entries[i].volume as Price;
        volume += entries[i].volume;
        if volume == 0 {
            return Err(format!("entries_mvwap: Cannot divide by zero total volume").into())
        }

        prices.push((entries[i].date.clone(), notional / volume as Price));

        let i0 = i - base;
        notional -= entries[i0].adj_close * entries[i0].volume as Price;
        volume -= entries[i0].volume;
    }

    Ok(prices)
}

#[inline(always)]
pub fn hist_mvwap(hist: &History, days: usize) -> Result<DatePriceList, Box<dyn Error>> {
    entries_mvwap(hist.entries(), days)
}

// Rate of Change
pub fn entries_roc(entries: &[HistoryEntry], days: usize) -> Result<DatePriceList, Box<dyn Error>> {
    if days < 1 {
        return Err(format!("entries_roc: days < 1").into())
    }
    if days > entries.len() {
        return Err(format!("entries_roc: days > len").into())
    }

    let size = entries.len();
    let mut rocs: DatePriceList = Vec::with_capacity(size - days);
    for i in days..size {
        let p0 = entries[i - days].adj_close;
        if price_zero(p0) {
            return Err(format!("entries_roc: Cannot divide by zero price").into())
        }
        rocs.push((entries[i].date.clone(), 100.0 * (entries[i].adj_close - p0) / p0));
    }

    Ok(rocs)
}

#[inline(always)]
pub fn hist_roc(hist: &History, days: usize) -> Result<DatePriceList, Box<dyn Error>> {
    entries_roc(hist.entries(), days)
}

// Percent change relative to first history point
pub fn entries_pctch(entries: &[HistoryEntry]) -> Result<DatePriceList, Box<dyn Error>> {
    let size = entries.len();

    if size < 2 {
        return Err(format!("entries_pctch: len < 2").into())
    }

    let p0 = entries[0].adj_close;
    if price_zero(p0) {
        return Err(format!("entries_pctch: Cannot divide by zero price").into())
    }

    let mut pctch: DatePriceList = Vec::with_capacity(entries.len() - 1);
    for i in 1..size {
        pctch.push((entries[i].date.clone(), 100.0 * (entries[i].adj_close - p0) / p0));
    }

    Ok(pctch)
}

#[inline(always)]
pub fn hist_pctch(hist: &History) -> Result<DatePriceList, Box<dyn Error>> {
    entries_pctch(hist.entries())
}

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::price_type::price_eql;
    use crate::util::datetime::{make_date, date_plus_days, is_weekend};

    #[test]
    fn test_entries_vwap() {
        let hist = hist_data();
        let entries = hist.entries();
        assert!(price_eql(entries_vwap(&entries).unwrap(), 145.282762));
        assert!(price_eql(entries_vwap(&entries[3..8]).unwrap(), 142.228875));
    }

    #[test]
    fn test_hist_vwap() {
        let hist = hist_data();
        assert!(price_eql(hist_vwap(&hist).unwrap(), 145.282762));
    }

    #[test]
    fn test_entries_mvwap() {
        let hist = hist_data();
        let entries = hist.entries();
        let expect = expect_mvwap();
        let actual = entries_mvwap(&entries, 5).unwrap();
        assert!(date_prices_eql(&actual, &expect));

        let actual = entries_mvwap(&entries[3..10], 5).unwrap();
        assert!(date_prices_eql(&actual, &expect[3..6]));
    }

    #[test]
    fn test_hist_mvwap() {
        let hist = hist_data();
        let expect = expect_mvwap();
        let actual = hist_mvwap(&hist, 5).unwrap();
        assert!(date_prices_eql(&actual, &expect));
    }

    #[test]
    fn test_entries_roc() {
        let hist = hist_data();
        let entries = hist.entries();
        let expect = expect_roc1();
        let actual = entries_roc(&entries, 1).unwrap();
        assert!(date_prices_eql(&actual, &expect));

        let actual = entries_roc(&entries[3..10], 1).unwrap();
        assert!(date_prices_eql(&actual, &expect[3..9]));
    }

    #[test]
    fn test_entries_roc3() {
        let hist = hist_data();
        let entries = hist.entries();
        let expect = expect_roc3();
        let actual = entries_roc(&entries, 3).unwrap();
        assert!(date_prices_eql(&actual, &expect));

        let actual = entries_roc(&entries[3..10], 3).unwrap();
        assert!(date_prices_eql(&actual, &expect[3..7]));
    }

    #[test]
    fn test_hist_roc() {
        let hist = hist_data();
        let expect = expect_roc1();
        let actual = hist_roc(&hist, 1).unwrap();
        assert!(date_prices_eql(&actual, &expect));
    }

    #[test]
    fn test_hist_pctch() {
        let hist = hist_data();
        let expect = expect_pctch();
        let actual = hist_pctch(&hist).unwrap();
        println!("{:?}", actual);
        println!("{:?}", expect);
        assert!(date_prices_eql(&actual, &expect));
    }

    fn hist_data() -> History {
        History::parse_csv(
            "AAPL",
            "2021-10-01,141.899994,142.919998,139.110001,142.649994,142.442108,94639600\n\
             2021-10-04,141.759995,142.210007,138.270004,139.139999,138.937225,98322000\n\
             2021-10-05,139.490005,142.240005,139.360001,141.110001,140.904358,80861100\n\
             2021-10-06,139.470001,142.149994,138.369995,142.000000,141.793060,83221100\n\
             2021-10-07,143.059998,144.220001,142.720001,143.289993,143.081177,61732700\n\
             2021-10-08,144.029999,144.179993,142.559998,142.899994,142.691742,58718700\n\
             2021-10-11,142.270004,144.809998,141.809998,142.809998,142.601883,64452200\n\
             2021-10-12,143.229996,143.250000,141.039993,141.509995,141.303772,73035900\n\
             2021-10-13,141.240005,141.399994,139.199997,140.910004,140.704651,78762700\n\
             2021-10-14,142.110001,143.880005,141.509995,143.759995,143.550491,69907100\n\
             2021-10-15,143.770004,144.899994,143.509995,144.839996,144.628922,67885200\n\
             2021-10-18,143.449997,146.839996,143.160004,146.550003,146.336426,85589200\n\
             2021-10-19,147.009995,149.169998,146.550003,148.759995,148.543198,76378900\n\
             2021-10-20,148.699997,149.750000,148.119995,149.259995,149.042480,58418800\n\
             2021-10-21,148.809998,149.639999,147.869995,149.479996,149.262146,61421000\n\
             2021-10-22,149.690002,150.179993,148.639999,148.690002,148.473312,58883400\n\
             2021-10-25,148.679993,149.369995,147.619995,148.639999,148.423386,50720600\n\
             2021-10-26,149.330002,150.839996,149.009995,149.320007,149.102402,60893400\n\
             2021-10-27,149.360001,149.729996,148.490005,148.850006,148.633087,56094900\n\
             2021-10-28,149.820007,153.169998,149.720001,152.570007,152.347656,100077900\n\
             2021-10-29,147.220001,149.940002,146.410004,149.800003,149.581696,124850400").unwrap()
    }

    fn expect_mvwap() -> DatePriceList {
        let date0 = make_date(2021, 10, 07);
        let prices = vec![141.287520, 141.217479, 142.115587, 142.228876, 141.980042,
                          142.101272, 142.488000, 143.346371, 144.789113, 146.380997,
                          147.452895, 148.191956, 148.749638, 148.877932, 148.796869,
                          149.797197, 149.927214];
        make_date_prices(&prices, date0)
    }

    fn expect_roc1() -> DatePriceList {
        let date0 = make_date(2021, 10, 04);
        let prices = vec![-2.460566,  1.415843,  0.630712, 0.908448, -0.272177,
                          -0.062974, -0.910304, -0.423995, 2.022562,  0.751255,
                          1.180610,  1.508012,  0.336119, 0.147384, -0.528488,
                          -0.033626,  0.457485, -0.314760, 2.499153, -1.815557];
        make_date_prices(&prices, date0)
    }

    fn expect_roc3() -> DatePriceList {
        let date0 = make_date(2021, 10, 06);
        let prices = vec![-0.455657, 2.982607,  1.268508,  0.570424, -1.242235,
                          -1.392576, 0.665214,  2.353192,  4.002550,  3.478014,
                          3.051642, 1.999310, -0.047047, -0.415380, -0.107022,
                          0.107611, 2.643970,  0.321452];
        make_date_prices(&prices, date0)
    }

    fn expect_pctch() -> DatePriceList {
        let date0 = make_date(2021, 10, 04);
        let prices = vec![-2.460567, -1.079561, -0.455657, 0.448652, 0.175253,
                          0.112168, -0.799157, -1.219764, 0.778129, 1.53523,
                          2.733965, 4.283207, 4.633722, 4.787937, 4.234144,
                          4.199094, 4.67579, 4.346312, 6.954087, 5.012273];
        make_date_prices(&prices, date0)
    }

    fn make_date_prices(prices: &Vec<Price>, date0: LocalDate) -> DatePriceList {
        let mut days: i64 = 0;
        prices
            .iter()
            .map(|p| {
                let mut date = date_plus_days(&date0, days);
                while is_weekend(&date) {
                    days += 1;
                    date = date_plus_days(&date0, days);
                }
                days += 1;
                (date, *p)
            })
            .collect()
    }

    fn date_prices_eql(actual: &[(LocalDate, Price)], expect: &[(LocalDate, Price)]) -> bool {
        if actual.len() != expect.len() {
            return false
        }

        let true_cnt: usize = actual
            .iter()
            .zip(expect.iter())
            .map(|(dp1, dp2)| dp1.0 == dp2.0 && price_eql(dp1.1, dp2.1))
            .filter(|b| *b)
            .count();
        return true_cnt == expect.len()
    }
}
