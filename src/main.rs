mod portfolio;
mod sputil;
mod yfinance;

use sputil::datetime;
use yfinance::{types, query};
use portfolio::{stock, reports, algorithms};

fn main() {
    let mut query = query::HistoryQuery::new(
        String::from("AAPL"),
        datetime::today_plus_days(-5),
        datetime::today_plus_days(1),
        types::Interval::Daily,
        types::Events::History);

    if query.execute() {
        println!("Query Result = \n{}", query.result);
    }

    println!("-----");

    let mut s = stock::Stock::new(
        String::from("AAPL"),
        datetime::today_plus_days(-3),
        100,
        120.25);
    s.set_current_price(129.50);
    println!("{}", s);
    println!("      Base Price: ${:.2}", s.base_price);
    println!("       Net Price: ${:.2}", s.net_price());
    println!("   Base Notional: ${:.2}", s.base_notional());
    println!("Current Notional: ${:.2}", s.current_notional());
    println!("    Net Notional: ${:.2}", s.net_notional());

    println!("-----");

    let mut s2 = stock::Stock::new(
        String::from("DELL"),
        datetime::today_plus_days(-2),
        100,
        79.21);
    s2.set_current_price(80.14);

    let mut s3 = stock::Stock::new(
        String::from("AAPL"),
        datetime::today_plus_days(-2),
        100,
        123.25);
    s3.set_current_price(129.50);

    let mut stocks = stock::StockList::new();
    stocks.push(s);
    stocks.push(s2);
    stocks.push(s3);

    reports::value_report(&stocks);

    println!("-----");

    println!("Stock group by:");
    for (symbol, size_price) in algorithms::stock_groupby(&stocks).iter() {
        println!("{}: {} {:.2}", symbol, size_price.0, size_price.1);
    }
}
