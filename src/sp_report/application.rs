use crate::arguments::Arguments;
use sp_lib::datastore::datastore;
use sp_lib::portfolio::report_type::ReportType;
use sp_lib::portfolio::{algorithms, extra_sort_ftns, report_type, reports, stock, stocks_reader, stocks_update};
use sp_lib::util::common_app;
use std::error::Error;

pub struct Application {
    args: Arguments,
    rtype: ReportType,
    stocks: stock::StockList,
    ds: datastore::DataStore
}

impl common_app::AppTrait for Application {
    fn new() -> Self {
        let args = Arguments::new();
        let ds = datastore::DataStore::new(args.ds_root(), args.ds_name());
        Application {
            args: args,
            rtype: ReportType::Value,
            stocks: stock::StockList::new(),
            ds: ds
        }
    }

    fn run(self: &mut Self) -> common_app::RunResult {
        if !self.ds.exists() {
            return Err(format!("Datastore {} does not exist", self.ds).into());
        }

        if let Some(rtype) = self.args.report_type() {
            self.rtype = report_type::str2reporttype(rtype)?;
        }

        self.read()?;
        self.update()?;
        self.include()?;
        self.exclude()?;
        self.sort()?;
        self.report();
        self.export()?;
        Ok(())
    }
}

impl Application {
    fn read(self: &mut Application) -> Result<(), Box<dyn Error>> {
        let reader = stocks_reader::StocksReader::new(String::from(self.args.stocks_file()));
        self.stocks = reader.read()?;
        Ok(())
    }

    fn include(self: &mut Application) -> Result<(), Box<dyn Error>> {
        if let Some(include_expr) = self.args.include() {
            algorithms::filter_stocks(&mut self.stocks, &include_expr, true)?;
        }
        Ok(())
    }

    fn exclude(self: &mut Application) -> Result<(), Box<dyn Error>> {
        if let Some(exclude_expr) = self.args.exclude() {
            algorithms::filter_stocks(&mut self.stocks, &exclude_expr, false)?;
        }
        Ok(())
    }

    fn update(self: &mut Application) -> Result<(), Box<dyn Error>> {
        let count = stocks_update::update_stocks_from_ds(&mut self.stocks, &self.ds)?;

        if count != self.stocks.len() {
            return Err(format!("update stocks failed; updated={} expected={}", count, self.stocks.len()).into());
        }

        Ok(())
    }

    fn sort(self: &mut Application) -> Result<(), Box<dyn Error>> {
        if let Some(order_by) = self.args.order_by() {
            if let Some(extra_sort) = extra_sort_ftns::extra_sort_ftn(&order_by) {
                extra_sort(&self.ds, &mut self.stocks, self.args.desc());
            } else {
                algorithms::sort_stocks(&mut self.stocks, &order_by, self.args.desc())?;
            }
        }
        Ok(())
    }

    fn report(self: &Application) {
        reports::print_report(
            reports::ReportParams::new(self.rtype, &self.stocks)
                .show_groupby(self.args.show_groupby())
                .with_datastore(&self.ds)
        );
    }

    fn export(self: &Application) -> Result<(), Box<dyn Error>> {
        if let Some(export_file) = self.args.export_file() {
            let report_params = reports::ReportParams::new(self.rtype, &self.stocks).with_datastore(&self.ds);
            reports::export_report(report_params, &export_file)?;
        }
        Ok(())
    }
}
