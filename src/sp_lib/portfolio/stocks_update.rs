use crate::util::datetime;
use crate::util::error::Error;
use crate::portfolio::stock::{Price, Stock, StockList};
use crate::yfinance::query::HistoryQuery;
use crate::yfinance::types::{Interval, Events};
use crate::datastore::datastore::DataStore;
use crate::datastore::history::History;
use crate::datastore::dividends;

pub fn update_stock_from_csv(stock: &mut Stock, csv: &str) -> Result<bool, Error> {
    let hist = History::parse_csv(&stock.symbol, csv)?;
    if hist.count() > 0 {
        let latest = &hist.entries()[hist.count() - 1];
        if latest.adj_close > 0.0 {
            stock.set_latest_price(latest.adj_close, latest.date);
            return Ok(true)
        }
    }
    Ok(false)
}

pub fn update_stock(stock: &mut Stock, opt_day: Option<datetime::SPDate>) -> Result<bool, Error> {
    let day = opt_day.unwrap_or_else(datetime::today);
    let back_delta =
        if datetime::is_monday(&day) {
            -3
        } else if datetime::is_weekend(&day) {
            -2
        } else {
            -1
        };
    let mut query = HistoryQuery::new(
        stock.symbol.to_string(),
        datetime::date_plus_days(&day, back_delta),
        datetime::date_plus_days(&day, 1),
        Interval::Daily,
        Events::History);

    query.execute()?;
    match update_stock_from_csv(stock, &query.result) {
        Ok(updated) => Ok(updated),
        Err(e) => Err(format!("Failed to update {} - {}", stock.symbol, e).into())
    }
}

pub fn update_stock_from_ds(stock: &mut Stock, ds: &DataStore) -> Result<bool, Error> {
    let hist = History::ds_select_last(ds, &stock.symbol)?;
    if hist.count() != 1 {
        return Err(format!("Failed to find last history for {} in datastore {}", stock.symbol, ds).into())
    }

    if ds.symbol_exists(dividends::tag(), &stock.symbol) {
        let div = dividends::Dividends::ds_select_if(ds, &stock.symbol, |entry| entry.date > stock.date)?;
        stock.cum_dividend = stock.quantity as Price * div.entries().iter().fold(0.0, |cum, d| cum + d.price);
    }

    let entry = &hist.entries()[0];
    if entry.adj_close > 0.0 {
        stock.set_latest_price(entry.adj_close, entry.date);
        return Ok(true)
    }
    Ok(false)
}

pub fn update_stocks(stocks: &mut StockList, opt_day: Option<datetime::SPDate>) -> Result<usize, Error> {
    let mut count: usize = 0;
    for stock in stocks.iter_mut() {
        if update_stock(stock, opt_day)? {
            count += 1;
        }
    }
    Ok(count)
}

pub fn update_stocks_from_ds(stocks: &mut StockList, ds: &DataStore) -> Result<usize, Error> {
    let mut count: usize = 0;
    for stock in stocks.iter_mut() {
        if update_stock_from_ds(stock, ds)? {
            count += 1;
        }
    }
    Ok(count)
}
