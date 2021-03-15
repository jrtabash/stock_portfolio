use std::collections::HashSet;
use crate::portfolio::{stock, reports, stocks_reader, stocks_update, algorithms};
use crate::application::arguments::Arguments;

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
        self.read() && self.filter() && self.update() && self.sort() && self.report()
    }

    // --------------------------------------------------------------------------------
    // Private

    fn read(self: &mut Application) -> bool {
        let reader = stocks_reader::StocksReader::new(String::from(self.args.get_stocks_file()));
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

    fn filter(self: &mut Application) -> bool {
        match self.args.get_filter() {
            Some(symbols) => {
                let symbol_set: HashSet<&str> = symbols.split(',').map(|name| name.trim()).collect();
                self.stocks.retain(|stock| symbol_set.contains(stock.symbol.as_str()));
                true
            },
            None => true
        }
    }

    fn update(self: &mut Application) -> bool {
        let count =
            if self.args.get_use_cache() {
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
        match self.args.get_order_by() {
            Some(order_by) => {
                match algorithms::sort_stocks(&mut self.stocks, &order_by, self.args.get_desc()) {
                    Ok(_) => true,
                    Err(error) => {
                        println!("Error: {}", error);
                        false
                    }
                }
            },
            None => true
        }
    }

    fn report(self: &Application) -> bool {
        reports::value_report(&self.stocks, self.args.get_show_groupby());
        true
    }
}
