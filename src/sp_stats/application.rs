use crate::arguments::Arguments;
use sp_lib::datastore::{datastore, dividends, history};
use sp_lib::stats::{description, hist_desc, hist_ftns};
use sp_lib::portfolio::stocks_config;
use sp_lib::util::{common_app, datetime};
use std::error::Error;

const DESC: &str = "desc";
const DIVDESC: &str = "divdesc";
const SA: &str = "sa";
const VWAP: &str = "vwap";
const VOLAT: &str = "volat";
const SMA: &str = "sma";
const MVWAP: &str = "mvwap";
const ROC: &str = "roc";
const PCTCH: &str = "pctch";
const MVOLAT: &str = "mvolat";
const RSI: &str = "rsi";

pub struct Application {
    args: Arguments,
    ds: datastore::DataStore,
    hist: history::History,
    div: dividends::Dividends
}

impl common_app::AppTrait for Application {
    fn new() -> Self {
        let args = Arguments::new();
        let config = stocks_config::StocksConfig::from_file(args.config_file()).expect("Missing config file");
        let ds = datastore::DataStore::new(config.ds_root(), config.ds_name());
        Application {
            args: args,
            ds: ds,
            hist: history::History::new(""),
            div: dividends::Dividends::new("")
        }
    }

    fn run(self: &mut Self) -> common_app::RunResult {
        if !self.ds.exists() {
            return Err(format!("Datastore {} does not exist", self.ds).into());
        }

        self.load_data()?;
        self.print_date_and_symbol();

        match self.args.calculate().as_str() {
            DESC => self.describe()?,
            DIVDESC => self.div_describe()?,
            SA => self.calc_sa()?,
            VWAP => self.calc_vwap()?,
            VOLAT => self.calc_volat()?,
            SMA => self.calc_sma()?,
            MVWAP => self.calc_mvwap()?,
            ROC => self.calc_roc()?,
            PCTCH => self.calc_pctch()?,
            MVOLAT => self.calc_mvolat()?,
            RSI => self.calc_rsi()?,
            _ => return Err(format!("Invalid calculate option - '{}'", self.args.calculate()).into())
        };

        Ok(())
    }
}

impl Application {
    fn date_range<Entry>(entries: &Vec<Entry>, extract_date: impl Fn(&Entry) -> datetime::SPDate) -> (datetime::SPDate, datetime::SPDate) {
        if entries.len() > 0 {
            (extract_date(&entries[0]), extract_date(&entries[entries.len() - 1]))
        } else {
            (datetime::earliest_date(), datetime::earliest_date())
        }
    }

    fn print_date_and_symbol(&self) {
        let (first_date, last_date) = if self.args.calculate() == DIVDESC {
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
        } else if self.ds.symbol_exists(history::tag(), symbol) {
            self.hist = match self.args.from() {
                Some(from) => history::History::ds_select_if(&self.ds, symbol, |entry| entry.date >= from)?,
                None => history::History::ds_select_all(&self.ds, symbol)?
            };
        }
        Ok(())
    }

    fn describe(self: &Self) -> Result<(), Box<dyn Error>> {
        fn print_field(name: &str, hd: &hist_desc::HistDesc, extract: impl Fn(&description::Description) -> f64) {
            println!(
                "{}: {:12.4} {:12.4} {:12.4} {:12.4} {:12.4} {:16.4}",
                name,
                extract(hd.open()),
                extract(hd.high()),
                extract(hd.low()),
                extract(hd.close()),
                extract(hd.adj_close()),
                extract(hd.volume())
            );
        }

        let hdesc = hist_desc::HistDesc::from_hist(&self.hist);

        println!(
            " field: {:>12} {:>12} {:>12} {:>12} {:>12} {:>16}",
            "open", "high", "low", "close", "adj_close", "volume"
        );
        println!(
            " count: {:>12} {:>12} {:>12} {:>12} {:>12} {:>16}",
            hdesc.open().count(),
            hdesc.high().count(),
            hdesc.low().count(),
            hdesc.close().count(),
            hdesc.adj_close().count(),
            hdesc.volume().count()
        );
        print_field("   min", &hdesc, |desc| desc.min());
        print_field("   max", &hdesc, |desc| desc.max());
        print_field("  mean", &hdesc, |desc| desc.mean());
        print_field("   std", &hdesc, |desc| desc.stddev());
        print_field("   25%", &hdesc, |desc| desc.lower_quartile());
        print_field("   50%", &hdesc, |desc| desc.median());
        print_field("   75%", &hdesc, |desc| desc.upper_quartile());
        Ok(())
    }

    fn div_describe(&self) -> Result<(), Box<dyn Error>> {
        let desc = description::Description::from_vec(&self.div.entries(), |entry| entry.price);
        println!(" field: dividend");
        println!(" count: {}", desc.count());
        println!("   min: {:.4}", desc.min());
        println!("   max: {:.4}", desc.max());
        println!("  mean: {:.4}", desc.mean());
        println!("   std: {:.4}", desc.stddev());
        println!("   25%: {:.4}", desc.lower_quartile());
        println!("   50%: {:.4}", desc.median());
        println!("   75%: {:.4}", desc.upper_quartile());
        Ok(())
    }

    fn calc_vwap(&self) -> Result<(), Box<dyn Error>> {
        let vwap = hist_ftns::hist_vwap(&self.hist)?;
        println!(" field: adj_close");
        println!("  vwap: {:.4}", vwap);
        Ok(())
    }

    fn calc_sa(&self) -> Result<(), Box<dyn Error>> {
        let sa = hist_ftns::hist_sa(&self.hist)?;
        println!(" field: adj_close");
        println!("    sa: {:.4}", sa);
        Ok(())
    }

    fn calc_volat(&self) -> Result<(), Box<dyn Error>> {
        let volat = hist_ftns::hist_volatility(&self.hist)?;
        println!(" field: adj_close");
        println!(" volat: {:.4}", volat);
        Ok(())
    }

    pub fn print_dp_list(dps: &hist_ftns::DatePriceList, name: &str, show_field: bool) {
        if show_field {
            println!(" field: adj_close");
        }
        println!("{:>6}: ", name);
        for (date, price) in dps.iter() {
            println!("{} {:.4}", date.format("%Y-%m-%d"), price);
        }
    }

    fn check_window(&self, min_window: usize) -> Result<(), Box<dyn Error>> {
        if self.args.window() < min_window {
            Err(format!("Window size {} less than required size {}", self.args.window(), min_window).into())
        } else {
            Ok(())
        }
    }

    fn calc_mvwap(&self) -> Result<(), Box<dyn Error>> {
        self.check_window(1)?;
        let mvwap = hist_ftns::hist_mvwap(&self.hist, self.args.window())?;
        Self::print_dp_list(&mvwap, MVWAP, true);
        Ok(())
    }

    fn calc_sma(&self) -> Result<(), Box<dyn Error>> {
        self.check_window(1)?;
        let sma = hist_ftns::hist_sma(&self.hist, self.args.window())?;
        Self::print_dp_list(&sma, SMA, true);
        Ok(())
    }

    fn calc_roc(&self) -> Result<(), Box<dyn Error>> {
        self.check_window(2)?;
        let roc = hist_ftns::hist_roc(&self.hist, self.args.window() - 1)?;
        Self::print_dp_list(&roc, ROC, true);
        Ok(())
    }

    fn calc_pctch(&self) -> Result<(), Box<dyn Error>> {
        let pctch = hist_ftns::hist_pctch(&self.hist)?;
        Self::print_dp_list(&pctch, PCTCH, true);
        Ok(())
    }

    fn calc_mvolat(&self) -> Result<(), Box<dyn Error>> {
        self.check_window(1)?;
        let mvolat = hist_ftns::hist_mvolatility(&self.hist, self.args.window())?;
        Self::print_dp_list(&mvolat, MVOLAT, true);
        Ok(())
    }

    fn calc_rsi(&self) -> Result<(), Box<dyn Error>> {
        self.check_window(2)?;
        let rsi = hist_ftns::hist_rsi(&self.hist, self.args.window())?;
        Self::print_dp_list(&rsi, RSI, false);
        Ok(())
    }
}
