use std::error::Error;
use sp_lib::portfolio::{stock, stocks_reader};
use sp_lib::datastore::datastore;
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
        // TODO
        eprintln!("Check {}/{}", self.args.ds_root(), self.args.ds_name());
        Ok(())
    }

    fn create(self: &mut Self) -> Result<(), Box<dyn Error>> {
        let ds = datastore::DataStore::new(self.args.ds_root(), self.args.ds_name());
        if ds.exists() {
            return Err(format!("Datastore {} already exists", ds).into());
        }

        if ds.create().is_err() {
            return Err(format!("Failed to create datastore {}", ds).into());
        }

        println!("Datastore {} created", ds);
        Ok(())
    }

    fn delete(self: &mut Self) -> Result<(), Box<dyn Error>> {
        // TODO
        eprintln!("delete {}/{}", self.args.ds_root(), self.args.ds_name());
        Ok(())
    }
}
