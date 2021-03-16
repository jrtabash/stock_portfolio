extern crate clap;

use clap::{Arg, App};

pub struct Arguments {
    stocks_file: String,
    order_by: Option<String>,
    filter: Option<String>,
    use_cache: bool,
    show_groupby: bool,
    desc: bool
}

impl Arguments {
    pub fn new() -> Arguments {
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

        let stocks_file = String::from(parsed_args.value_of("stocks_file").unwrap());
        let order_by = match parsed_args.value_of("order_by") {
            Some(value) => Some(String::from(value)),
            None => None
        };
        let filter = match parsed_args.value_of("filter") {
            Some(value) => Some(String::from(value)),
            None => None
        };
        let use_cache = parsed_args.is_present("use_cache");
        let show_groupby = parsed_args.is_present("show_groupby");
        let desc = parsed_args.is_present("desc");

        Arguments { stocks_file, order_by, filter, use_cache, show_groupby, desc }
    }

    pub fn get_stocks_file(self: &Arguments) -> &String {
        &self.stocks_file
    }

    pub fn get_order_by(self: &Arguments) -> Option<&String> {
        match &self.order_by {
            Some(value) => Some(&value),
            None => None
        }
    }

    pub fn get_filter(self: &Arguments) -> Option<&String> {
        match &self.filter {
            Some(value) => Some(&value),
            None => None
        }
    }

    pub fn get_use_cache(self: &Arguments) -> bool {
        self.use_cache
    }

    pub fn get_show_groupby(self: &Arguments) -> bool {
        self.show_groupby
    }

    pub fn get_desc(self: &Arguments) -> bool {
        self.desc
    }
}