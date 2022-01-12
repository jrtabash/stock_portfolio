use std::error::Error;
use sp_lib::portfolio::{stock, reports, stocks_reader, stocks_update, algorithms};
use sp_lib::datastore::datastore;
use crate::arguments::Arguments;

pub struct Application {
    args: Arguments,
    stocks: stock::StockList,
    ds: datastore::DataStore
}

impl Application {
    pub fn new() -> Application {
        let args = Arguments::new();
        let ds = datastore::DataStore::new(args.ds_root(), args.ds_name());
        Application {
            args: args,
            stocks: stock::StockList::new(),
            ds: ds
        }
    }

    pub fn run(self: &mut Application) -> Result<(), Box<dyn Error>> {
        if !self.ds.exists() {
            return Err(format!("Datastore {} does not exist", self.ds).into());
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

    // --------------------------------------------------------------------------------
    // Private

    fn is_field_expression(expr: &str) -> bool {
        let type_or_symbols =
            expr == "stock" ||           // Is stock type?
            expr == "etf" ||             // Is etf type?
            expr.contains(',') ||        // Is list of symbols?
            !expr.trim().contains(' ');  // Is a single symbol?
        !type_or_symbols
    }

    fn read(self: &mut Application) -> Result<(), Box<dyn Error>> {
        let reader = stocks_reader::StocksReader::new(String::from(self.args.stocks_file()));
        self.stocks = reader.read()?;
        Ok(())
    }

    fn include(self: &mut Application) -> Result<(), Box<dyn Error>> {
        if let Some(include_expr) = self.args.include() {
            if Self::is_field_expression(&include_expr) {
                algorithms::filter_stocks_by_expr(&mut self.stocks, &include_expr, true)?;
            }
            else {
                algorithms::filter_stocks(&mut self.stocks, &include_expr, true);
            }
        }
        Ok(())
    }

    fn exclude(self: &mut Application) -> Result<(), Box<dyn Error>> {
        if let Some(exclude_expr) = self.args.exclude() {
            if Self::is_field_expression(&exclude_expr) {
                algorithms::filter_stocks_by_expr(&mut self.stocks, &exclude_expr, false)?;
            }
            else {
                algorithms::filter_stocks(&mut self.stocks, &exclude_expr, false);
            }
        }
        Ok(())
    }

    fn update(self: &mut Application) -> Result<(), Box<dyn Error>> {
        let count = stocks_update::update_stocks_from_ds(&mut self.stocks, &self.ds)?;

        if count != self.stocks.len() {
            return Err(format!("update stocks failed; updated={} expected={}", count, self.stocks.len()).into())
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
