use std::error::Error;
use crate::portfolio::{stock, reports, stocks_reader, stocks_update, algorithms};
use crate::arguments::Arguments;

pub struct Application {
    args: Arguments,
    stocks: stock::StockList
}

impl Application {
    pub fn new() -> Application {
        Application {
            args: Arguments::new(),
            stocks: stock::StockList::new()
        }
    }

    pub fn run(self: &mut Application) -> Result<(), Box<dyn Error>> {
        self.read()?;
        self.include();
        self.exclude();
        self.update()?;
        self.sort()?;
        self.report();
        self.export()?;
        Ok(())
    }

    // --------------------------------------------------------------------------------
    // Private

    fn read(self: &mut Application) -> Result<(), Box<dyn Error>> {
        let reader = stocks_reader::StocksReader::new(String::from(self.args.stocks_file()));
        self.stocks = reader.read()?;
        Ok(())
    }

    fn include(self: &mut Application) {
        if let Some(include_expr) = self.args.include() {
            algorithms::filter_stocks(&mut self.stocks, &include_expr, true);
        }
    }

    fn exclude(self: &mut Application) {
        if let Some(exclude_expr) = self.args.exclude() {
            algorithms::filter_stocks(&mut self.stocks, &exclude_expr, false);
        }
    }

    fn update(self: &mut Application) -> Result<(), Box<dyn Error>> {
        let count =
            if self.args.use_cache() {
                stocks_update::update_stocks_with_cache(&mut self.stocks)?
            } else {
                stocks_update::update_stocks(&mut self.stocks)?
            };

        if count != self.stocks.len() {
            return Err(format!("update_stocks failed; updated={} expected={}", count, self.stocks.len()).into())
        }

        Ok(())
    }

    fn sort(self: &mut Application) -> Result<(), Box<dyn Error>> {
        if let Some(order_by) = self.args.order_by() {
            algorithms::sort_stocks(&mut self.stocks, &order_by, self.args.desc())?;
        }
        Ok(())
    }

    fn report(self: &Application) {
        reports::value_report(&self.stocks, self.args.show_groupby());
    }

    fn export(self: &Application) -> Result<(), Box<dyn Error>> {
        if let Some(export_file) = self.args.export_file() {
            reports::value_export(&self.stocks, &export_file)?;
        }
        Ok(())
    }
}
