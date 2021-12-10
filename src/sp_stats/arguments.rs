extern crate clap;

use sp_lib::util::common_args;
use clap::{Arg, App};

pub struct Arguments {
    calculate: String,
    ds_root: String,
    ds_name: String,
    symbol: String,
    window: usize
}

impl Arguments {
    pub fn new() -> Self {
        let parsed_args = App::new("Stock Portfolio Stats Tool")
            .version(common_args::app_version())
            .about("Stats tool - describe and calculate")

            // Options
            .arg(common_args::ds_root())
            .arg(common_args::ds_name())
            .arg(Arg::with_name("calculate")
                 .short("c")
                 .long("calc")
                 .help("Calculate stats, one of desc, divdesc, vwap, mvwap, roc")
                 .required(true)
                 .takes_value(true))
            .arg(Arg::with_name("symbol")
                 .short("y")
                 .long("symbol")
                 .help("Stock symbol")
                 .required(true)
                 .takes_value(true))
            .arg(Arg::with_name("window")
                 .short("w")
                 .long("window")
                 .help("Number of days window, required with mvwap and roc calculations")
                 .takes_value(true))
            .get_matches();

        Arguments {
            calculate: String::from(parsed_args.value_of("calculate").unwrap()),
            ds_root: common_args::parsed_ds_root(&parsed_args).expect("Missing datastore root"),
            ds_name: common_args::parsed_ds_name(&parsed_args),
            symbol: String::from(parsed_args.value_of("symbol").unwrap()),
            window: match parsed_args.value_of("window") {
                Some(win) => win.parse::<usize>().expect("Invalid calculation window"),
                None => 0
            }
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
}
