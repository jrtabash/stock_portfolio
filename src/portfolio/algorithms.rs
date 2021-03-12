use std::collections::HashMap;
use std::error::Error;
use crate::portfolio::stock::*;
use crate::sputil::price_type::*;

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

pub fn sort_stocks(stocks: &mut StockList, order_by: &str, desc: bool) -> Result<(), Box<dyn Error>> {
    match (order_by, desc) {
        ("symbol", false) => { stocks.sort_by(|lhs, rhs| lhs.symbol.cmp(&rhs.symbol)); Ok(()) },
        ("symbol", true)  => { stocks.sort_by(|lhs, rhs| rhs.symbol.cmp(&lhs.symbol)); Ok(()) },

        ("date", false) => { stocks.sort_by(|lhs, rhs| lhs.date.cmp(&rhs.date)); Ok(()) },
        ("date", true)  => { stocks.sort_by(|lhs, rhs| rhs.date.cmp(&lhs.date)); Ok(()) },

        ("value", false) => { stocks.sort_by(|lhs, rhs| price_cmp(lhs.latest_notional(), rhs.latest_notional())); Ok(()) },
        ("value", true)  => { stocks.sort_by(|lhs, rhs| price_cmp(rhs.latest_notional(), lhs.latest_notional())); Ok(()) },

        _ => Result::Err(format!("Unsupported sort stocks order by '{}'", order_by).into())
    }
}
