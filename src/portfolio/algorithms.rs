use std::collections::HashMap;
use crate::portfolio::stock::*;

pub fn latest_notional(stocks: &StockList) -> Price {
    stocks.iter().map(|stock| stock.latest_notional()).sum()
}

pub fn net_notional(stocks: &StockList) -> Price {
    stocks.iter().map(|stock| stock.net_notional()).sum()
}

pub fn base_notional(stocks: &StockList) -> Price {
    stocks.iter().map(|stock| stock.base_notional()).sum()
}

// Group by stock symbol, and calcuate aggregate quantity and current value.
pub fn stock_groupby(stocks: &StockList) -> HashMap<String, (u32, Price)> {
    let mut groupby = HashMap::new();
    for stock in stocks.iter() {
        let size_price = groupby.entry(stock.symbol.to_string()).or_insert((0, 0.0));
        (*size_price).0 += stock.quantity;
        (*size_price).1 += stock.latest_notional();
    }
    groupby
}
