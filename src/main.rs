mod portfolio;
mod sputil;
mod yfinance;

use sputil::datetime;
use yfinance::{types, query};
use portfolio::{reports, algorithms, stocks_reader};

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

    let reader = stocks_reader::StocksReader::new(String::from("/tmp/stocks_sample.csv"));
    match reader.read() {
        Ok(mut stocks) => {
            for stock in stocks.iter_mut() {
                match stock.symbol.as_str() {
                    "AAPL" => stock.set_current_price(129.25),
                    "DELL" => stock.set_current_price(80.50),
                    _ => stock.set_current_price(25.00)
                }
            }

            reports::value_report(&stocks);
            println!("-----");
            println!("Stock group by:");
            for (symbol, size_price) in algorithms::stock_groupby(&stocks).iter() {
                println!("  {}: {} {:.2}", symbol, size_price.0, size_price.1);
            }
        }
        Err(e) => println!("{}", e)
    }
}
