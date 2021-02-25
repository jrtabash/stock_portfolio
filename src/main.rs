mod portfolio;
mod sputil;
mod yfinance;

use portfolio::{reports, stocks_reader, stocks_update};

fn main() {
    // TODO:JT - Add command line arguments:
    //   1. stocks file
    //   2. show groupby
    let stocks_file = String::from("/tmp/stocks_sample.csv");
    let show_groupby = true;

    let reader = stocks_reader::StocksReader::new(stocks_file);
    match reader.read() {
        Ok(mut stocks) => {
            let count_updated = stocks_update::update_stocks(&mut stocks);
            if count_updated == stocks.len() {
                reports::value_report(&stocks, show_groupby);
            }
            else {
                println!("update_stocks failed; updated={} expected={}", count_updated, stocks.len());
            }
        }
        Err(e) => println!("{}", e)
    }
}
