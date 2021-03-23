use crate::sputil::datetime::*;
use crate::portfolio::stock::*;
use crate::portfolio::stocks_cache::*;
use crate::yfinance::query::*;
use crate::yfinance::types::*;

pub fn update_stock_from_csv(stock: &mut Stock, csv: &str) -> bool {
    // Function assumes multi-line csv data.
    //
    // Example
    // -------
    // Date,Open,High,Low,Close,Adj Close,Volume\n
    // 2021-02-24,25.0,30.0,20.0,26.0,26.0,10000\n
    // 2021-02-25,26.1,31.0,22.0,24.0,24.0,9000\n
    // 2021-02-26,24.9,32.0,24.0,28.0,28.0,11000

    let mut success = false;
    match csv.rfind('\n') {
        Some(last_newline) => {
            let last_line = &csv[last_newline..];
            let latest: Vec<&str> = last_line.split(',').collect();

            match parse_date(&latest[0]) {
                Ok(latest_date) => {
                    let latest_price = latest[5].parse::<Price>().unwrap_or_else(|error| {
                        println!("Failed to update {} latest price - {}", stock.symbol, error);
                        return 0.0
                    });

                    if latest_price > 0.0 {
                        stock.set_latest_price(latest_price, latest_date);
                        success = true;
                    }
                }
                Err(e) => println!("Failed to update {} last date - {}", stock.symbol, e)
            }
        }
        None => {
            println!("Failed to update {} current price - no data", stock.symbol);
        }
    }
    success
}

pub fn update_stock(stock: &mut Stock) -> bool {
    let mut query = HistoryQuery::new(
        stock.symbol.to_string(),
        today_plus_days(-4),
        today_plus_days(1),
        Interval::Daily,
        Events::History);

    query.execute() && update_stock_from_csv(stock, &query.result)
}

pub fn update_stocks(stocks: &mut StockList) -> usize {
    let mut count: usize = 0;
    for stock in stocks.iter_mut() {
        if update_stock(stock) {
            count += 1;
        }
    }
    count
}

pub fn update_stocks_with_cache(stocks: &mut StockList) -> usize {
    let today = today();
    match StocksCache::from_cache_file() {
        Ok(mut cache) => {
            let mut count: usize = 0;
            for stock in stocks.iter_mut() {
                match cache.get_mut(&stock.symbol) {
                    Some(cache_entry) => {
                        if cache_entry.is_updated(&today) {
                            stock.set_latest_price(cache_entry.latest_price, cache_entry.latest_date.clone());
                            count += 1;
                        }
                        else {
                            if update_stock(stock) {
                                count += 1;
                                cache_entry.latest_price = stock.latest_price;
                                cache_entry.latest_date = stock.latest_date.clone();
                            }
                        }
                    },
                    None => {
                        if update_stock(stock) {
                            count += 1;
                            cache.add(stock.symbol.to_string(), CacheEntry::new(stock.latest_price, stock.latest_date.clone()));
                        }
                    }
                }
            }
            if let Err(error) = StocksCache::save_cache_file(&cache) {
                println!("Error: Failed to save cache file - {}", error);
            }
            count
        },
        Err(e) => {
            println!("Error: {}", e);
            0
        }
    }
}
