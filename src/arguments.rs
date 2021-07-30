extern crate clap;

use clap::{Arg, App};

pub struct Arguments {
    stocks_file: String,
    order_by: Option<String>,
    include: Option<String>,
    exclude: Option<String>,
    export_file: Option<String>,
    use_cache: bool,
    show_groupby: bool,
    desc: bool
}

impl Arguments {
    pub fn new() -> Arguments {
        let parsed_args = App::new("Stock Portfolio Tool")
            .version("0.1.1")
            .about("Get latest close prices and report the gains and losses of stocks in portfolio.")

            // Options
            .arg(Arg::with_name("stocks_file")
                 .short("s")
                 .long("stocks")
                 .help("CSV file containing stocks in portfolio, formatted as 'symbol,type,date,quantity,base_price' including a header line. \
                        Supported type values include stock and etf")
                 .required(true)
                 .takes_value(true))
            .arg(Arg::with_name("order_by")
                 .short("o")
                 .long("orderby")
                 .help("Order stocks by one of symbol, type, date, days, price, net, pct, size or value")
                 .takes_value(true))
            .arg(Arg::with_name("include")
                 .short("i")
                 .long("include")
                 .help("Include stocks by type or symbols; one of stock, etf or a comma separated list of symbols")
                 .takes_value(true))
            .arg(Arg::with_name("exclude")
                 .short("x")
                 .long("exclude")
                 .help("Exclude stocks by type or symbols; one of stock, etf or a comma separated list of symbols")
                 .takes_value(true))
            .arg(Arg::with_name("export_file")
                 .short("e")
                 .long("export")
                 .help("Export gains and losses table to a csv file")
                 .takes_value(true))

            // Flags
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
        let include = match parsed_args.value_of("include") {
            Some(value) => Some(String::from(value)),
            None => None
        };
        let exclude = match parsed_args.value_of("exclude") {
            Some(value) => Some(String::from(value)),
            None => None
        };
        let export_file = match parsed_args.value_of("export_file") {
            Some(value) => Some(String::from(value)),
            None => None
        };
        let use_cache = parsed_args.is_present("use_cache");
        let show_groupby = parsed_args.is_present("show_groupby");
        let desc = parsed_args.is_present("desc");

        Arguments {
            stocks_file,
            order_by,
            include,
            exclude,
            export_file,
            use_cache,
            show_groupby,
            desc
        }
    }

    pub fn stocks_file(self: &Arguments) -> &String {
        &self.stocks_file
    }

    pub fn order_by(self: &Arguments) -> Option<&String> {
        self.order_by.as_ref()
    }

    pub fn include(self: &Arguments) -> Option<&String> {
        self.include.as_ref()
    }

    pub fn exclude(self: &Arguments) -> Option<&String> {
        self.exclude.as_ref()
    }

    pub fn export_file(self: &Arguments) -> Option<&String> {
        self.export_file.as_ref()
    }

    pub fn use_cache(self: &Arguments) -> bool {
        self.use_cache
    }

    pub fn show_groupby(self: &Arguments) -> bool {
        self.show_groupby
    }

    pub fn desc(self: &Arguments) -> bool {
        self.desc
    }
}
