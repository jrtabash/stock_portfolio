extern crate clap;

use sp_lib::util::common_args;
use clap::{Arg, App};

pub struct Arguments {
    stocks_file: String,
    order_by: Option<String>,
    include: Option<String>,
    exclude: Option<String>,
    export_file: Option<String>,
    ds_root: String,
    ds_name: String,
    show_groupby: bool,
    desc: bool
}

impl Arguments {
    pub fn new() -> Arguments {
        let parsed_args = App::new("Stock Portfolio Report")
            .version(common_args::app_version())
            .about("Get latest close prices and report the gains and losses of stocks in portfolio.")

            // Options
            .arg(common_args::stocks_file(true))
            .arg(Arg::with_name("order_by")
                 .short("o")
                 .long("orderby")
                 .help("Order stocks by one of symbol, type, date, days, price, net, pct, div, size or value")
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
            .arg(common_args::ds_root("Datastore root path, use to update portfolio latest prices"))
            .arg(common_args::ds_name("Datastore name, used with datastore root (default: sp_datastore)"))

            // Flags
            .arg(Arg::with_name("show_groupby")
                 .short("g")
                 .long("show-groupby")
                 .help("Show quantities and current notional values grouped by symbol"))
            .arg(Arg::with_name("desc")
                 .short("d")
                 .long("desc")
                 .help("Used with order by option to sort in descending order"))
            .get_matches();

        let stocks_file = common_args::parsed_stocks_file(&parsed_args).unwrap();
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
        let ds_root = common_args::parsed_ds_root(&parsed_args);
        let ds_name = common_args::parsed_ds_name(&parsed_args);
        let show_groupby = parsed_args.is_present("show_groupby");
        let desc = parsed_args.is_present("desc");

        Arguments {
            stocks_file,
            order_by,
            include,
            exclude,
            export_file,
            ds_root,
            ds_name,
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

    pub fn ds_root(self: &Self) -> &String {
        &self.ds_root
    }

    pub fn ds_name(self: &Self) -> &String {
        &self.ds_name
    }

    pub fn show_groupby(self: &Arguments) -> bool {
        self.show_groupby
    }

    pub fn desc(self: &Arguments) -> bool {
        self.desc
    }
}
