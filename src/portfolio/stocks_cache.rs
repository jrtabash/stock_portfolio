use std::io::prelude::*;
use std::env;
use std::path::PathBuf;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::collections::HashMap;
use chrono::{Date, Local};

use crate::sputil::datetime::*;
use crate::portfolio::stock::Price;

// --------------------------------------------------------------------------------
// Cache Entry

pub struct CacheEntry {
    pub latest_price: Price,
    pub latest_date: Date<Local>
}

impl CacheEntry {
    pub fn new(latest_price: Price, latest_date: Date<Local>) -> CacheEntry {
        CacheEntry { latest_price, latest_date }
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

    pub fn add(self: &mut StocksCache, symbol: String, entry: CacheEntry) {
        self.table.insert(symbol, entry);
    }

    #[allow(dead_code)]
    pub fn get(self: &StocksCache, symbol: &str) -> Option<&CacheEntry> {
        self.table.get(symbol)
    }

    pub fn get_mut(self: &mut StocksCache, symbol: &str) -> Option<&mut CacheEntry> {
        self.table.get_mut(symbol)
    }

    #[allow(dead_code)]
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
            let date = parse_date(&cache_tokens[1])?;
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
