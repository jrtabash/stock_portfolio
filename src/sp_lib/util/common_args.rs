extern crate clap;

use clap::{Arg, ArgMatches};

// --------------------------------------------------------------------------------
// Common Version

pub fn app_version() -> &'static str {
    "0.2.1"
}

// --------------------------------------------------------------------------------
// Common Help Text

pub fn ds_root_help() -> &'static str {
    "Datastore root path"
}

pub fn ds_name_help() -> &'static str {
    "Datastore name (default: sp_datastore)"
}

pub fn stocks_file_help() -> &'static str {
    "CSV file containing stocks in portfolio, formatted as 'symbol,type,date,quantity,base_price' including a header line. \
     Supported type values include stock and etf"
}

// --------------------------------------------------------------------------------
// Common Arguments

pub fn ds_root(help: &str) -> Arg {
    Arg::with_name("ds_root")
        .short("r")
        .long("root")
        .help(if help.is_empty() { ds_root_help() } else { help })
        .required(true)
        .takes_value(true)
}

pub fn ds_name(help: &str) -> Arg {
    Arg::with_name("ds_name")
        .short("n")
        .long("name")
        .help(if help.is_empty() { ds_name_help() } else { help })
        .takes_value(true)
}

pub fn stocks_file(required: bool) -> Arg<'static, 'static> {
    Arg::with_name("stocks_file")
        .short("s")
        .long("stocks")
        .help(stocks_file_help())
        .required(required)
        .takes_value(true)
}

// --------------------------------------------------------------------------------
// Common Parsed Matches

pub fn parsed_ds_root(parsed_args: &ArgMatches) -> String {
    String::from(parsed_args.value_of("ds_root").unwrap())
}

pub fn parsed_ds_name(parsed_args: &ArgMatches) -> String {
    String::from(
        match parsed_args.value_of("ds_name") {
            Some(value) => value,
            None => "sp_datastore"
        })
}

pub fn parsed_stocks_file(parsed_args: &ArgMatches) -> Option<String> {
    match parsed_args.value_of("stocks_file") {
        Some(value) => Some(String::from(value)),
        None => None
    }
}
