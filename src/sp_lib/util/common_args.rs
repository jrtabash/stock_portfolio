extern crate clap;

use std::env;
use std::error::Error;
use clap::{Arg, ArgMatches};
use crate::util::datetime;

// --------------------------------------------------------------------------------
// Common Version

pub fn app_version() -> &'static str {
    "0.7.2"
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
     Supported type values include stock, etf and index"
}

pub fn from_date_help() -> &'static str {
    "Start date YYYY-MM-DD"
}

pub fn to_date_help() -> &'static str {
    "Stop date YYYY-MM-DD"
}

pub fn symbol_help() -> &'static str {
    "Stock symbol"
}

pub fn export_file_help() -> &'static str {
    "Export file"
}

pub fn filter_help() -> &'static str {
    "Filter stocks by type, symbols or expression;\n\
     If type, must be one of 'stock', 'etf', or 'index'.\n\
     If symbols, must be a comma separated list of symbol names.\n\
     If expression, must follow the format '<field> <op> <value>', where:\n\
     <field> : one of days, price, net, pct, div, size, value\n\
     <op>    : one of =, !=, <, >, <=, >=\n\
     Example : 'days > 365'"
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

pub fn from_date(required: bool, custom_help: Option<&'static str>) -> Arg<'static, 'static> {
    Arg::with_name("from_date")
        .short("f")
        .long("from")
        .help(custom_help.unwrap_or(from_date_help()))
        .required(required)
        .takes_value(true)
}

pub fn to_date(required: bool, custom_help: Option<&'static str>) -> Arg<'static, 'static> {
    Arg::with_name("to_date")
        .short("t")
        .long("to")
        .help(custom_help.unwrap_or(to_date_help()))
        .required(required)
        .takes_value(true)
}

pub fn symbol(required: bool, custom_help: Option<&'static str>) -> Arg<'static, 'static> {
    Arg::with_name("symbol")
        .short("y")
        .long("symbol")
        .help(custom_help.unwrap_or(symbol_help()))
        .required(required)
        .takes_value(true)
}

pub fn export_file(custom_help: Option<&'static str>) -> Arg<'static, 'static> {
    Arg::with_name("export_file")
        .short("e")
        .long("export")
        .help(custom_help.unwrap_or(export_file_help()))
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

pub fn parsed_from_date(parsed_args: &ArgMatches) -> Option<datetime::LocalDate> {
    match parsed_args.value_of("from_date") {
        Some(date) => Some(datetime::parse_date(date).expect("Invalid from date")),
        None => None
    }
}

pub fn parsed_to_date(parsed_args: &ArgMatches) -> Option<datetime::LocalDate> {
    match parsed_args.value_of("to_date") {
        Some(date) => Some(datetime::parse_date(date).expect("Invalid to date")),
        None => None
    }
}

pub fn parsed_symbol(parsed_args: &ArgMatches) -> Option<String> {
    match parsed_args.value_of("symbol") {
        Some(value) => Some(String::from(value)),
        None => None
    }
}

pub fn parsed_export_file(parsed_args: &ArgMatches) -> Option<String> {
    match parsed_args.value_of("export_file") {
        Some(value) => Some(String::from(value)),
        None => None
    }
}
