use crate::sputil::datetime::*;
use crate::portfolio::stock::*;
use crate::yfinance::query::*;
use crate::yfinance::types::*;

pub fn update_stock(stock: &mut Stock) -> bool {
    let mut success = false;

    let mut query = HistoryQuery::new(
        stock.symbol.to_string(),
        today_plus_days(-4),
        today_plus_days(1),
        Interval::Daily,
        Events::History);

    if query.execute() {
        match query.result.rfind('\n') {
            Some(last_newline) => {
                let last_line = &query.result[last_newline..];
                let latest: Vec<&str> = last_line.split(',').collect();

                let current_price = latest[5].parse::<Price>().unwrap_or_else(|error| {
                    println!("Failed to update {} current price - {}", stock.symbol, error);
                    return 0.0
                });

                if current_price > 0.0 {
                    stock.set_current_price(current_price);
                    success = true;
                }
            }
            None => {
                println!("Failed to update {} current price - no data", stock.symbol);
            }
        }
    }

    success
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
