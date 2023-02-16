extern crate clap;

use clap::{App, Arg};
use sp_lib::util::{common_args, datetime};

pub struct Arguments {
    calculate: String,
    config_file: String,
    symbol: String,
    field: String,
    window: usize,
    from: Option<datetime::SPDate>
}

impl Arguments {
    pub fn new() -> Self {
        #[rustfmt::skip]
        let parsed_args = App::new("Stock Portfolio Stats Tool")
            .version(common_args::app_version())
            .about("Stats tool - describe and calculate")

            // Options
            .arg(common_args::stocks_config())
            .arg(common_args::from_date(false, Some("Start from date YYYY-MM-DD")))
            .arg(common_args::symbol(true, None))
            .arg(Arg::with_name("calculate")
                 .short("c")
                 .long("calc")
                 .help("Calculate stats, one of desc, divdesc, sa, vwap, volat, sma, mvwap, roc, pctch, mvolat, rsi.\n\
                        desc    : describe history\n\
                        divdesc : describe dividends\n\
                        sa      : calculate simple average price\n\
                        vwap    : calculate volume weighted average price\n\
                        volat   : calculate volatility\n\
                        sma     : calculate simple moving average price\n\
                        mvwap   : calculate moving volume weighted average price\n\
                        roc     : calculate rate of change\n\
                        pctch   : calculate percent change relative to from date\n\
                        mvolat  : calculate moving volatility\n\
                        rsi     : Calculate Relative Strength Index")
                 .required(true)
                 .takes_value(true))
            .arg(Arg::with_name("window")
                 .short("w")
                 .long("window")
                 .help("Number of days, required with sma, mvwap, roc, mvolat and rsi calculations\n\
                        Required minimum: sma=1, mvwap=1, roc=2, mvolat=1, rsi=2")
                 .takes_value(true))
            .arg(Arg::with_name("field")
                 .short("i")
                 .long("field")
                 .help("Symbol history field to use in calculation.\n\
                        One of open, high, low, close, adj_close. Default adj_close.\n\
                        Applies to sa, vwap, volat, sma, mvwap, roc, pctch and mvolat")
                 .takes_value(true))
            .get_matches();

        Arguments {
            calculate: String::from(parsed_args.value_of("calculate").unwrap()),
            config_file: common_args::parsed_stocks_config(&parsed_args),
            symbol: common_args::parsed_symbol(&parsed_args).unwrap(),
            field: String::from(parsed_args.value_of("field").unwrap_or("adj_close")),
            window: match parsed_args.value_of("window") {
                Some(win) => win.parse::<usize>().expect("Invalid calculation window"),
                None => 0
            },
            from: common_args::parsed_from_date(&parsed_args)
        }
    }

    #[inline(always)]
    pub fn calculate(&self) -> &String {
        &self.calculate
    }

    #[inline(always)]
    pub fn config_file(&self) -> &String {
        &self.config_file
    }

    #[inline(always)]
    pub fn symbol(&self) -> &String {
        &self.symbol
    }

    #[inline(always)]
    pub fn field(&self) -> &String {
        &self.field
    }

    #[inline(always)]
    pub fn window(&self) -> usize {
        self.window
    }

    #[inline(always)]
    pub fn from(&self) -> Option<datetime::SPDate> {
        self.from
    }
}

impl Default for Arguments {
    fn default() -> Self {
        Self::new()
    }
}
