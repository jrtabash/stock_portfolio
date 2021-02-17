use chrono::{Local, Duration};

mod yfinance;
use yfinance::{types, query};

fn main() {
    let mut query = query::HistoryQuery::new(
        String::from("AAPL"),
        Local::today() + Duration::days(-5),
        Local::today() + Duration::days(1),
        types::Interval::Daily,
        types::Events::History);

    if query.execute() {
        println!("Query Result = \n{}", query.result);
    }
}
