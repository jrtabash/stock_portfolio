use std::error::Error;

use crate::util::datetime;
use crate::util::datetime::LocalDate;
use crate::util::price_type::PriceType;
use crate::datastore::datastore::DataStore;

pub type Price = PriceType;

// --------------------------------------------------------------------------------
// History Tag

#[inline(always)]
pub fn tag() -> &'static str {
    &"history"
}

// --------------------------------------------------------------------------------
// History Entry

pub struct HistoryEntry {
    pub date: LocalDate,
    pub open: Price,
    pub high: Price,
    pub low: Price,
    pub close: Price,
    pub adj_close: Price,
    pub volume: u64
}

impl HistoryEntry {
    pub fn new(date: LocalDate,
               open: Price,
               high: Price,
               low: Price,
               close: Price,
               adj_close: Price,
               volume: u64) -> Self {
        HistoryEntry {
            date: date,
            open: open,
            high: high,
            low: low,
            close: close,
            adj_close: adj_close,
            volume: volume
        }
    }

    pub fn parse_csv(csv: &str) -> Result<Self, Box<dyn Error>> {
        let values: Vec<&str> = csv.split(',').map(|field| field.trim()).collect();
        if values.len() == HistoryEntry::number_of_fields() {
            Ok(HistoryEntry {
                date: datetime::parse_date(&values[0])?,
                open: values[1].parse::<Price>()?,
                high: values[2].parse::<Price>()?,
                low: values[3].parse::<Price>()?,
                close: values[4].parse::<Price>()?,
                adj_close: values[5].parse::<Price>()?,
                volume: values[6].parse::<u64>()?
            })
        }
        else {
            Err(format!("HistoryEntry: Invalid csv data length={} expected=7", values.len()).into())
        }
    }

    #[inline(always)]
    pub fn number_of_fields() -> usize {
        return 7
    }
}

// --------------------------------------------------------------------------------
// History

pub struct History {
    symbol: String,
    entries: Vec<HistoryEntry>
}

impl History {
    pub fn new(symbol: &str) -> Self {
        History {
            symbol: String::from(symbol),
            entries: Vec::new()
        }
    }

    pub fn parse_csv(symbol: &str, csv: &str) -> Result<Self, Box<dyn Error>> {
        let mut hist = History::new(symbol);
        for line in csv.split('\n') {
            if line.is_empty() || line.starts_with(char::is_alphabetic) {
                continue;
            }
            hist.entries.push(HistoryEntry::parse_csv(line)?);
        }
        Ok(hist)
    }

    pub fn parse_filter_csv(symbol: &str, csv: &str, pred: impl Fn(&HistoryEntry) -> bool) -> Result<Self, Box<dyn Error>> {
        let mut hist = History::new(symbol);
        for line in csv.split('\n') {
            if line.is_empty() || line.starts_with(char::is_alphabetic) {
                continue;
            }
            let entry = HistoryEntry::parse_csv(line)?;
            if pred(&entry) {
                hist.entries.push(entry);
            }
        }
        Ok(hist)
    }

    pub fn ds_select_all(ds: &DataStore, symbol: &str) -> Result<Self, Box<dyn Error>> {
        let content = ds.select_symbol(tag(), symbol)?;
        History::parse_csv(symbol, &content)
    }

    pub fn ds_select_if(ds: &DataStore, symbol: &str, pred: impl Fn(&HistoryEntry) -> bool) -> Result<Self, Box<dyn Error>> {
        let content = ds.select_symbol(tag(), symbol)?;
        History::parse_filter_csv(symbol, &content, pred)
    }

    pub fn ds_select_last(ds: &DataStore, symbol: &str) -> Result<Self, Box<dyn Error>> {
        let content = ds.select_last(tag(), symbol)?;
        History::parse_csv(symbol, &content)
    }

    pub fn check_csv(csv: &str) -> Result<(), Box<dyn Error>> {
        let hist = History::parse_csv("history_check", csv)?;
        let cnt = hist.count();
        if cnt > 0 {
            let entries = hist.entries;
            let mut last_date = entries[0].date;
            for i in 1..cnt {
                let curr_date = entries[i].date;
                datetime::check_dup_or_back_gap(&last_date, &curr_date)?;
                last_date = curr_date;
            }
        }
        Ok(())
    }

    #[inline(always)]
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    #[inline(always)]
    pub fn entries(&self) -> &Vec<HistoryEntry> {
        &self.entries
    }

    #[inline(always)]
    pub fn count(&self) -> usize {
        self.entries.len()
    }
}

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_entry() {
        let csv = "2021-02-25,26.1,31.0,22.0,24.0,24.0,9000";
        let entry = HistoryEntry::parse_csv(&csv).unwrap();
        check_entry(&entry, datetime::make_date(2021, 2, 25), 26.1, 31.0, 22.0, 24.0, 24.0, 9000);
    }

    #[test]
    fn test_history_entry_with_whitespace() {
        let csv = "2021-02-25, 26.1,31.0  ,22.0, 24.0 ,24.0,9000\n";
        let entry = HistoryEntry::parse_csv(&csv).unwrap();
        check_entry(&entry, datetime::make_date(2021, 2, 25), 26.1, 31.0, 22.0, 24.0, 24.0, 9000);
    }

    #[test]
    fn test_history_entry_error() {
        let csv = "2021-02-25,26.1,31.0,22.0,24.0,24.0";
        assert!(HistoryEntry::parse_csv(&csv).is_err());

        let csv = "2021-02-25,26.1,31.0,22.0,24.0,24.0,9000,123";
        assert!(HistoryEntry::parse_csv(&csv).is_err());
    }

    #[test]
    fn test_history_parse_csv() {
        let csv = "2021-02-24,25.0,30.0,20.0,26.0,26.0,10000\n\
                   2021-02-25,26.1,31.0,22.0,24.0,24.0,9000\n\
                   2021-02-26,24.9,32.0,24.0,28.0,28.0,11000";
        let hist = History::parse_csv("AAPL", &csv).unwrap();
        assert_eq!(hist.symbol(), "AAPL");
        assert_eq!(hist.count(), 3);

        let entries = hist.entries();
        check_entry(&entries[0], datetime::make_date(2021, 2, 24), 25.0, 30.0, 20.0, 26.0, 26.0, 10000);
        check_entry(&entries[1], datetime::make_date(2021, 2, 25), 26.1, 31.0, 22.0, 24.0, 24.0, 9000);
        check_entry(&entries[2], datetime::make_date(2021, 2, 26), 24.9, 32.0, 24.0, 28.0, 28.0, 11000);
    }

    #[test]
    fn test_history_parse_csv_with_header() {
        let csv = "Date,Open,High,Low,Close,Adj Close,Volume\n\
                   2021-02-24,25.0,30.0,20.0,26.0,26.0,10000\n\
                   2021-02-25,26.1,31.0,22.0,24.0,24.0,9000\n\
                   2021-02-26,24.9,32.0,24.0,28.0,28.0,11000";
        let hist = History::parse_csv("AAPL", &csv).unwrap();
        assert_eq!(hist.symbol(), "AAPL");
        assert_eq!(hist.count(), 3);

        let entries = hist.entries();
        check_entry(&entries[0], datetime::make_date(2021, 2, 24), 25.0, 30.0, 20.0, 26.0, 26.0, 10000);
        check_entry(&entries[1], datetime::make_date(2021, 2, 25), 26.1, 31.0, 22.0, 24.0, 24.0, 9000);
        check_entry(&entries[2], datetime::make_date(2021, 2, 26), 24.9, 32.0, 24.0, 28.0, 28.0, 11000);
    }

    #[test]
    fn test_history_parse_csv_with_empty_lines() {
        let csv = "\n\
                   2021-02-24,25.0,30.0,20.0,26.0,26.0,10000\n\
                   \n\
                   2021-02-25,26.1,31.0,22.0,24.0,24.0,9000\n\
                   \n\
                   2021-02-26,24.9,32.0,24.0,28.0,28.0,11000\n\
                   \n";
        let hist = History::parse_csv("AAPL", &csv).unwrap();
        assert_eq!(hist.symbol(), "AAPL");
        assert_eq!(hist.count(), 3);

        let entries = hist.entries();
        check_entry(&entries[0], datetime::make_date(2021, 2, 24), 25.0, 30.0, 20.0, 26.0, 26.0, 10000);
        check_entry(&entries[1], datetime::make_date(2021, 2, 25), 26.1, 31.0, 22.0, 24.0, 24.0, 9000);
        check_entry(&entries[2], datetime::make_date(2021, 2, 26), 24.9, 32.0, 24.0, 28.0, 28.0, 11000);
    }

    #[test]
    fn test_history_parse_filter_csv() {
        let csv = "2021-02-24,25.0,30.0,20.0,26.0,26.0,10000\n\
                   2021-02-25,26.1,31.0,22.0,24.0,24.0,9000\n\
                   2021-02-26,24.9,32.0,24.0,28.0,28.0,11000";
        let hist = History::parse_filter_csv("AAPL", &csv, |entry| entry.date > datetime::make_date(2021, 2, 24)).unwrap();
        assert_eq!(hist.symbol(), "AAPL");
        assert_eq!(hist.count(), 2);

        let entries = hist.entries();
        check_entry(&entries[0], datetime::make_date(2021, 2, 25), 26.1, 31.0, 22.0, 24.0, 24.0, 9000);
        check_entry(&entries[1], datetime::make_date(2021, 2, 26), 24.9, 32.0, 24.0, 28.0, 28.0, 11000);
    }

    #[test]
    fn test_history_parse_filter_csv_with_header() {
        let csv = "Date,Open,High,Low,Close,Adj Close,Volume\n\
                   2021-02-24,25.0,30.0,20.0,26.0,26.0,10000\n\
                   2021-02-25,26.1,31.0,22.0,24.0,24.0,9000\n\
                   2021-02-26,24.9,32.0,24.0,28.0,28.0,11000";
        let hist = History::parse_filter_csv("AAPL", &csv, |entry| entry.date > datetime::make_date(2021, 2, 24)).unwrap();
        assert_eq!(hist.symbol(), "AAPL");
        assert_eq!(hist.count(), 2);

        let entries = hist.entries();
        check_entry(&entries[0], datetime::make_date(2021, 2, 25), 26.1, 31.0, 22.0, 24.0, 24.0, 9000);
        check_entry(&entries[1], datetime::make_date(2021, 2, 26), 24.9, 32.0, 24.0, 28.0, 28.0, 11000);
    }

    #[test]
    fn test_check_csv() {
        let csv = "2021-02-24,25.0,30.0,20.0,26.0,26.0,10000\n\
                   2021-02-25,26.1,31.0,22.0,24.0,24.0,9000\n\
                   2021-02-26,24.9,32.0,24.0,28.0,28.0,11000";
        assert!(History::check_csv(&csv).is_ok());

        let csv = "2021-02-24,25.0,30.0,20.0,26.0,26.0,10000\n\
                   2021-02-24,26.1,31.0,22.0,24.0,24.0,9000\n\
                   2021-02-26,24.9,32.0,24.0,28.0,28.0,11000";
        match History::check_csv(&csv) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(&format!("{}", err), "Duplicate date 2021-02-24")
        };

        let csv = "2021-02-24,25.0,30.0,20.0,26.0,26.0,10000\n\
                   2021-02-25,26.1,31.0,22.0,24.0,24.0,9000\n\
                   2021-02-23,24.9,32.0,24.0,28.0,28.0,11000";
        match History::check_csv(&csv) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(&format!("{}", err), "Earlier date 2021-02-23")
        };
    }

    fn check_entry(entry: &HistoryEntry,
                   date: LocalDate,
                   open: Price,
                   high: Price,
                   low: Price,
                   close: Price,
                   adj_close: Price,
                   volume: u64) {
        assert_eq!(entry.date, date);
        assert_eq!(entry.open, open);
        assert_eq!(entry.high, high);
        assert_eq!(entry.low, low);
        assert_eq!(entry.close, close);
        assert_eq!(entry.adj_close, adj_close);
        assert_eq!(entry.volume, volume);
    }
}
