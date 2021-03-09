mod portfolio;
mod sputil;
mod yfinance;

extern crate clap;

use clap::{Arg, App};
use portfolio::{reports, stocks_reader, stocks_update};

fn main() {
    let parsed_args = App::new("Stock Portfolio Tool")
        .version("0.1.0")
        .about("Get latest close prices and report the gains and losses of stocks in portfolio.")
        .arg(Arg::with_name("stocks_file")
             .short("s")
             .long("stocks")
             .help("CSV file containing stocks in portfolio, formatted as 'symbol,date,quantity,base_price' including a header line")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("show_groupby")
             .short("g")
             .long("show-groupby")
             .help("Show quantities and current notional values grouped by symbol"))
        .arg(Arg::with_name("use_cache")
             .short("c")
             .long("use-cache")
             .help("Use local cache to store latest stock prices"))
        .get_matches();

    let stocks_file = parsed_args.value_of("stocks_file").unwrap();
    let show_groupby = parsed_args.is_present("show_groupby");
    let use_cache = parsed_args.is_present("use_cache");

    let reader = stocks_reader::StocksReader::new(String::from(stocks_file));
    match reader.read() {
        Ok(mut stocks) => {
            let count_updated =
                if use_cache {
                    stocks_update::update_stocks_with_cache(&mut stocks)
                } else {
                    stocks_update::update_stocks(&mut stocks)
                };
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
