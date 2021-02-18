mod yfinance;
mod sputil;

use yfinance::{types, query};
use sputil::datetime;

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
}
