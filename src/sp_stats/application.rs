use std::error::Error;
use sp_lib::datastore::{datastore, history, dividends};
use sp_lib::stats::{description, hist_desc};
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
const FIELD_VOLUME: &str = "volume";
const FIELD_DIVIDEND: &str = "dividend";

pub struct Application {
    args: Arguments,
    ds: datastore::DataStore,
    hist: history::History,
    div: dividends::Dividends,
    single_field: bool,
    div_field: bool
}

impl Application {
    pub fn new() -> Self {
        let args = Arguments::new();
        let ds = datastore::DataStore::new(args.ds_root(), args.ds_name());

        let mut single_field = false;
        let mut div_field = false;
        if let Some(field) = args.field() {
            single_field = true;
            div_field = field == FIELD_DIVIDEND;
        }

        Application {
            args: args,
            ds: ds,
            hist: history::History::new(""),
            div: dividends::Dividends::new(""),
            single_field: single_field,
            div_field: div_field
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

    #[inline(always)]
    fn is_single_field(&self) -> bool {
        self.single_field
    }

    #[inline(always)]
    fn is_dividend_field(&self) -> bool {
        self.div_field
    }

    fn load_data(&mut self) -> Result<(), Box<dyn Error>> {
        let symbol = self.args.symbol();
        if self.is_dividend_field() {
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
        if self.is_single_field() {
            let field = self.args.field().unwrap();
            let desc =
                if self.is_dividend_field() {
                    description::Description::from_vec(&self.div.entries(), |entry| entry.price)
                } else {
                    description::Description::from_vec(
                        &self.hist.entries(),
                        match field.as_str() {
                            FIELD_OPEN      => |entry: &history::HistoryEntry| entry.open,
                            FIELD_HIGH      => |entry: &history::HistoryEntry| entry.high,
                            FIELD_LOW       => |entry: &history::HistoryEntry| entry.low,
                            FIELD_CLOSE     => |entry: &history::HistoryEntry| entry.close,
                            FIELD_ADJ_CLOSE => |entry: &history::HistoryEntry| entry.adj_close,
                            FIELD_VOLUME    => |entry: &history::HistoryEntry| entry.volume as f64,
                            _               => return Err(format!("Unknown field {}", field).into())
                        })
                };
            desc.print(self.args.symbol(), field);
        }
        else {
            let desc = hist_desc::HistDesc::from_hist(&self.hist);
            desc.print(self.args.symbol());
        }
        Ok(())
    }

    fn calc_vwap(&self) -> Result<(), Box<dyn Error>> {
        // TODO
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
