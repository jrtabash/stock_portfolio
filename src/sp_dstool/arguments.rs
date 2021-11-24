extern crate clap;

use clap::{Arg, App};

pub struct Arguments {
    ds_operation: String,
    ds_root: String,
    ds_name: String,
    stocks_file: Option<String>,
    symbol: Option<String>,
    export_file: Option<String>,
    verbose: bool
}

impl Arguments {
    pub fn new() -> Self {
        let parsed_args = App::new("Stock Portfolio Datastore Tool")
            .version("0.1.0")
            .about("Datastore tool - create, delete, update, drop, export, check or stat.")

            // Options
            .arg(Arg::with_name("ds_root")
                 .short("r")
                 .long("root")
                 .help("Datastore root path")
                 .required(true)
                 .takes_value(true))
            .arg(Arg::with_name("ds_name")
                 .short("n")
                 .long("name")
                 .help("Datastore name (default: sp_datastore)")
                 .takes_value(true))
            .arg(Arg::with_name("ds_operation")
                 .short("o")
                 .long("dsop")
                 .help("Datastore tool operation, one of create, delete, update, drop, export, check, stat")
                 .required(true)
                 .takes_value(true))
            .arg(Arg::with_name("stocks_file")
                 .short("s")
                 .long("stocks")
                 .help("CSV file containing stocks in portfolio, refer to sp_report --help for more information. \
                        File is required with update operation")
                 .takes_value(true))
            .arg(Arg::with_name("symbol")
                 .short("y")
                 .long("symbol")
                 .help("Stock symbol. Optional with update and check operations. Required with drop and export symbol operation")
                 .takes_value(true))
            .arg(Arg::with_name("export_file")
                 .short("e")
                 .long("export")
                 .help("Export symbol history and dividends to csv file. Required with export operation")
                 .takes_value(true))

            // Flags
            .arg(Arg::with_name("verbose")
                 .short("v")
                 .long("verbose")
                 .help("Verbose mode"))

            .get_matches();

        Arguments {
            ds_operation: String::from(parsed_args.value_of("ds_operation").unwrap()),
            ds_root: String::from(parsed_args.value_of("ds_root").unwrap()),
            ds_name: String::from(
                match parsed_args.value_of("ds_name") {
                    Some(value) => value,
                    None => "sp_datastore"
                }),
            stocks_file: match parsed_args.value_of("stocks_file") {
                Some(value) => Some(String::from(value)),
                None => None
            },
            symbol: match parsed_args.value_of("symbol") {
                Some(value) => Some(String::from(value)),
                None => None
            },
            export_file: match parsed_args.value_of("export_file") {
                Some(value) => Some(String::from(value)),
                None => None
            },
            verbose: parsed_args.is_present("verbose")
        }
    }

    pub fn ds_operation(self: &Self) -> &String {
        &self.ds_operation
    }

    pub fn ds_root(self: &Self) -> &String {
        &self.ds_root
    }

    pub fn ds_name(self: &Self) -> &String {
        &self.ds_name
    }

    pub fn stocks_file(self: &Self) -> Option<&String> {
        self.stocks_file.as_ref()
    }

    pub fn symbol(self: &Self) -> Option<&String> {
        self.symbol.as_ref()
    }

    pub fn export_file(self: &Arguments) -> Option<&String> {
        self.export_file.as_ref()
    }

    pub fn is_verbose(self: &Self) -> bool {
        self.verbose
    }
}
