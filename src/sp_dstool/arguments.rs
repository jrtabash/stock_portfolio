extern crate clap;

use clap::{App, Arg};
use sp_lib::util::common_args;

pub struct Arguments {
    ds_operation: String,
    config_file: String,
    symbol: Option<String>,
    export_file: Option<String>,
    verbose: bool,
    auto_reset: bool
}

impl Arguments {
    pub fn new() -> Self {
        #[rustfmt::skip]
        let parsed_args = App::new("Stock Portfolio Datastore Tool")
            .version(common_args::app_version())
            .about("Datastore tool - create, delete, update, drop, reset, showh, showd, shows, export, check or stat.")

            // Options
            .arg(common_args::stocks_config())
            .arg(common_args::symbol(
                false,
                Some("Stock symbol. Optional with update and check operations. Required with drop, reset, showh, showd, shows, consym and export operations")))
            .arg(common_args::export_file(
                Some("Export symbol history and dividends to csv file. Required with export operation")))
            .arg(Arg::with_name("ds_operation")
                 .short("o")
                 .long("dsop")
                 .help("Datastore tool operation, one of create, delete, update, drop, reset, showh, showd, shows, export, check, stat.\n\
                        create : create empty datastore\n\
                        delete : delete existing datastore\n\
                        update : update history, dividend and split data\n\
                        drop   : drop a symbol\n\
                        reset  : Reset a symbol. Equivalent to drop + update\n\
                        showh  : show history for symbol\n\
                        showd  : show dividends for symbol\n\
                        shows  : show splits for symbol\n\
                        export : export symbol history and dividends\n\
                        consym : check datastore contains symbol\n\
                        check  : check history, dividend and split data\n\
                        stat   : calculate files count and size")
                 .required(true)
                 .takes_value(true))

            // Flags
            .arg(Arg::with_name("verbose")
                 .short("v")
                 .long("verbose")
                 .help("Verbose mode"))
            .arg(Arg::with_name("auto_reset")
                 .short("a")
                 .long("auto-reset")
                 .help("Auto reset stocks on dividend and split updates"))

            .get_matches();

        Arguments {
            ds_operation: String::from(parsed_args.value_of("ds_operation").unwrap()),
            config_file: common_args::parsed_stocks_config(&parsed_args),
            symbol: common_args::parsed_symbol(&parsed_args),
            export_file: common_args::parsed_export_file(&parsed_args),
            verbose: parsed_args.is_present("verbose"),
            auto_reset: parsed_args.is_present("auto_reset")
        }
    }

    #[inline(always)]
    pub fn ds_operation(&self) -> &String {
        &self.ds_operation
    }

    #[inline(always)]
    pub fn config_file(&self) -> &String {
        &self.config_file
    }

    #[inline(always)]
    pub fn symbol(&self) -> Option<&String> {
        self.symbol.as_ref()
    }

    #[inline(always)]
    pub fn export_file(&self) -> Option<&String> {
        self.export_file.as_ref()
    }

    #[inline(always)]
    pub fn is_verbose(&self) -> bool {
        self.verbose
    }

    #[inline(always)]
    pub fn is_auto_reset(&self) -> bool {
        self.auto_reset
    }
}

impl Default for Arguments {
    fn default() -> Self {
        Self::new()
    }
}
