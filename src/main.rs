use chrono::{Local, Duration};
mod yfinance;

fn main() {
    let mut query = yfinance::HistoryQuery::new(
        String::from("AAPL"),
        Local::today() + Duration::days(-5),
        Local::today() + Duration::days(1),
        yfinance::Interval::Daily,
        yfinance::Events::History);

    if query.execute() {
        println!("Query Result = \n{}", query.result);
    }
}
