extern crate clap;

use std::env;
use clap::{Arg, ArgMatches};
use crate::util::datetime;
use crate::util::error::Error;

// --------------------------------------------------------------------------------
// Common Version

pub fn app_version() -> &'static str {
    "0.9.4"
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

pub fn stocks_config_help() -> &'static str {
    "Config file containing datastore root and name, stocks, closed positions and cash in portfolio. \
     Both root and name can be set to \"$default\" which will use home path for root and sp_datastore for name.\n\
     \n\
     The stocks CSV block \"csv{\" should contain stocks in portfolio, with the following columns:\n\
     \tsymbol\n\ttype\n\tdate\n\tquantity\n\tbase_price\n\
     including a header line. Supported type values include cash, etf and index. A stocks CSV file block \"csv_file{\" can \
     be used instead of a stocks CSV block. It should contain the path to a CSV file. The file should contain the CSV stocks data.\n\
     \n\
     The closed positions CSV block \"csv{\" should contain closed positions in portfolio, with the following columns:\n\
     \tsymbol\n\ttype\n\tbase_date\n\texit_date\n\tquantity\n\tbase_price\n\texit_price\n\tbase_fee\n\texit_fee\n\tdividend\n\
     including a header line. Supported type values include cash, etf and index. The closed positions CSV \
     file block \"csv_file{\" can be used instead of a closed positions CSV block. It should contain the path to a CSV file. \
     The file should contain the CSV closed positions data.\n\
     \n\
     Sample config 1:\n\
     \tds_root: $default\n\
     \tds_name: my_datastore\n\
     \tstocks: csv{\n\
     \t  symbol,type,date,quantity,base_price\n\
     \t  AAPL,cash,2020-09-20,100,115.00\n\
     \t}\n\
     \n\
     Sample config 2:\n\
     \tds_root: $default\n\
     \tds_name: my_datastore\n\
     \tcash: 1250.00\n\
     \tstocks: csv_file{\n\
     \t  /path/to/my/stocks.csv\n\
     \t}\n\
     \tclosed_positions: csv_file{\n\
     \t  /path/to/my/closed_positions.csv\n\
     \t}\n"
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
     If type, must be one of 'cash', 'etf', or 'index'.\n\
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

pub fn stocks_config() -> Arg<'static, 'static> {
    Arg::with_name("stocks_config")
        .short("l")
        .long("config")
        .help(stocks_config_help())
        .required(true)
        .takes_value(true)
}

pub fn from_date(required: bool, custom_help: Option<&'static str>) -> Arg<'static, 'static> {
    Arg::with_name("from_date")
        .short("f")
        .long("from")
        .help(custom_help.unwrap_or_else(from_date_help))
        .required(required)
        .takes_value(true)
}

pub fn to_date(required: bool, custom_help: Option<&'static str>) -> Arg<'static, 'static> {
    Arg::with_name("to_date")
        .short("t")
        .long("to")
        .help(custom_help.unwrap_or_else(to_date_help))
        .required(required)
        .takes_value(true)
}

pub fn symbol(required: bool, custom_help: Option<&'static str>) -> Arg<'static, 'static> {
    Arg::with_name("symbol")
        .short("y")
        .long("symbol")
        .help(custom_help.unwrap_or_else(symbol_help))
        .required(required)
        .takes_value(true)
}

pub fn export_file(custom_help: Option<&'static str>) -> Arg<'static, 'static> {
    Arg::with_name("export_file")
        .short("e")
        .long("export")
        .help(custom_help.unwrap_or_else(export_file_help))
        .takes_value(true)
}

// --------------------------------------------------------------------------------
// Common Parsed Matches

pub fn parsed_ds_root(parsed_args: &ArgMatches) -> Result<String, Error> {
    match parsed_args.value_of("ds_root") {
        Some(value) => Ok(String::from(value)),
        None => Ok(env::var("HOME")?)
    }
}

pub fn parsed_ds_name(parsed_args: &ArgMatches) -> String {
    parsed_args.value_of("ds_name").unwrap_or("sp_datastore").to_string()
}

pub fn parsed_stocks_file(parsed_args: &ArgMatches) -> Option<String> {
    parsed_args.value_of("stocks_file").map(String::from)
}

pub fn parsed_stocks_config(parsed_args: &ArgMatches) -> String {
    String::from(parsed_args.value_of("stocks_config").unwrap())
}

pub fn parsed_from_date(parsed_args: &ArgMatches) -> Option<datetime::SPDate> {
    parsed_args.value_of("from_date").map(|date| datetime::parse_date(date).expect("Invalid from date"))
}

pub fn parsed_to_date(parsed_args: &ArgMatches) -> Option<datetime::SPDate> {
    parsed_args.value_of("to_date").map(|date| datetime::parse_date(date).expect("Invalid to date"))
}

pub fn parsed_symbol(parsed_args: &ArgMatches) -> Option<String> {
    parsed_args.value_of("symbol").map(String::from)
}

pub fn parsed_export_file(parsed_args: &ArgMatches) -> Option<String> {
    parsed_args.value_of("export_file").map(String::from)
}
