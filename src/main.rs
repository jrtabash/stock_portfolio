mod portfolio;
mod sputil;
mod yfinance;

extern crate clap;

use std::process;
use std::collections::HashSet;
use clap::{Arg, App};
use portfolio::{stock, reports, stocks_reader, stocks_update, algorithms};

fn main() {
    let parsed_args = App::new("Stock Portfolio Tool")
        .version("0.1.1")
        .about("Get latest close prices and report the gains and losses of stocks in portfolio.")
        .arg(Arg::with_name("stocks_file")
             .short("s")
             .long("stocks")
             .help("CSV file containing stocks in portfolio, formatted as 'symbol,date,quantity,base_price' including a header line")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("order_by")
             .short("o")
             .long("orderby")
             .help("Order stocks by one of symbol, date or value")
             .takes_value(true))
        .arg(Arg::with_name("filter")
             .short("f")
             .long("filter")
             .help("Filter stocks by specified symbols; Comma separated list of symbols")
             .takes_value(true))
        .arg(Arg::with_name("show_groupby")
             .short("g")
             .long("show-groupby")
             .help("Show quantities and current notional values grouped by symbol"))
        .arg(Arg::with_name("use_cache")
             .short("c")
             .long("use-cache")
             .help("Use local cache to store latest stock prices"))
        .arg(Arg::with_name("desc")
             .short("d")
             .long("desc")
             .help("Used with order by option to sort in descending order"))
        .get_matches();

    let stocks_file = parsed_args.value_of("stocks_file").unwrap();
    let show_groupby = parsed_args.is_present("show_groupby");
    let use_cache = parsed_args.is_present("use_cache");
    let order_by = parsed_args.value_of("order_by");
    let desc = parsed_args.is_present("desc");
    let symbols_filter = parsed_args.value_of("filter");

    let reader = stocks_reader::StocksReader::new(String::from(stocks_file));
    match reader.read() {
        Ok(mut stocks) => {
            if !filter(&mut stocks, symbols_filter) {
                process::exit(1);
            }

            if !update(&mut stocks, use_cache) {
                process::exit(1);
            }

            if !sort(&mut stocks, order_by, desc) {
                process::exit(1);
            }

            reports::value_report(&stocks, show_groupby);
        }
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }
}

fn filter(stocks: &mut stock::StockList, opt_symbols: Option<&str>) -> bool {
    match opt_symbols {
        Some(symbols) => {
            let symbol_set: HashSet<&str> = symbols.split(',').map(|name| name.trim()).collect();
            stocks.retain(|stock| symbol_set.contains(stock.symbol.as_str()));
            true
        },
        None => true
    }
}

fn update(stocks: &mut stock::StockList, use_cache: bool) -> bool {
    let count =
        if use_cache {
            stocks_update::update_stocks_with_cache(stocks)
        } else {
            stocks_update::update_stocks(stocks)
        };

    let success = count == stocks.len();
    if !success {
        println!("update_stocks failed; updated={} expected={}", count, stocks.len());
    }
    success
}

fn sort(stocks: &mut stock::StockList, opt_order_by: Option<&str>, desc: bool) -> bool {
    match opt_order_by {
        Some(order_by) => {
            match algorithms::sort_stocks(stocks, &order_by, desc) {
                Ok(_) => true,
                Err(error) => {
                    println!("Error: {}", error);
                    false
                }
            }
        },
        None => true
    }
}
