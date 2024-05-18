use std::cmp::Ordering;
use std::collections::HashMap;
use crate::util::error::Error;
use crate::portfolio::symbol_trait::GetSymbol;
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

pub fn latest_dividend(stocks: &StockList) -> Price {
    stocks.iter().map(|stock| stock.latest_dividend()).sum()
}

pub fn pct_change(stocks: &StockList) -> f64 {
    let base = base_notional(stocks);
    let latest = latest_notional(stocks);
    100.0 * (latest - base) / base
}

// Return (pct_change, pct_change_with_dividend)
pub fn calc_pct_change(stocks: &StockList) -> (f64, f64) {
    let base = base_notional(stocks);
    let net = latest_notional(stocks) - base;
    let div = cumulative_dividend(stocks);
    (100.0 * net / base, 100.0 * (net + div) / base)
}

pub fn stock_groupby<T>(stocks: &StockList,
                        init: fn (&Stock) -> T,
                        ftn: fn(&Stock, &T) -> T) -> HashMap<String, T> {
    let mut groupby: HashMap<String, T> = HashMap::new();
    for stock in stocks.iter() {
        let entry = groupby.entry(stock.symbol.to_string()).or_insert_with(|| init(stock));
        *entry = ftn(stock, entry);
    }
    groupby
}

// Group by stock symbol, and calcuate aggregate quantity, base value and current value.
pub fn stock_aggregate(stocks: &StockList) -> HashMap<String, (u32, Price, Price)> {
    stock_groupby(
        stocks,
        |_| (0, 0.0, 0.0),
        |stock, size_prices| {
            let sp = *size_prices;
            (sp.0 + stock.quantity, sp.1 + stock.base_notional(), sp.2 + stock.latest_notional())
        })
}

// Group by stock symbol, and calculate aggregate quantity and cumulative dividend
pub fn dividend_aggregate(stocks: &StockList) -> HashMap<String, (u32, Price)> {
    stock_groupby(
        stocks,
        |_| (0, 0.0),
        |stock, size_price| {
            let sp = *size_price;
            (sp.0 + stock.quantity, sp.1 + stock.cum_dividend)
        })
}

pub fn sort_stocks(stocks: &mut StockList, order_by: &str, desc: bool) -> Result<(), Error> {
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

        ("laddt", false) => stocks.sort_by(|lhs, rhs| lhs.latest_div_date.cmp(&rhs.latest_div_date)),
        ("laddt", true)  => stocks.sort_by(|lhs, rhs| rhs.latest_div_date.cmp(&lhs.latest_div_date)),

        ("ladiv", false) => stocks.sort_by(|lhs, rhs| price_type::price_cmp(lhs.latest_dividend(), rhs.latest_dividend())),
        ("ladiv", true) => stocks.sort_by(|lhs, rhs| price_type::price_cmp(rhs.latest_dividend(), lhs.latest_dividend())),

        ("div", false) => stocks.sort_by(|lhs, rhs| price_type::price_cmp(lhs.cum_dividend, rhs.cum_dividend)),
        ("div", true) => stocks.sort_by(|lhs, rhs| price_type::price_cmp(rhs.cum_dividend, lhs.cum_dividend)),

        ("yrdiv", false) => stocks.sort_by(|lhs, rhs| price_type::price_cmp(lhs.yearly_dividend(), rhs.yearly_dividend())),
        ("yrdiv", true) => stocks.sort_by(|lhs, rhs| price_type::price_cmp(rhs.yearly_dividend(), lhs.yearly_dividend())),

        ("dudiv", false) => stocks.sort_by(|lhs, rhs| price_type::price_cmp(lhs.daily_unit_dividend(), rhs.daily_unit_dividend())),
        ("dudiv", true) => stocks.sort_by(|lhs, rhs| price_type::price_cmp(rhs.daily_unit_dividend(), lhs.daily_unit_dividend())),

        ("divret", false) => stocks.sort_by(|lhs, rhs| price_type::price_cmp(lhs.cum_dividend_return(), rhs.cum_dividend_return())),
        ("divret", true) => stocks.sort_by(|lhs, rhs| price_type::price_cmp(rhs.cum_dividend_return(), lhs.cum_dividend_return())),

        _ => return Err(format!("Unsupported sort stocks order by '{}'", order_by).into())
    }
    Ok(())
}

pub fn sort_stocks_by_extra_ftn(stocks: &mut StockList, extra_ftn: impl Fn(&Stock) -> f64, desc: bool) {
    fn user_data_cmp(lhs: &Stock, rhs: &Stock) -> Ordering {
        if      lhs.user_data < rhs.user_data { Ordering::Less }
        else if lhs.user_data > rhs.user_data { Ordering::Greater }
        else                                  { Ordering::Equal }
    }

    for s in stocks.iter_mut() {
        s.user_data = extra_ftn(s);
    }

    if desc {
        stocks.sort_by(|lhs, rhs| user_data_cmp(rhs, lhs));
    }
    else {
        stocks.sort_by(user_data_cmp);
    }
}

pub fn filter_stocks(stocks: &mut StockList, filter_expr: &str, keep: bool) -> Result<(), Error> {
    let filter = stocks_filter::StocksFilter::from(filter_expr)?;
    filter.filter_stocks(stocks, keep);
    Ok(())
}

pub fn stock_base_dates(stocks: &StockList) -> HashMap<String, datetime::SPDate> {
    stock_groupby(
        stocks,
        |stock| stock.date,
        |stock, cur_date| if stock.date < *cur_date { stock.date } else { *cur_date })
}

pub fn match_list_to_symbols<Entry: GetSymbol>(entries: &mut Vec<Entry>, symbols: &Vec<String>) -> Result<(), Error> {
    let mut score: u32 = 0;
    let ordering: HashMap<&String, u32> = symbols
        .iter()
        .map(|sym| { score += 1; (sym, score) })
        .collect();

    entries.retain(|ent| ordering.contains_key(ent.get_symbol()));

    entries.sort_by(|ent1, ent2| {
        let scr1 = ordering.get(ent1.get_symbol()).unwrap();
        let scr2 = ordering.get(ent2.get_symbol()).unwrap();
        scr1.cmp(scr2)
    });

    Ok(())
}
