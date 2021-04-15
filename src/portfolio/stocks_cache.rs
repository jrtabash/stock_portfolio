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
        CacheEntry { latest_price, latest_date }
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
        let table = Table::new();
        StocksCache { table }
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
                _ => Result::Err(Box::new(error))
            }
        }
    }

    pub fn from_csv(csv_data: &str) -> Result<StocksCache, Box<dyn Error>> {
        let mut cache = StocksCache::new();
        for cache_line in csv_data.split("\n") {
            if cache_line == "" {
                continue;
            }

            let cache_tokens: Vec<&str> = cache_line.split(",").collect();
            if cache_tokens.len() != 3 {
                return Result::Err(format!("StocksCache::from_csv - Invalid cache line '{}'", cache_line).into())
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
