use std::collections::HashMap;
use std::error::Error;
use crate::portfolio::stock::{Price, Stock, StockList};
use crate::util::{price_type, datetime};
use crate::portfolio::stocks_filter;

pub fn latest_notional(stocks: &StockList) -> Price {
    stocks.iter().map(|stock| stock.latest_notional()).sum()
}

pub fn net_notional(stocks: &StockList) -> Price {
    stocks.iter().map(|stock| stock.net_notional()).sum()
}

pub fn base_notional(stocks: &StockList) -> Price {
    stocks.iter().map(|stock| stock.base_notional()).sum()
}

pub fn cumulative_dividend(stocks: &StockList) -> Price {
    stocks.iter().map(|stock| stock.cum_dividend).sum()
}

pub fn pct_change(stocks: &StockList) -> f64 {
    let base: Price = base_notional(stocks);
    let latest: Price = latest_notional(stocks);
    100.0 * (latest - base) / base
}

pub fn stock_groupby<T>(stocks: &StockList,
                        init: fn (&Stock) -> T,
                        ftn: fn(&Stock, &T) -> T) -> HashMap<String, T> {
    let mut groupby: HashMap<String, T> = HashMap::new();
    for stock in stocks.iter() {
        let entry = groupby.entry(stock.symbol.to_string()).or_insert(init(stock));
        *entry = ftn(stock, entry);
    }
    groupby
}

// Group by stock symbol, and calcuate aggregate quantity and current value.
pub fn stock_aggregate(stocks: &StockList) -> HashMap<String, (u32, Price)> {
    stock_groupby(
        stocks,
        |_| (0, 0.0),
        |stock, size_price| {
            let sp = *size_price;
            (sp.0 + stock.quantity, sp.1 + stock.latest_notional())
        })
}

pub fn sort_stocks(stocks: &mut StockList, order_by: &str, desc: bool) -> Result<(), Box<dyn Error>> {
    match (order_by, desc) {
        ("symbol", false) => stocks.sort_by(|lhs, rhs| lhs.symbol.cmp(&rhs.symbol)),
        ("symbol", true)  => stocks.sort_by(|lhs, rhs| rhs.symbol.cmp(&lhs.symbol)),

        ("date", false) => stocks.sort_by(|lhs, rhs| lhs.date.cmp(&rhs.date)),
        ("date", true)  => stocks.sort_by(|lhs, rhs| rhs.date.cmp(&lhs.date)),

        ("value", false) => stocks.sort_by(|lhs, rhs| price_type::price_cmp(lhs.latest_notional(), rhs.latest_notional())),
        ("value", true)  => stocks.sort_by(|lhs, rhs| price_type::price_cmp(rhs.latest_notional(), lhs.latest_notional())),

        ("price", false) => stocks.sort_by(|lhs, rhs| price_type::price_cmp(lhs.latest_price, rhs.latest_price)),
        ("price", true)  => stocks.sort_by(|lhs, rhs| price_type::price_cmp(rhs.latest_price, lhs.latest_price)),

        ("net", false) => stocks.sort_by(|lhs, rhs| price_type::price_cmp(lhs.net_price(), rhs.net_price())),
        ("net", true)  => stocks.sort_by(|lhs, rhs| price_type::price_cmp(rhs.net_price(), lhs.net_price())),

        ("size", false) => stocks.sort_by(|lhs, rhs| lhs.quantity.cmp(&rhs.quantity)),
        ("size", true)  => stocks.sort_by(|lhs, rhs| rhs.quantity.cmp(&lhs.quantity)),

        ("type", false) => stocks.sort_by(|lhs, rhs| lhs.stype.cmp(&rhs.stype)),
        ("type", true)  => stocks.sort_by(|lhs, rhs| rhs.stype.cmp(&lhs.stype)),

        ("pct", false) => stocks.sort_by(|lhs, rhs| price_type::price_cmp(lhs.pct_change(), rhs.pct_change())),
        ("pct", true)  => stocks.sort_by(|lhs, rhs| price_type::price_cmp(rhs.pct_change(), lhs.pct_change())),

        ("days", false) => stocks.sort_by(|lhs, rhs| lhs.days_held.cmp(&rhs.days_held)),
        ("days", true) => stocks.sort_by(|lhs, rhs| rhs.days_held.cmp(&lhs.days_held)),

        ("div", false) => stocks.sort_by(|lhs, rhs| price_type::price_cmp(lhs.cum_dividend, rhs.cum_dividend)),
        ("div", true) => stocks.sort_by(|lhs, rhs| price_type::price_cmp(rhs.cum_dividend, lhs.cum_dividend)),

        _ => return Err(format!("Unsupported sort stocks order by '{}'", order_by).into())
    }
    Ok(())
}

pub fn filter_stocks(stocks: &mut StockList, filter_expr: &str, keep: bool) -> Result<(), Box<dyn Error>> {
    let filter = stocks_filter::StocksFilter::from(filter_expr)?;
    filter.filter_stocks(stocks, keep);
    Ok(())
}

pub fn stock_base_dates(stocks: &StockList) -> HashMap<String, datetime::LocalDate> {
    stock_groupby(
        stocks,
        |stock| stock.date.clone(),
        |stock, cur_date| if stock.date < *cur_date { stock.date.clone() } else { *cur_date })
}
