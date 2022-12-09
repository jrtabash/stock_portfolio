extern crate clap;

use clap::{App, Arg};
use sp_lib::util::{common_args, datetime};

pub struct Arguments {
    calculate: String,
    ds_root: String,
    ds_name: String,
    symbol: String,
    window: usize,
    from: Option<datetime::LocalDate>
}

impl Arguments {
    pub fn new() -> Self {
        #[rustfmt::skip]
        let parsed_args = App::new("Stock Portfolio Stats Tool")
            .version(common_args::app_version())
            .about("Stats tool - describe and calculate")

            // Options
            .arg(common_args::ds_root())
            .arg(common_args::ds_name())
            .arg(common_args::from_date(false, Some("Start from date YYYY-MM-DD")))
            .arg(common_args::symbol(true, None))
            .arg(Arg::with_name("calculate")
                 .short("c")
                 .long("calc")
                 .help("Calculate stats, one of desc, divdesc, sa, vwap, volat, sma, mvwap, roc, pctch, mvolat.\n\
                        desc    : describe symbol history\n\
                        divdesc : describe symbol dividends\n\
                        sa      : calculate symbol adjusted close simple average price\n\
                        vwap    : calculate symbol adjusted close volume weighted average price\n\
                        volat   : calculate symbol adjusted close volatility\n\
                        sma     : calculate symbol adjusted close simple moving average price\n\
                        mvwap   : calculate symbol adjusted close moving volume weighted average price\n\
                        roc     : calculate symbol adjusted close rate of change\n\
                        pctch   : calculate symbol adjusted close percent change relative to from date\n\
                        mvolat  : calculate symbol adjusted close moving volatility")
                 .required(true)
                 .takes_value(true))
            .arg(Arg::with_name("window")
                 .short("w")
                 .long("window")
                 .help("Number of days, required with sma, mvwap, roc and mvolat calculations\n\
                        Required minimum: sma=1, mvwap=1, roc=2, mvolat=1")
                 .takes_value(true))
            .get_matches();

        Arguments {
            calculate: String::from(parsed_args.value_of("calculate").unwrap()),
            ds_root: common_args::parsed_ds_root(&parsed_args).expect("Missing datastore root"),
            ds_name: common_args::parsed_ds_name(&parsed_args),
            symbol: common_args::parsed_symbol(&parsed_args).unwrap(),
            window: match parsed_args.value_of("window") {
                Some(win) => win.parse::<usize>().expect("Invalid calculation window"),
                None => 0
            },
            from: common_args::parsed_from_date(&parsed_args)
        }
    }

    pub fn calculate(&self) -> &String {
        &self.calculate
    }

    pub fn ds_root(&self) -> &String {
        &self.ds_root
    }

    pub fn ds_name(&self) -> &String {
        &self.ds_name
    }

    pub fn symbol(&self) -> &String {
        &self.symbol
    }

    pub fn window(&self) -> usize {
        self.window
    }

    pub fn from(&self) -> Option<datetime::LocalDate> {
        self.from
    }
}
