extern crate clap;

use clap::{App, Arg};
use sp_lib::util::common_args;

pub struct Arguments {
    config_file: String,
    report_type: Option<String>,
    order_by: Option<String>,
    include: Option<String>,
    exclude: Option<String>,
    export_file: Option<String>,
    show_groupby: bool,
    desc: bool
}

impl Arguments {
    pub fn new() -> Arguments {
        #[rustfmt::skip]
        let parsed_args = App::new("Stock Portfolio Report")
            .version(common_args::app_version())
            .about("Get latest close prices and generate portfolio report.\n\
                    Supported reports include gains & losses, top/bottom performers, volatility, and day change.")

            // Options
            .arg(common_args::stocks_config())
            .arg(common_args::export_file(Some("Export gains and losses table to a csv file")))
            .arg(Arg::with_name("report_type")
                 .short("p")
                 .long("type")
                 .help("Report type, one of value, top, volat (default: value)\n\
                        value : stocks value (gains & losses)\n\
                        top   : Top/Bottom performing stocks\n\
                        volat : Stocks volatility\n\
                        daych : Stocks day change")
                 .takes_value(true))
            .arg(Arg::with_name("order_by")
                 .short("o")
                 .long("orderby")
                 .help("Order stocks by one of:\n\
                        symbol : stock symbol        | type    : stock type\n\
                        date   : base date           | days    : days held\n\
                        price  : latest price        | size    : quantity\n\
                        net    : net price           | pct     : percent change\n\
                        value  : notional value      | div     : cumulative dividend\n\
                        volat  : orderall volatility | volat22 : 22 day volatility\n\
                        prevpr : previous day price  | volume  : day volume\n\
                        change : day change          | pctchg  : day percent change\n\
                        low    : day low price       | high    : day high price")
                 .takes_value(true))
            .arg(Arg::with_name("include")
                 .short("i")
                 .long("include")
                 .help(common_args::filter_help())
                 .takes_value(true))
            .arg(Arg::with_name("exclude")
                 .short("x")
                 .long("exclude")
                 .help(common_args::filter_help())
                 .takes_value(true))

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

        let config_file = common_args::parsed_stocks_config(&parsed_args);
        let report_type = match parsed_args.value_of("report_type") {
            Some(value) => Some(String::from(value)),
            None => None
        };
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
        let export_file = common_args::parsed_export_file(&parsed_args);
        let show_groupby = parsed_args.is_present("show_groupby");
        let desc = parsed_args.is_present("desc");

        Arguments {
            config_file,
            report_type,
            order_by,
            include,
            exclude,
            export_file,
            show_groupby,
            desc
        }
    }

    pub fn config_file(self: &Arguments) -> &String {
        &self.config_file
    }

    pub fn report_type(self: &Arguments) -> Option<&String> {
        self.report_type.as_ref()
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

    pub fn show_groupby(self: &Arguments) -> bool {
        self.show_groupby
    }

    pub fn desc(self: &Arguments) -> bool {
        self.desc
    }
}
