use crate::sputil::datetime::*;
use crate::portfolio::stock::*;
use crate::yfinance::query::*;
use crate::yfinance::types::*;

pub fn update_stock(stock: &mut Stock) -> bool {
    let mut query = HistoryQuery::new(
        stock.symbol.to_string(),
        today_plus_days(-4),
        today_plus_days(1),
        Interval::Daily,
        Events::History);

    if query.execute() {
        let history: Vec<&str> = query.result.split("\n").collect();
        if history.len() > 1 {
            let latest: Vec<&str> = history[history.len() - 1].split(",").collect();

            let current_price = latest[5].parse::<Price>().unwrap_or_else(|error| {
                println!("Failed to update {} current price - {}", stock.symbol, error);
                return 0.0
            });

            if current_price > 0.0 {
                stock.set_current_price(current_price);
                return true
            }
        }
    }
    return false
}

pub fn update_stocks(stocks: &mut StockList) -> usize {
    let mut count: usize = 0;
    for stock in stocks.iter_mut() {
        if update_stock(stock) {
            count += 1;
        }
    }
    return count;
}
