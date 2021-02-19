mod portfolio;
mod sputil;
mod yfinance;

use sputil::datetime;
use yfinance::{types, query};
use portfolio::stock;

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
        datetime::today_plus_days(0),
        100,
        120.25);
    s.set_current_price(129.50);
    println!("{}", s);
    println!("      Base Price: ${:.2}", s.base_price);
    println!("       Net Price: ${:.2}", s.net_price());
    println!("   Base Notional: ${:.2}", s.base_notional());
    println!("Current Notional: ${:.2}", s.current_notional());
    println!("    Net Notional: ${:.2}", s.net_notional());
}
