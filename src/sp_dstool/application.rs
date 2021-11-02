use std::error::Error;
use std::path::Path;
use std::fs;
use sp_lib::portfolio::{stock, stocks_reader};
use sp_lib::datastore::{datastore, history, dividends};
use crate::arguments::Arguments;

pub struct Application {
    args: Arguments,
    stocks: stock::StockList
}

impl Application {
    pub fn new() -> Self {
        Application {
            args: Arguments::new(),
            stocks: stock::StockList::new()
        }
    }

    pub fn run(self: &mut Self) -> Result<(), Box<dyn Error>> {
        self.read_stocks()?;

        match self.args.ds_operation().as_str() {
            "update" => self.update()?,
            "check" => self.check()?,
            "create" => self.create()?,
            "delete" => self.delete()?,
            _ => return Err(format!("Invalid ds_operation - '{}'", self.args.ds_operation()).into())
        };

        Ok(())
    }

    // --------------------------------------------------------------------------------
    // Private

    fn read_stocks(self: &mut Self) -> Result<(), Box<dyn Error>> {
        if let Some(file) = self.args.stocks_file() {
            let reader = stocks_reader::StocksReader::new(String::from(file));
            self.stocks = reader.read()?;
        }
        Ok(())
    }

    fn update(self: &Self) -> Result<(), Box<dyn Error>> {
        if self.args.stocks_file().is_none() {
            return Err("Missing stocks file for update operation".into());
        }

        let stck_count = self.stocks.len();
        let mut upd_count: usize = 0;
        let mut err_count: usize = 0;

        for stck in self.stocks.iter() {
            match self.update_stock_data(&stck.symbol) {
                Ok(_) => upd_count += 1,
                Err(err) => {
                    eprintln!("{}: {}", stck.symbol, err);
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

    fn update_stock_data(self: &Self, symbol: &str) -> Result<(), Box<dyn Error>> {
        self.update_stock_history(symbol)?;
        self.update_stock_dividends(symbol)?;
        Ok(())
    }

    fn update_stock_history(self: &Self, _symbol: &str) -> Result<(), Box<dyn Error>> {
        // TODO
        Ok(())
    }

    fn update_stock_dividends(self: &Self, _symbol: &str) -> Result<(), Box<dyn Error>> {
        // TODO
        Ok(())
    }

    fn check(self: &Self) -> Result<(), Box<dyn Error>> {
        let ds = datastore::DataStore::new(self.args.ds_root(), self.args.ds_name());
        if !ds.exists() {
            return Err(format!("Datastore {} does not exists", ds).into());
        }

        let mut count: usize = 0;

        for entry in fs::read_dir(ds.base_path())? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_file() {
                if let Err(err) = self.check_entry(&ds, &entry_path) {
                    count += 1;
                    eprintln!("{}: {}", entry.file_name().to_str().unwrap(), err);
                }
            }
        }

        println!("Check found {} error{}", count, if count == 1 { "" } else { "s" });
        Ok(())
    }

    fn check_entry(self: &Self, ds: &datastore::DataStore, entry_path: &Path) -> Result<(), Box<dyn Error>> {
        let content = ds.read_file(&entry_path)?;

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
        let ds = datastore::DataStore::new(self.args.ds_root(), self.args.ds_name());
        if ds.exists() {
            return Err(format!("Datastore {} already exists", ds).into());
        }

        ds.create()?;

        println!("Datastore {} created", ds);
        Ok(())
    }

    fn delete(self: &Self) -> Result<(), Box<dyn Error>> {
        let ds = datastore::DataStore::new(self.args.ds_root(), self.args.ds_name());
        if !ds.exists() {
            return Err(format!("Datastore {} does not exists", ds).into());
        }

        ds.delete()?;

        println!("Datastore {} deleted", ds);
        Ok(())
    }
}
