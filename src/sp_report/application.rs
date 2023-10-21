use crate::arguments::Arguments;
use sp_lib::datastore::datastore;
use sp_lib::portfolio::report_type::ReportType;
use sp_lib::portfolio::{algorithms, extra_sort_ftns, report_type, reports, stocks_config, stocks_update};
use sp_lib::util::common_app;
use sp_lib::util::error::Error;

pub struct Application {
    args: Arguments,
    rtype: ReportType,
    config: stocks_config::StocksConfig,
    ds: datastore::DataStore
}

impl common_app::AppTrait for Application {
    fn new() -> Self {
        let args = Arguments::new();
        let config = stocks_config::StocksConfig::from_file(args.config_file()).expect("Missing config file");
        let ds = datastore::DataStore::new(config.ds_root(), config.ds_name());
        Application {
            args,
            rtype: ReportType::Value,
            config,
            ds
        }
    }

    fn run(&mut self) -> common_app::RunResult {
        if !self.ds.exists() {
            return Err(format!("Datastore {} does not exist", self.ds).into());
        }

        if let Some(rtype) = self.args.report_type() {
            self.rtype = report_type::str2reporttype(rtype)?;
        }

        self.update()?;
        self.include()?;
        self.exclude()?;
        self.sort()?;
        self.match_positions_to_stocks()?;
        self.report();
        self.export()?;
        Ok(())
    }
}

impl Application {
    fn include(self: &mut Application) -> Result<(), Error> {
        if let Some(include_expr) = self.args.include() {
            algorithms::filter_stocks(self.config.stocks_mut(), include_expr, true)?;
        }
        Ok(())
    }

    fn exclude(self: &mut Application) -> Result<(), Error> {
        if let Some(exclude_expr) = self.args.exclude() {
            algorithms::filter_stocks(self.config.stocks_mut(), exclude_expr, false)?;
        }
        Ok(())
    }

    fn update(self: &mut Application) -> Result<(), Error> {
        let count = stocks_update::update_stocks_from_ds(self.config.stocks_mut(), &self.ds)?;

        if count != self.config.stocks().len() {
            return Err(format!("update stocks failed; updated={} expected={}", count, self.config.stocks().len()).into());
        }

        Ok(())
    }

    fn sort(self: &mut Application) -> Result<(), Error> {
        if let Some(order_by) = self.args.order_by() {
            if let Some(extra_sort) = extra_sort_ftns::extra_sort_ftn(order_by) {
                extra_sort(&self.ds, self.config.stocks_mut(), self.args.desc());
            } else {
                algorithms::sort_stocks(self.config.stocks_mut(), order_by, self.args.desc())?;
            }
        }
        Ok(())
    }

    fn match_positions_to_stocks(self: &mut Application) -> Result<(), Error> {
        if self.args.match_symbols() {
            let mut symbols: Vec<String> = self.config.stocks().iter().map(|s| s.symbol.clone()).collect();
            symbols.dedup();
            algorithms::match_list_to_symbols(self.config.closed_positions_mut(), &symbols)?;
        }
        Ok(())
    }

    fn report(self: &Application) {
        reports::print_report(
            reports::ReportParams::new(self.rtype, self.config.stocks(), self.config.closed_positions())
                .show_groupby(self.args.show_groupby())
                .with_datastore(&self.ds)
        );
    }

    fn export(self: &Application) -> Result<(), Error> {
        if let Some(export_file) = self.args.export_file() {
            let report_params = reports::ReportParams::new(self.rtype, self.config.stocks(), self.config.closed_positions())
                .with_datastore(&self.ds);
            reports::export_report(report_params, export_file)?;
        }
        Ok(())
    }
}
