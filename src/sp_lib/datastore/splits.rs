use std::error::Error;

use crate::util::datetime;
use crate::util::datetime::SPDate;
use crate::datastore::datastore::DataStore;

// --------------------------------------------------------------------------------
// Splits Tag

#[inline(always)]
pub fn tag() -> &'static str {
    &"splits"
}

// --------------------------------------------------------------------------------
// Splits Entry

pub struct SplitEntry {
    pub date: SPDate,
    pub split: String
}

impl SplitEntry {
    pub fn new(date: SPDate, split: String) -> Self {
        SplitEntry {
            date: date,
            split: split
        }
    }

    pub fn parse_csv(csv: &str) -> Result<Self, Box<dyn Error>> {
        let values: Vec<&str> = csv.split(',').map(|field| field.trim()).collect();
        if values.len() == SplitEntry::number_of_fields() {
            Ok(SplitEntry {
                date: datetime::parse_date(&values[0])?,
                split: String::from(values[1])
            })
        }
        else {
            Err(format!("SplitEntry: Invalid csv data length={} expected=2", values.len()).into())
        }
    }

    #[inline(always)]
    pub fn number_of_fields() -> usize {
        return 2
    }
}

// --------------------------------------------------------------------------------
// Splits

pub struct Splits {
    symbol: String,
    entries: Vec<SplitEntry>
}

impl Splits {
    pub fn new(symbol: &str) -> Self {
        Splits {
            symbol: String::from(symbol),
            entries: Vec::new()
        }
    }

    pub fn parse_csv(symbol: &str, csv: &str) -> Result<Self, Box<dyn Error>> {
        let mut splt = Splits::new(symbol);
        for line in csv.split('\n') {
            if line.is_empty() || line.starts_with(char::is_alphabetic) {
                continue;
            }
            splt.entries.push(SplitEntry::parse_csv(line)?);
        }
        Ok(splt)
    }

    pub fn parse_filter_csv(symbol: &str, csv: &str, pred: impl Fn(&SplitEntry) -> bool) -> Result<Self, Box<dyn Error>> {
        let mut splt = Splits::new(symbol);
        for line in csv.split('\n') {
            if line.is_empty() || line.starts_with(char::is_alphabetic) {
                continue;
            }
            let entry = SplitEntry::parse_csv(line)?;
            if pred(&entry) {
                splt.entries.push(entry);
            }
        }
        Ok(splt)
    }

    pub fn ds_select_all(ds: &DataStore, symbol: &str) -> Result<Self, Box<dyn Error>> {
        let content = ds.select_symbol(tag(), symbol)?;
        Splits::parse_csv(symbol, &content)
    }

    pub fn ds_select_if(ds: &DataStore, symbol: &str, pred: impl Fn(&SplitEntry) -> bool) -> Result<Self, Box<dyn Error>> {
        let content = ds.select_symbol(tag(), symbol)?;
        Splits::parse_filter_csv(symbol, &content, pred)
    }

    pub fn ds_select_last(ds: &DataStore, symbol: &str) -> Result<Self, Box<dyn Error>> {
        let content = ds.select_last(tag(), symbol)?;
        Splits::parse_csv(symbol, &content)
    }

    pub fn ds_select_last_n(ds: &DataStore, symbol: &str, n: usize) -> Result<Self, Box<dyn Error>> {
        let content = ds.select_last_n(tag(), symbol, n)?;
        Splits::parse_csv(symbol, &content)
    }

    pub fn check_csv(csv: &str) -> Result<(), Box<dyn Error>> {
        let splt = Splits::parse_csv("splits_check", csv)?;
        let cnt = splt.count();
        if cnt > 0 {
            let entries = splt.entries;
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
    pub fn entries(&self) -> &Vec<SplitEntry> {
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
    fn test_split_entry() {
        let csv = "2021-02-25,2:1";
        let entry = SplitEntry::parse_csv(&csv).unwrap();
        check_split(&entry, datetime::make_date(2021, 2, 25), "2:1");
    }

    #[test]
    fn test_split_entry_with_whitespace() {
        let csv = "2021-02-25, 2:1 \n";
        let entry = SplitEntry::parse_csv(&csv).unwrap();
        check_split(&entry, datetime::make_date(2021, 2, 25), "2:1");
    }

    #[test]
    fn test_split_entry_error() {
        let csv = "2021-02-25";
        assert!(SplitEntry::parse_csv(&csv).is_err());

        let csv = "2021-02-25,2:1,3:2";
        assert!(SplitEntry::parse_csv(&csv).is_err());
    }

    #[test]
    fn test_splits_parse_csv() {
        let csv = "2019-02-24,2:1\n\
                   2020-02-21,3:2\n\
                   2021-02-26,4:3";
        let splt = Splits::parse_csv("SYMB", &csv).unwrap();
        assert_eq!(splt.symbol(), "SYMB");
        assert_eq!(splt.count(), 3);

        let entries = splt.entries();
        check_split(&entries[0], datetime::make_date(2019, 2, 24), "2:1");
        check_split(&entries[1], datetime::make_date(2020, 2, 21), "3:2");
        check_split(&entries[2], datetime::make_date(2021, 2, 26), "4:3");
    }

    #[test]
    fn test_splits_parse_csv_with_header() {
        let csv = "Date,Split\n\
                   2019-02-24,2:1\n\
                   2020-02-21,3:2\n\
                   2021-02-26,4:3";
        let splt = Splits::parse_csv("SYMB", &csv).unwrap();
        assert_eq!(splt.symbol(), "SYMB");
        assert_eq!(splt.count(), 3);

        let entries = splt.entries();
        check_split(&entries[0], datetime::make_date(2019, 2, 24), "2:1");
        check_split(&entries[1], datetime::make_date(2020, 2, 21), "3:2");
        check_split(&entries[2], datetime::make_date(2021, 2, 26), "4:3");
    }

    #[test]
    fn test_splits_parse_csv_with_empty_lines() {
        let csv = "\n\
                   2019-02-24,2:1\n\
                   \n\
                   2020-02-21,3:2\n\
                   \n\
                   2021-02-26,4:3\n\
                   \n";
        let splt = Splits::parse_csv("SYMB", &csv).unwrap();
        assert_eq!(splt.symbol(), "SYMB");
        assert_eq!(splt.count(), 3);

        let entries = splt.entries();
        check_split(&entries[0], datetime::make_date(2019, 2, 24), "2:1");
        check_split(&entries[1], datetime::make_date(2020, 2, 21), "3:2");
        check_split(&entries[2], datetime::make_date(2021, 2, 26), "4:3");
    }

    #[test]
    fn test_splits_parse_filter_csv() {
        let csv = "2019-02-24,2:1\n\
                   2020-02-21,3:2\n\
                   2021-02-26,4:3";
        let splt = Splits::parse_filter_csv("SYMB", &csv, |entry| entry.date > datetime::make_date(2019, 2, 24)).unwrap();
        assert_eq!(splt.symbol(), "SYMB");
        assert_eq!(splt.count(), 2);

        let entries = splt.entries();
        check_split(&entries[0], datetime::make_date(2020, 2, 21), "3:2");
        check_split(&entries[1], datetime::make_date(2021, 2, 26), "4:3");
    }

    #[test]
    fn test_splits_parse_filter_csv_with_header() {
        let csv = "Date,Split\n\
                   2019-02-24,2:1\n\
                   2020-02-21,3:2\n\
                   2021-02-26,4:3";
        let splt = Splits::parse_filter_csv("SYMB", &csv, |entry| entry.date > datetime::make_date(2019, 2, 24)).unwrap();
        assert_eq!(splt.symbol(), "SYMB");
        assert_eq!(splt.count(), 2);

        let entries = splt.entries();
        check_split(&entries[0], datetime::make_date(2020, 2, 21), "3:2");
        check_split(&entries[1], datetime::make_date(2021, 2, 26), "4:3");
    }

    #[test]
    fn test_check_csv() {
        let csv = "2019-02-24,2:1\n\
                   2020-02-21,3:2\n\
                   2021-02-26,4:3";
        assert!(Splits::check_csv(&csv).is_ok());

        let csv = "2019-02-24,2:1\n\
                   2019-02-24,2:1\n\
                   2021-02-26,3:2";
        match Splits::check_csv(&csv) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(&format!("{}", err), "Duplicate date 2019-02-24")
        };

        let csv = "2019-02-24,2:1\n\
                   2020-02-21,2:1\n\
                   2020-02-20,3:2";
        match Splits::check_csv(&csv) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(&format!("{}", err), "Earlier date 2020-02-20")
        };
    }

    fn check_split(entry: &SplitEntry, date: SPDate, split: &str) {
        assert_eq!(entry.date, date);
        assert_eq!(entry.split, split);
    }
}
