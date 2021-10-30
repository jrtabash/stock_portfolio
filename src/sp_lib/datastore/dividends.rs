use std::error::Error;

use crate::util::datetime;
use crate::util::datetime::LocalDate;
use crate::util::price_type::PriceType;

pub type Price = PriceType;

// --------------------------------------------------------------------------------
// Dividend Entry

pub struct DividendEntry {
    pub date: LocalDate,
    pub price: Price
}

impl DividendEntry {
    pub fn new(date: LocalDate, price: Price) -> Self {
        DividendEntry {
            date: date,
            price: price
        }
    }

    pub fn parse_csv(csv: &str) -> Result<Self, Box<dyn Error>> {
        let values: Vec<&str> = csv.split(',').map(|field| field.trim()).collect();
        if values.len() == DividendEntry::number_of_fields() {
            Ok(DividendEntry {
                date: datetime::parse_date(&values[0])?,
                price: values[1].parse::<Price>()?
            })
        }
        else {
            Err(format!("DividendEntry: Invalid csv data length={} expected=2", values.len()).into())
        }
    }

    #[inline(always)]
    pub fn number_of_fields() -> usize {
        return 2
    }
}

// --------------------------------------------------------------------------------
// Dividends

pub type DividendPred = fn(&DividendEntry) -> bool;

pub struct Dividends {
    symbol: String,
    entries: Vec<DividendEntry>
}

impl Dividends {
    pub fn new(symbol: &str) -> Self {
        Dividends {
            symbol: String::from(symbol),
            entries: Vec::new()
        }
    }

    pub fn parse_csv(symbol: &str, csv: &str) -> Result<Self, Box<dyn Error>> {
        let mut div = Dividends::new(symbol);
        for line in csv.split('\n') {
            if line.is_empty() || line.starts_with(char::is_alphabetic) {
                continue;
            }
            div.entries.push(DividendEntry::parse_csv(line)?);
        }
        Ok(div)
    }

    pub fn parse_filter_csv(symbol: &str, csv: &str, pred: DividendPred) -> Result<Self, Box<dyn Error>> {
        let mut div = Dividends::new(symbol);
        for line in csv.split('\n') {
            if line.is_empty() || line.starts_with(char::is_alphabetic) {
                continue;
            }
            let entry = DividendEntry::parse_csv(line)?;
            if pred(&entry) {
                div.entries.push(entry);
            }
        }
        Ok(div)
    }

    pub fn check_csv(csv: &str) -> Result<(), Box<dyn Error>> {
        let div = Dividends::parse_csv("dividends_check", csv)?;
        let cnt = div.count();
        if cnt > 0 {
            let entries = div.entries;
            let mut last_date = entries[0].date;
            for i in 1..cnt {
                let curr_date = entries[i].date;
                if curr_date == last_date {
                    return Err(format!("Duplicate date {}", curr_date.format("%Y-%m-%d")).into())
                }
                else if curr_date < last_date {
                    return Err(format!("Earlier date {}", curr_date.format("%Y-%m-%d")).into())
                }
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
    pub fn entries(&self) -> &Vec<DividendEntry> {
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
    fn test_dividend_entry() {
        let csv = "2021-02-25,1.24";
        let entry = DividendEntry::parse_csv(&csv).unwrap();
        check_dividend(&entry, datetime::make_date(2021, 2, 25), 1.24);
    }

    #[test]
    fn test_dividend_entry_with_whitespace() {
        let csv = "2021-02-25, 1.24 \n";
        let entry = DividendEntry::parse_csv(&csv).unwrap();
        check_dividend(&entry, datetime::make_date(2021, 2, 25), 1.24);
    }

    #[test]
    fn test_dividend_entry_error() {
        let csv = "2021-02-25";
        assert!(DividendEntry::parse_csv(&csv).is_err());

        let csv = "2021-02-25,1.24,2.01";
        assert!(DividendEntry::parse_csv(&csv).is_err());
    }

    #[test]
    fn test_dividends_parse_csv() {
        let csv = "2019-02-24,2.5\n\
                   2020-02-21,1.9\n\
                   2021-02-26,2.1";
        let div = Dividends::parse_csv("AAPL", &csv).unwrap();
        assert_eq!(div.symbol(), "AAPL");
        assert_eq!(div.count(), 3);

        let entries = div.entries();
        check_dividend(&entries[0], datetime::make_date(2019, 2, 24), 2.5);
        check_dividend(&entries[1], datetime::make_date(2020, 2, 21), 1.9);
        check_dividend(&entries[2], datetime::make_date(2021, 2, 26), 2.1);
    }

    #[test]
    fn test_dividends_parse_csv_with_header() {
        let csv = "Date,Price\n\
                   2019-02-24,2.5\n\
                   2020-02-21,1.9\n\
                   2021-02-26,2.1";
        let div = Dividends::parse_csv("AAPL", &csv).unwrap();
        assert_eq!(div.symbol(), "AAPL");
        assert_eq!(div.count(), 3);

        let entries = div.entries();
        check_dividend(&entries[0], datetime::make_date(2019, 2, 24), 2.5);
        check_dividend(&entries[1], datetime::make_date(2020, 2, 21), 1.9);
        check_dividend(&entries[2], datetime::make_date(2021, 2, 26), 2.1);
    }

    #[test]
    fn test_dividends_parse_csv_with_empty_lines() {
        let csv = "\n\
                   2019-02-24,2.5\n\
                   \n\
                   2020-02-21,1.9\n\
                   \n\
                   2021-02-26,2.1\n\
                   \n";
        let div = Dividends::parse_csv("AAPL", &csv).unwrap();
        assert_eq!(div.symbol(), "AAPL");
        assert_eq!(div.count(), 3);

        let entries = div.entries();
        check_dividend(&entries[0], datetime::make_date(2019, 2, 24), 2.5);
        check_dividend(&entries[1], datetime::make_date(2020, 2, 21), 1.9);
        check_dividend(&entries[2], datetime::make_date(2021, 2, 26), 2.1);
    }

    #[test]
    fn test_dividends_parse_filter_csv() {
        let csv = "2019-02-24,2.5\n\
                   2020-02-21,1.9\n\
                   2021-02-26,2.1";
        let div = Dividends::parse_filter_csv("AAPL", &csv, |entry| entry.date > datetime::make_date(2019, 2, 24)).unwrap();
        assert_eq!(div.symbol(), "AAPL");
        assert_eq!(div.count(), 2);

        let entries = div.entries();
        check_dividend(&entries[0], datetime::make_date(2020, 2, 21), 1.9);
        check_dividend(&entries[1], datetime::make_date(2021, 2, 26), 2.1);
    }

    #[test]
    fn test_dividends_parse_filter_csv_with_header() {
        let csv = "Date,Price\n\
                   2019-02-24,2.5\n\
                   2020-02-21,1.9\n\
                   2021-02-26,2.1";
        let div = Dividends::parse_filter_csv("AAPL", &csv, |entry| entry.date > datetime::make_date(2019, 2, 24)).unwrap();
        assert_eq!(div.symbol(), "AAPL");
        assert_eq!(div.count(), 2);

        let entries = div.entries();
        check_dividend(&entries[0], datetime::make_date(2020, 2, 21), 1.9);
        check_dividend(&entries[1], datetime::make_date(2021, 2, 26), 2.1);
    }

    #[test]
    fn test_check_csv() {
        let csv = "2019-02-24,2.5\n\
                   2020-02-21,1.9\n\
                   2021-02-26,2.1";
        assert!(Dividends::check_csv(&csv).is_ok());

        let csv = "2019-02-24,2.5\n\
                   2019-02-24,1.9\n\
                   2021-02-26,2.1";
        match Dividends::check_csv(&csv) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(&format!("{}", err), "Duplicate date 2019-02-24")
        };

        let csv = "2019-02-24,2.5\n\
                   2020-02-21,1.9\n\
                   2020-02-20,2.1";
        match Dividends::check_csv(&csv) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(&format!("{}", err), "Earlier date 2020-02-20")
        };
    }

    fn check_dividend(entry: &DividendEntry, date: LocalDate, price: Price) {
        assert_eq!(entry.date, date);
        assert_eq!(entry.price, price);
    }
}
