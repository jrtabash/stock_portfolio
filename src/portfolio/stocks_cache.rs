use std::io::prelude::*;
use std::env;
use std::path::PathBuf;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::collections::HashMap;

use crate::sputil::datetime;
use crate::sputil::datetime::LocalDate;
use crate::portfolio::stock::Price;

// --------------------------------------------------------------------------------
// Cache Entry

pub struct CacheEntry {
    pub latest_price: Price,
    pub latest_date: LocalDate
}

impl CacheEntry {
    pub fn new(latest_price: Price, latest_date: LocalDate) -> CacheEntry {
        CacheEntry {
            latest_price,
            latest_date
        }
    }

    pub fn update(self: &mut CacheEntry, price: Price, date: &LocalDate) {
        self.latest_price = price;
        self.latest_date = date.clone();
    }

    #[inline(always)]
    pub fn is_updated(self: &CacheEntry, today: &LocalDate) -> bool {
        self.latest_date.eq(today) || (datetime::is_friday(&self.latest_date) && datetime::is_weekend(&today))
    }
}

// --------------------------------------------------------------------------------
// Stocks Cache

type Table = HashMap<String, CacheEntry>;

pub struct StocksCache {
    table: Table
}

impl StocksCache {
    pub fn new() -> StocksCache {
        StocksCache {
            table: Table::new()
        }
    }

    #[inline(always)]
    pub fn add(self: &mut StocksCache, symbol: String, entry: CacheEntry) {
        self.table.insert(symbol, entry);
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn get(self: &StocksCache, symbol: &str) -> Option<&CacheEntry> {
        self.table.get(symbol)
    }

    #[inline(always)]
    pub fn get_mut(self: &mut StocksCache, symbol: &str) -> Option<&mut CacheEntry> {
        self.table.get_mut(symbol)
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn size(self: &StocksCache) -> usize {
        self.table.len()
    }

    pub fn from_cache_file() -> Result<StocksCache, Box<dyn Error>> {
        let cache_file = StocksCache::make_cache_file();
        match File::open(cache_file.as_path()) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut content = String::new();
                reader.read_to_string(&mut content)?;
                StocksCache::from_csv(&content)
            },
            Err(error) => match error.kind() {
                ErrorKind::NotFound => Ok(StocksCache::new()),
                _ => Err(Box::new(error))
            }
        }
    }

    pub fn from_csv(csv_data: &str) -> Result<StocksCache, Box<dyn Error>> {
        let mut cache = StocksCache::new();
        for cache_line in csv_data.lines() {
            if cache_line == "" {
                continue;
            }

            let cache_tokens: Vec<&str> = cache_line.split(",").collect();
            if cache_tokens.len() != 3 {
                return Err(format!("StocksCache::from_csv - Invalid cache line '{}'", cache_line).into())
            }

            let symbol = String::from(cache_tokens[0]);
            let date = datetime::parse_date(&cache_tokens[1])?;
            let price = cache_tokens[2].parse::<Price>()?;
            cache.add(symbol, CacheEntry::new(price, date));
        }
        Ok(cache)
    }

    pub fn save_cache_file(cache: &StocksCache) -> Result<(), Box<dyn Error>> {
        let cache_file = StocksCache::make_cache_file();
        let mut file = File::create(cache_file.as_path())?;
        for (symbol, cache_entry) in cache.table.iter() {
            write!(file, "{},{},{}\n", symbol, cache_entry.latest_date.format("%Y-%m-%d"), cache_entry.latest_price)?;
        }
        Ok(())
    }

    fn make_cache_file() -> PathBuf {
        let mut pbuf = env::temp_dir();
        pbuf.push("stock_portfolio_cache.spc");
        pbuf
    }
}

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_entry() {
        let mut cache_entry = CacheEntry::new(10.25, datetime::today());
        assert_eq!(cache_entry.latest_price, 10.25);
        assert_eq!(cache_entry.latest_date, datetime::today());

        let new_price = 20.52;
        let new_date = datetime::today_plus_days(1);

        cache_entry.update(new_price, &new_date);
        assert_eq!(cache_entry.latest_price, new_price);
        assert_eq!(cache_entry.latest_date, new_date);
    }

    #[test]
    fn test_cache_entry_is_updated() {
        let thu = datetime::make_date(2021, 3, 18);
        let fri = datetime::make_date(2021, 3, 19);
        let sat = datetime::make_date(2021, 3, 20);
        let sun = datetime::make_date(2021, 3, 21);
        let mon = datetime::make_date(2021, 3, 22);

        let mut cache_entry = CacheEntry::new(10.25, thu);
        assert!(cache_entry.is_updated(&thu));
        assert!(!cache_entry.is_updated(&fri));

        cache_entry.latest_date = fri.clone();
        assert!(cache_entry.is_updated(&fri));
        assert!(cache_entry.is_updated(&sat));
        assert!(cache_entry.is_updated(&sun));
        assert!(!cache_entry.is_updated(&mon));

        cache_entry.latest_date = mon.clone();
        assert!(cache_entry.is_updated(&mon));
    }

    #[test]
    fn test_stocks_cache() {
        let mut cache = StocksCache::new();
        assert_eq!(cache.size(), 0);

        cache.add(String::from("AAPL"), CacheEntry::new(125.0, datetime::today()));
        assert_eq!(cache.size(), 1);

        match cache.get("AAPL") {
            Some(entry) => {
                assert_eq!(entry.latest_price, 125.0);
                assert_eq!(entry.latest_date, datetime::today());
            },
            None => { assert!(false); }
        }

        cache.add(String::from("DELL"), CacheEntry::new(80.0, datetime::today_plus_days(-1)));
        assert_eq!(cache.size(), 2);

        match cache.get_mut("DELL") {
            Some(entry) => {
                entry.latest_price = 81.0;
                entry.latest_date = datetime::today();
            },
            None => { assert!(false); }
        }

        match cache.get("DELL") {
            Some(entry) => {
                assert_eq!(entry.latest_price, 81.0);
                assert_eq!(entry.latest_date, datetime::today());
            },
            None => { assert!(false); }
        }
    }

    #[test]
    fn test_stocks_cache_from_csv() {
        fn test_cache_entry(entry: Option<&CacheEntry>, price: Price, date: &LocalDate) -> bool {
            match entry {
                Some(ce) => format!("{:.2}", ce.latest_price) == format!("{:.2}", price) && ce.latest_date == *date,
                None => false
            }
        }

        let csv_data = "AAPL,2021-02-26,125.0\n\
                        DELL,2021-02-26,80.0\n";

        let cache = StocksCache::from_csv(&csv_data).unwrap();
        let date = datetime::parse_date("2021-02-26").unwrap();
        assert_eq!(cache.size(), 2);
        assert!(test_cache_entry(cache.get("AAPL"), 125.0, &date));
        assert!(test_cache_entry(cache.get("DELL"), 80.0, &date));

        match StocksCache::from_csv("bad csv data") {
            Ok(_) => { assert!(false); },
            Err(_) => {}
        }
    }

    #[test]
    fn test_stocks_cache_from_cache_file() {
        match StocksCache::from_cache_file() {
            Ok(_) => {},
            Err(_) => { assert!(false); }
        }
    }
}
