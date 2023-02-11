extern crate clap;

use clap::{App, Arg};
use sp_lib::util::{common_args, datetime};

pub struct Arguments {
    events: String,
    interval: String,
    symbol: String,
    from: Option<datetime::SPDate>,
    to: Option<datetime::SPDate>
}

impl Arguments {
    pub fn new() -> Self {
        #[rustfmt::skip]
        let parsed_args = App::new("YFinance History Tool")
            .version(common_args::app_version())
            .about("Yhist tool - Query yfinance history")

            // Options
            .arg(common_args::symbol(true, None))
            .arg(common_args::from_date(false, Some("Start date YYYY-MM-DD (default: today - 7days)")))
            .arg(common_args::to_date(false, Some("Stop date YYYY-MM-DD (default: today)")))
            .arg(Arg::with_name("events")
                 .short("e")
                 .long("events")
                 .help("Events to query, one of history, dividend, split")
                 .required(true)
                 .takes_value(true))
            .arg(Arg::with_name("interval")
                 .short("i")
                 .long("interval")
                 .help("Interval to query, one of day, week, month")
                 .required(true)
                 .takes_value(true))
            .get_matches();

        Arguments {
            symbol: common_args::parsed_symbol(&parsed_args).unwrap(),
            from: common_args::parsed_from_date(&parsed_args),
            to: common_args::parsed_to_date(&parsed_args),
            events: String::from(parsed_args.value_of("events").expect("Missing events")),
            interval: String::from(parsed_args.value_of("interval").expect("Missing interval"))
        }
    }

    #[inline(always)]
    pub fn symbol(&self) -> &String {
        &self.symbol
    }

    #[inline(always)]
    pub fn from(&self) -> Option<datetime::SPDate> {
        self.from
    }

    #[inline(always)]
    pub fn to(&self) -> Option<datetime::SPDate> {
        self.to
    }

    #[inline(always)]
    pub fn events(&self) -> &String {
        &self.events
    }

    #[inline(always)]
    pub fn interval(&self) -> &String {
        &self.interval
    }
}
