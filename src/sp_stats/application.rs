use std::error::Error;
use sp_lib::datastore::{datastore, history, dividends};
use sp_lib::stats::{description};
use crate::arguments::Arguments;

const DESC: &str = "desc";
const VWAP: &str = "vwap";
const MVWAP: &str = "mvwap";
const ROC: &str = "roc";

const FIELD_OPEN: &str = "open";
const FIELD_HIGH: &str = "high";
const FIELD_LOW: &str = "low";
const FIELD_CLOSE: &str = "close";
const FIELD_ADJ_CLOSE: &str = "adj_close";
const FIELD_DIVIDEND: &str = "dividend";

pub struct Application {
    args: Arguments,
    ds: datastore::DataStore,
    hist: history::History,
    div: dividends::Dividends
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
            VWAP => self.calc_vwap()?,
            MVWAP => self.calc_mvwap()?,
            ROC => self.calc_roc()?,
            _ => return Err(format!("Invalid calculate option - '{}'", self.args.calculate()).into())
        };

        Ok(())
    }

    // --------------------------------------------------------------------------------
    // Private

    fn load_data(self: &mut Self) -> Result<(), Box<dyn Error>> {
        let symbol = self.args.symbol();
        if self.args.field() == FIELD_DIVIDEND {
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
        let desc =
            match self.args.field().as_str() {
                FIELD_OPEN => description::Description::from_vec(&self.hist.entries(), |entry| entry.open),
                FIELD_HIGH => description::Description::from_vec(&self.hist.entries(), |entry| entry.high),
                FIELD_LOW => description::Description::from_vec(&self.hist.entries(), |entry| entry.low),
                FIELD_CLOSE => description::Description::from_vec(&self.hist.entries(), |entry| entry.close),
                FIELD_ADJ_CLOSE => description::Description::from_vec(&self.hist.entries(), |entry| entry.adj_close),
                FIELD_DIVIDEND => description::Description::from_vec(&self.div.entries(), |entry| entry.price),
                _ => return Err(format!("Unknown field {}", self.args.field()).into())
            };
        println!("Symbol: {}", self.args.symbol());
        println!(" Field: {}", self.args.field());
        println!(" Count: {}", desc.count());
        println!("   Sum: {:.4}", desc.sum());
        println!("   Min: {:.4}", desc.min());
        println!("   Max: {:.4}", desc.max());
        println!("  Mean: {:.4}", desc.mean());
        println!("   Std: {:.4}", desc.stddev());
        Ok(())
    }

    fn calc_vwap(self: &Self) -> Result<(), Box<dyn Error>> {
        // TODO
        Ok(())
    }

    fn calc_mvwap(self: &Self) -> Result<(), Box<dyn Error>> {
        // TODO
        Ok(())
    }

    fn calc_roc(self: &Self) -> Result<(), Box<dyn Error>> {
        // TODO
        Ok(())
    }
}
