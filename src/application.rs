use crate::portfolio::{stock, reports, stocks_reader, stocks_update, algorithms};
use crate::arguments::Arguments;

pub struct Application {
    args: Arguments,
    stocks: stock::StockList
}

impl Application {
    pub fn new() -> Application {
        let args = Arguments::new();
        let stocks = stock::StockList::new();
        Application { args, stocks }
    }

    pub fn run(self: &mut Application) -> bool {
        self.read() && self.include() && self.exclude() && self.update() && self.sort() && self.report() && self.export()
    }

    // --------------------------------------------------------------------------------
    // Private

    fn read(self: &mut Application) -> bool {
        let reader = stocks_reader::StocksReader::new(String::from(self.args.stocks_file()));
        match reader.read() {
            Ok(stocks) => {
                self.stocks = stocks;
                true
            },
            Err(error) => {
                println!("{}", error);
                false
            }
        }
    }

    fn include(self: &mut Application) -> bool {
        if let Some(include_expr) = self.args.include() {
            algorithms::filter_stocks(&mut self.stocks, &include_expr, true);
        }
        true
    }

    fn exclude(self: &mut Application) -> bool {
        if let Some(exclude_expr) = self.args.exclude() {
            algorithms::filter_stocks(&mut self.stocks, &exclude_expr, false);
        }
        true
    }

    fn update(self: &mut Application) -> bool {
        let count =
            if self.args.use_cache() {
                stocks_update::update_stocks_with_cache(&mut self.stocks)
            } else {
                stocks_update::update_stocks(&mut self.stocks)
            };

        let success = count == self.stocks.len();
        if !success {
            println!("update_stocks failed; updated={} expected={}", count, self.stocks.len());
        }
        success
    }

    fn sort(self: &mut Application) -> bool {
        if let Some(order_by) = self.args.order_by() {
            if let Err(error) = algorithms::sort_stocks(&mut self.stocks, &order_by, self.args.desc()) {
                println!("Error: {}", error);
                return false
            }
        }
        true
    }

    fn report(self: &Application) -> bool {
        reports::value_report(&self.stocks, self.args.show_groupby());
        true
    }

    fn export(self: &Application) -> bool {
        if let Some(export_file) = self.args.export_file() {
            if let Err(error) = reports::value_export(&self.stocks, &export_file) {
                println!("Error: {}", error);
                return false
            }
        }
        true
    }
}
