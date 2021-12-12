use std::error::Error;
use sp_lib::datastore::{datastore, history, dividends};
use sp_lib::stats::{description, hist_desc, hist_ftns};
use sp_lib::util::datetime;
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
        self.print_date_and_symbol();

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

    fn date_range<Entry>(entries: &Vec<Entry>, extract_date: impl Fn (&Entry) -> datetime::LocalDate) -> (datetime::LocalDate, datetime::LocalDate) {
        if entries.len() > 0 {
            (extract_date(&entries[0]), extract_date(&entries[entries.len() - 1]))
        }
        else {
            (datetime::earliest_date(), datetime::earliest_date())
        }
    }

    fn print_date_and_symbol(&self) {
        let (first_date, last_date) =
            if self.args.calculate() == DIVDESC {
                Application::date_range(&self.div.entries(), |entry| entry.date)
            } else {
                Application::date_range(&self.hist.entries(), |entry| entry.date)
            };

        println!("  from: {}", first_date.format("%Y-%m-%d"));
        println!("    to: {}", last_date.format("%Y-%m-%d"));
        println!("symbol: {}", self.args.symbol());
    }

    fn load_data(&mut self) -> Result<(), Box<dyn Error>> {
        let symbol = self.args.symbol();
        if self.args.calculate() == DIVDESC {
            if self.ds.symbol_exists(dividends::tag(), symbol) {
                self.div = match self.args.from() {
                    Some(from) => dividends::Dividends::ds_select_if(&self.ds, symbol, |entry| entry.date >= from)?,
                    None => dividends::Dividends::ds_select_all(&self.ds, symbol)?
                };
            }
        }
        else if self.ds.symbol_exists(history::tag(), symbol) {
            self.hist = match self.args.from() {
                Some(from) => history::History::ds_select_if(&self.ds, symbol, |entry| entry.date >= from)?,
                None => history::History::ds_select_all(&self.ds, symbol)?
            };
        }
        Ok(())
    }

    fn describe(self: &Self) -> Result<(), Box<dyn Error>> {
        let desc = hist_desc::HistDesc::from_hist(&self.hist);
        desc.print();
        Ok(())
    }

    fn div_describe(&self) -> Result<(), Box<dyn Error>> {
        let desc = description::Description::from_vec(&self.div.entries(), |entry| entry.price);
        desc.print("dividend");
        Ok(())
    }

    fn calc_vwap(&self) -> Result<(), Box<dyn Error>> {
        let vwap = hist_ftns::hist_vwap(&self.hist)?;
        println!(" field: adj_close");
        println!("  vwap: {:.4}", vwap);
        Ok(())
    }

    fn calc_mvwap(&self) -> Result<(), Box<dyn Error>> {
        let mvwap = hist_ftns::hist_mvwap(&self.hist, self.args.window())?;
        println!(" field: adj_close");
        println!(" mvwap: ");
        for p in mvwap.iter() {
            println!("{:.4}", p);
        }
        Ok(())
    }

    fn calc_roc(&self) -> Result<(), Box<dyn Error>> {
        let roc = hist_ftns::hist_roc(&self.hist, self.args.window())?;
        println!(" field: adj_close");
        println!("   roc: ");
        for p in roc.iter() {
            println!("{:.4}", p);
        }
        Ok(())
    }
}
