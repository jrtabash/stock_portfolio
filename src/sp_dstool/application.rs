use std::error::Error;
use std::path::Path;
use std::fs;
use sp_lib::util::datetime;
use sp_lib::portfolio::{stock, stocks_reader};
use sp_lib::yfinance::{types, query};
use sp_lib::datastore::{datastore, history, dividends};
use crate::arguments::Arguments;

const UPDATE: &str = "update";
const DROP: &str = "drop";
const CHECK: &str = "check";
const CREATE: &str = "create";
const DELETE: &str = "delete";

pub struct Application {
    args: Arguments,
    stocks: stock::StockList,
    ds: datastore::DataStore
}

impl Application {
    pub fn new() -> Self {
        let args = Arguments::new();
        let ds = datastore::DataStore::new(args.ds_root(), args.ds_name());
        Application {
            args: args,
            stocks: stock::StockList::new(),
            ds: ds
        }
    }

    pub fn run(self: &mut Self) -> Result<(), Box<dyn Error>> {
        if !self.ds.exists() && self.args.ds_operation() != CREATE {
            return Err(format!("Datastore {} does not exist", self.ds).into());
        }

        if self.args.is_verbose() {
            println!("Running {} operation on datastore {}", self.args.ds_operation(), self.ds);
            println!("stocks: {}", if let Some(file) = self.args.stocks_file() { file } else { "" });
            println!("symbol: {}", if let Some(symbol) = self.args.symbol() { symbol } else { "" });
        }

        self.read_stocks()?;

        match self.args.ds_operation().as_str() {
            UPDATE => self.update()?,
            DROP => self.drop()?,
            CHECK => self.check()?,
            CREATE => self.create()?,
            DELETE => self.delete()?,
            _ => return Err(format!("Invalid ds_operation - '{}'", self.args.ds_operation()).into())
        };

        Ok(())
    }

    // --------------------------------------------------------------------------------
    // Private

    fn read_stocks(self: &mut Self) -> Result<(), Box<dyn Error>> {
        if self.args.is_verbose() { println!("Read stocks file"); }

        if let Some(file) = self.args.stocks_file() {
            let reader = stocks_reader::StocksReader::new(String::from(file));
            self.stocks = reader.read()?;
        }
        Ok(())
    }

    fn update(self: &Self) -> Result<(), Box<dyn Error>> {
        if self.args.is_verbose() { println!("Update stocks"); }

        if self.args.stocks_file().is_none() {
            return Err("Missing stocks file for update operation".into());
        }

        let stck_count = self.stocks.len();
        let mut upd_count: usize = 0;
        let mut err_count: usize = 0;

        for stock in self.stocks.iter() {
            if self.args.is_verbose() { println!("Update {}", stock.symbol); }

            match self.update_stock_data(&stock) {
                Ok(_) => upd_count += 1,
                Err(err) => {
                    eprintln!("{}: {}", stock.symbol, err);
                    err_count += 1;
                }
            };
        }

        println!("Updated {} out of {} stock{}", upd_count, stck_count, if stck_count == 1 { "" } else { "s" });
        if err_count == 0 {
            Ok(())
        }
        else {
            Err(format!("Failed to update {} stock{}", err_count, if err_count == 1 { "" } else { "s" }).into())
        }
    }

    fn update_stock_data(self: &Self, stock: &stock::Stock) -> Result<(), Box<dyn Error>> {
        self.update_stock_history(stock)?;
        self.update_stock_dividends(stock)?;
        Ok(())
    }

    fn update_stock_history(self: &Self, stock: &stock::Stock) -> Result<(), Box<dyn Error>> {
        let hist =
            if self.ds.symbol_exists(history::tag(), &stock.symbol) {
                history::History::ds_select_last(&self.ds, &stock.symbol)?
            } else {
                history::History::new(&stock.symbol)
            };

        if hist.count() > 1 {
            return Err(format!("Found unexpected history query result size {}, expected 0 or 1", hist.count()).into());
        }

        let begin_date =
            if hist.count() == 1 {
                datetime::date_plus_days(&hist.entries()[0].date, 1)
            } else {
                stock.date.clone()
            };

        let today = datetime::today();
        if begin_date <= today {
            let mut query = query::HistoryQuery::new(
                stock.symbol.to_string(),
                begin_date,
                datetime::date_plus_days(&today, 1),
                types::Interval::Daily,
                types::Events::History);

            query.execute()?;
            self.ds.insert_symbol(history::tag(), &stock.symbol, &query.result)?;
        }
        Ok(())
    }

    fn update_stock_dividends(self: &Self, stock: &stock::Stock) -> Result<(), Box<dyn Error>> {
        let div =
            if self.ds.symbol_exists(dividends::tag(), &stock.symbol) {
                dividends::Dividends::ds_select_last(&self.ds, &stock.symbol)?
            } else {
                dividends::Dividends::new(&stock.symbol)
            };

        if div.count() > 1 {
            return Err(format!("Found unexpected dividends query result size {}, expected 0 or 1", div.count()).into());
        }

        let begin_date =
            if div.count() == 1 {
                datetime::date_plus_days(&div.entries()[0].date, 1)
            } else {
                stock.date.clone()
            };

        let today = datetime::today();
        if begin_date <= today {
            let mut query = query::HistoryQuery::new(
                stock.symbol.to_string(),
                begin_date,
                datetime::date_plus_days(&today, 1),
                types::Interval::Daily,
                types::Events::Dividend);

            query.execute()?;
            self.ds.insert_symbol(dividends::tag(), &stock.symbol, &query.result)?;
        }
        Ok(())
    }

    fn drop(self: &Self) -> Result<(), Box<dyn Error>> {
        if self.args.is_verbose() { println!("Drop symbol"); }

        if self.args.symbol().is_none() {
            return Err("Missing symbol for drop operation".into());
        }

        let symbol = self.args.symbol().unwrap();
        let mut count = 0;
        count += self.drop_symbol(history::tag(), &symbol)?;
        count += self.drop_symbol(dividends::tag(), &symbol)?;
        println!("Dropped {} file{} for symbol {}", count, if count == 1 { "" } else { "s" }, symbol);
        Ok(())
    }

    fn drop_symbol(self: &Self, tag: &str, symbol: &str) -> Result<u8, Box<dyn Error>> {
        let mut count: u8 = 0;
        if self.ds.symbol_exists(tag, symbol) {
            self.ds.drop_symbol(tag, &symbol)?;
            count += 1;
        }
        Ok(count)
    }

    fn check(self: &Self) -> Result<(), Box<dyn Error>> {
        if self.args.is_verbose() { println!("Check datastore"); }

        let mut count: usize = 0;

        for entry in fs::read_dir(self.ds.base_path())? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_file() {
                if self.args.is_verbose() { println!("Check entry {}", if let Some(fname) = entry.file_name().to_str() { fname } else { "?" }); }

                if let Err(err) = self.check_entry(&entry_path) {
                    count += 1;
                    eprintln!("{}: {}", if let Some(fname) = entry.file_name().to_str() { fname } else { "?" }, err);
                }
            }
        }

        println!("Check found {} error{}", count, if count == 1 { "" } else { "s" });
        Ok(())
    }

    fn check_entry(self: &Self, entry_path: &Path) -> Result<(), Box<dyn Error>> {
        let content = self.ds.read_file(&entry_path)?;

        let content_ref = match content.find('\n') {
            Some(pos) => &content[0..pos],
            None => return Err(format!("Invalid entry data").into())
        };

        // TODO: Using number of fields is not future proof, fix it!
        let num_of_fields = content_ref.split(',').count();
        if num_of_fields == history::HistoryEntry::number_of_fields() {
            history::History::check_csv(&content)?;
        }
        else if num_of_fields == dividends::DividendEntry::number_of_fields() {
            dividends::Dividends::check_csv(&content)?;
        }
        else {
            return Err(format!("Unknown entry data").into())
        }

        Ok(())
    }

    fn create(self: &Self) -> Result<(), Box<dyn Error>> {
        if self.args.is_verbose() { println!("Create datastore"); }

        self.ds.create()?;

        println!("Datastore {} created", self.ds);
        Ok(())
    }

    fn delete(self: &Self) -> Result<(), Box<dyn Error>> {
        if self.args.is_verbose() { println!("Delete datastore"); }

        self.ds.delete()?;

        println!("Datastore {} deleted", self.ds);
        Ok(())
    }
}
