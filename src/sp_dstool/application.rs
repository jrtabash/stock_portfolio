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

    fn update(self: &mut Self) -> Result<(), Box<dyn Error>> {
        if self.args.stocks_file().is_none() {
            return Err("Missing stocks file for update operation".into());
        }

        // TODO
        eprintln!("Update {}/{} {}", self.args.ds_root(), self.args.ds_name(), self.args.stocks_file().unwrap());
        Ok(())
    }

    fn check(self: &mut Self) -> Result<(), Box<dyn Error>> {
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

    fn check_entry(self: &mut Self, ds: &datastore::DataStore, entry_path: &Path) -> Result<(), Box<dyn Error>> {
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

    fn create(self: &mut Self) -> Result<(), Box<dyn Error>> {
        let ds = datastore::DataStore::new(self.args.ds_root(), self.args.ds_name());
        if ds.exists() {
            return Err(format!("Datastore {} already exists", ds).into());
        }

        ds.create()?;

        println!("Datastore {} created", ds);
        Ok(())
    }

    fn delete(self: &mut Self) -> Result<(), Box<dyn Error>> {
        let ds = datastore::DataStore::new(self.args.ds_root(), self.args.ds_name());
        if !ds.exists() {
            return Err(format!("Datastore {} does not exists", ds).into());
        }

        ds.delete()?;

        println!("Datastore {} deleted", ds);
        Ok(())
    }
}
