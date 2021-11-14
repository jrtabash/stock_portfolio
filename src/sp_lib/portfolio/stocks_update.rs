use std::path::Path;
use std::error::Error;
use crate::util::datetime;
use crate::portfolio::stock::{Stock, StockList};
use crate::portfolio::stocks_cache::{CacheEntry, StocksCache};
use crate::yfinance::query::HistoryQuery;
use crate::yfinance::types::{Interval, Events};
use crate::datastore::datastore::DataStore;
use crate::datastore::history::History;

pub fn update_stock_from_csv(stock: &mut Stock, csv: &str) -> Result<bool, Box<dyn Error>> {
    let hist = History::parse_csv(&stock.symbol, csv)?;
    if hist.count() > 0 {
        let latest = &hist.entries()[hist.count() - 1];
        if latest.adj_close > 0.0 {
            stock.set_latest_price(latest.adj_close, latest.date.clone());
            return Ok(true)
        }
    }
    Ok(false)
}

pub fn update_stock(stock: &mut Stock) -> Result<bool, Box<dyn Error>> {
    let mut query = HistoryQuery::new(
        stock.symbol.to_string(),
        datetime::today_plus_days(-4),
        datetime::today_plus_days(1),
        Interval::Daily,
        Events::History);

    query.execute()?;
    match update_stock_from_csv(stock, &query.result) {
        Ok(updated) => Ok(updated),
        Err(e) => Err(format!("Failed to update {} - {}", stock.symbol, e).into())
    }
}

pub fn update_stock_from_ds(stock: &mut Stock, ds: &DataStore) -> Result<bool, Box<dyn Error>> {
    let hist = History::ds_select_last(ds, &stock.symbol)?;
    if hist.count() != 1 {
        return Err(format!("Failed to find last history for {} in datastore {}", stock.symbol, ds).into())
    }

    let entry = &hist.entries()[0];
    if entry.adj_close > 0.0 {
        stock.set_latest_price(entry.adj_close, entry.date.clone());
        return Ok(true)
    }
    Ok(false)
}

pub fn update_stocks(stocks: &mut StockList) -> Result<usize, Box<dyn Error>> {
    let mut count: usize = 0;
    for stock in stocks.iter_mut() {
        if update_stock(stock)? {
            count += 1;
        }
    }
    Ok(count)
}

pub fn update_stocks_from_ds(stocks: &mut StockList, ds: &DataStore) -> Result<usize, Box<dyn Error>> {
    let mut count: usize = 0;
    for stock in stocks.iter_mut() {
        if update_stock_from_ds(stock, ds)? {
            count += 1;
        }
    }
    Ok(count)
}

pub fn update_stocks_with_cache(stocks: &mut StockList, cache_file: &Path) -> Result<usize, Box<dyn Error>> {
    let today = datetime::today();
    let mut cache = StocksCache::from_cache_file(cache_file)?;
    let mut count: usize = 0;
    for stock in stocks.iter_mut() {
        match cache.get_mut(&stock.symbol) {
            Some(cache_entry) => {
                if cache_entry.is_updated(&today) {
                    stock.set_latest_price(cache_entry.latest_price, cache_entry.latest_date.clone());
                    count += 1;
                }
                else {
                    if update_stock(stock)? {
                        count += 1;
                        cache_entry.update(stock.latest_price, &stock.latest_date);
                    }
                }
            },
            None => {
                if update_stock(stock)? {
                    count += 1;
                    cache.add(stock.symbol.to_string(), CacheEntry::new(stock.latest_price, stock.latest_date.clone()));
                }
            }
        }
    }
    if let Err(error) = StocksCache::save_cache_file(&cache, cache_file) {
        eprintln!("Failed to save cache file - {}", error);
    }
    Ok(count)
}
