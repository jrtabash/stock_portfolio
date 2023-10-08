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
    desc: bool,
    match_symbols: bool
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
                        symbol : stock symbol       | type    : stock type\n\
                        date   : base date          | days    : days held\n\
                        price  : latest price       | size    : quantity\n\
                        net    : net price          | pct     : percent change\n\
                        value  : notional value     | div     : cumulative dividend\n\
                        volat  : overall volatility | volat22 : 22 day volatility\n\
                        prevpr : previous day price | volume  : day volume\n\
                        change : day change         | pctchg  : day percent change\n\
                        valchg : day value change   | low     : day low price\n\
                        high   : day high price")
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
            .arg(Arg::with_name("match_symbols")
                 .short("m")
                 .long("match-symbols")
                 .help("Match closed positions to configured stock symbols post filtering and ordering"))
            .get_matches();

        let config_file = common_args::parsed_stocks_config(&parsed_args);
        let report_type = parsed_args.value_of("report_type").map(String::from);
        let order_by = parsed_args.value_of("order_by").map(String::from);
        let include = parsed_args.value_of("include").map(String::from);
        let exclude = parsed_args.value_of("exclude").map(String::from);
        let export_file = common_args::parsed_export_file(&parsed_args);
        let show_groupby = parsed_args.is_present("show_groupby");
        let desc = parsed_args.is_present("desc");
        let match_symbols = parsed_args.is_present("match_symbols");

        Arguments {
            config_file,
            report_type,
            order_by,
            include,
            exclude,
            export_file,
            show_groupby,
            desc,
            match_symbols
        }
    }

    #[inline(always)]
    pub fn config_file(&self) -> &String {
        &self.config_file
    }

    #[inline(always)]
    pub fn report_type(&self) -> Option<&String> {
        self.report_type.as_ref()
    }

    #[inline(always)]
    pub fn order_by(&self) -> Option<&String> {
        self.order_by.as_ref()
    }

    #[inline(always)]
    pub fn include(&self) -> Option<&String> {
        self.include.as_ref()
    }

    #[inline(always)]
    pub fn exclude(&self) -> Option<&String> {
        self.exclude.as_ref()
    }

    #[inline(always)]
    pub fn export_file(&self) -> Option<&String> {
        self.export_file.as_ref()
    }

    #[inline(always)]
    pub fn show_groupby(&self) -> bool {
        self.show_groupby
    }

    #[inline(always)]
    pub fn desc(&self) -> bool {
        self.desc
    }

    #[inline(always)]
    pub fn match_symbols(&self) -> bool {
        self.match_symbols
    }
}

impl Default for Arguments {
    fn default() -> Self {
        Self::new()
    }
}

