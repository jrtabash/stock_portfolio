extern crate clap;

use clap::{Arg, App};

pub struct Arguments {
    ds_operation: String,
    ds_root: String,
    ds_name: String,
    stocks_file: Option<String>
}

impl Arguments {
    pub fn new() -> Self {
        let parsed_args = App::new("Stock Portfolio Datastore Tool")
            .version("0.1.0")
            .about("Datastore tool - create, delete, update, or check.")

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
                 .help("Datastore tool operation, one of create, delete, update, check")
                 .required(true)
                 .takes_value(true))
            .arg(Arg::with_name("stocks_file")
                 .short("s")
                 .long("stocks")
                 .help("CSV file containing stocks in portfolio, refer to sp_report --help for more information. \
                        File is required with update operation.")
                 .takes_value(true))
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
            }
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
}