extern crate clap;

use std::env;
use std::error::Error;
use clap::{Arg, ArgMatches};

// --------------------------------------------------------------------------------
// Common Version

pub fn app_version() -> &'static str {
    "0.2.1"
}

// --------------------------------------------------------------------------------
// Common Help Text

pub fn ds_root_help() -> &'static str {
    "Datastore root path (default: value of HOME environment variable)"
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

pub fn ds_root() -> Arg<'static, 'static> {
    Arg::with_name("ds_root")
        .short("r")
        .long("root")
        .help(ds_root_help())
        .takes_value(true)
}

pub fn ds_name() -> Arg<'static, 'static> {
    Arg::with_name("ds_name")
        .short("n")
        .long("name")
        .help(ds_name_help())
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

pub fn parsed_ds_root(parsed_args: &ArgMatches) -> Result<String, Box<dyn Error>> {
    match parsed_args.value_of("ds_root") {
        Some(value) => Ok(String::from(value)),
        None => Ok(env::var("HOME")?)
    }
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
