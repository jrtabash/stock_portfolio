use std::error::Error;
use sp_lib::datastore::{datastore, history, dividends};
use sp_lib::stats::{description, hist_desc, hist_ftns};
use crate::arguments::Arguments;

const DESC: &str = "desc";
const DIVDESC: &str = "divdesc";
const VWAP: &str = "vwap";
const MVWAP: &str = "mvwap";
const ROC: &str = "roc";

pub struct Application {
    args: Arguments,
    ds: datastore::DataStore,
    hist: history::History,
    div: dividends::Dividends,
}

impl Application {
    pub fn new() -> Self {
        let args = Arguments::new();
        let ds = datastore::DataStore::new(args.ds_root(), args.ds_name());
        Application {
            args: args,
            ds: ds,
            hist: history::History::new(""),
            div: dividends::Dividends::new("")
        }
    }

    pub fn run(self: &mut Self) -> Result<(), Box<dyn Error>> {
        if !self.ds.exists() {
            return Err(format!("Datastore {} does not exist", self.ds).into());
        }

        self.load_data()?;

        match self.args.calculate().as_str() {
            DESC => self.describe()?,
            DIVDESC => self.div_describe()?,
            VWAP => self.calc_vwap()?,
            MVWAP => self.calc_mvwap()?,
            ROC => self.calc_roc()?,
            _ => return Err(format!("Invalid calculate option - '{}'", self.args.calculate()).into())
        };

        Ok(())
    }

    // --------------------------------------------------------------------------------
    // Private

    fn load_data(&mut self) -> Result<(), Box<dyn Error>> {
        let symbol = self.args.symbol();
        if self.args.calculate() == DIVDESC {
            if self.ds.symbol_exists(dividends::tag(), symbol) {
                self.div = dividends::Dividends::ds_select_all(&self.ds, symbol)?;
            }
        }
        else if self.ds.symbol_exists(history::tag(), symbol) {
            self.hist = history::History::ds_select_all(&self.ds, self.args.symbol())?;
        }
        Ok(())
    }

    fn describe(self: &Self) -> Result<(), Box<dyn Error>> {
        let desc = hist_desc::HistDesc::from_hist(&self.hist);
        desc.print(self.args.symbol());
        Ok(())
    }

    fn div_describe(&self) -> Result<(), Box<dyn Error>> {
        let desc = description::Description::from_vec(&self.div.entries(), |entry| entry.price);
        desc.print(self.args.symbol(), "dividend");
        Ok(())
    }

    fn calc_vwap(&self) -> Result<(), Box<dyn Error>> {
        let vwap = hist_ftns::hist_vwap(&self.hist)?;
        println!("symbol: {}", self.args.symbol());
        println!(" field: adj_close");
        println!("  vwap: {:.4}", vwap);
        Ok(())
    }

    fn calc_mvwap(&self) -> Result<(), Box<dyn Error>> {
        // TODO
        Ok(())
    }

    fn calc_roc(&self) -> Result<(), Box<dyn Error>> {
        // TODO
        Ok(())
    }
}
