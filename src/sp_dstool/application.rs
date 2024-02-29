use crate::arguments::Arguments;
use sp_lib::datastore::{datastore, dividends, export, history, splits};
use sp_lib::portfolio::{algorithms, stocks_config};
use sp_lib::util::{common_app, datetime, misc};
use sp_lib::util::error::Error;
use sp_lib::yfinance::{query, types};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

const UPDATE: &str = "update";
const DROP: &str = "drop";
const RESET: &str = "reset";
const CHECK: &str = "check";
const CREATE: &str = "create";
const DELETE: &str = "delete";
const STAT: &str = "stat";
const SHOWH: &str = "showh";
const SHOWD: &str = "showd";
const SHOWS: &str = "shows";
const EXPORT: &str = "export";
const CONSYM: &str = "consym";
const SYMS: &str = "syms";

struct StatAgg {
    tot_size: u64,
    hist_size: u64,
    div_size: u64,
    splt_size: u64,
    hist_count: usize,
    div_count: usize,
    splt_count: usize
}

pub struct Application {
    args: Arguments,
    sym_dates: HashMap<String, datetime::SPDate>,
    config: stocks_config::StocksConfig,
    ds: datastore::DataStore
}

impl common_app::AppTrait for Application {
    fn new() -> Self {
        let args = Arguments::new();
        let config = stocks_config::StocksConfig::from_file(args.config_file()).expect("Missing config file");
        let ds = datastore::DataStore::new(config.ds_root(), config.ds_name());
        Application {
            args,
            sym_dates: HashMap::new(),
            config,
            ds
        }
    }

    fn run(&mut self) -> common_app::RunResult {
        if !self.ds.exists() && self.args.ds_operation() != CREATE {
            return Err(format!("Datastore {} does not exist", self.ds).into());
        }

        println!("Run {} on {}", self.args.ds_operation(), self.ds);
        if self.args.is_verbose() {
            println!("config: {}", self.args.config_file());
            println!("symbol: {}", if let Some(symbol) = self.args.symbol() { symbol } else { "" });
            println!("export: {}", if let Some(export) = self.args.export_file() { export } else { "" });
            println!("----------");
        }

        self.set_symbol_dates();

        match self.args.ds_operation().as_str() {
            UPDATE => self.update()?,
            DROP => self.drop()?,
            RESET => self.reset()?,
            CHECK => self.check()?,
            CREATE => self.create()?,
            DELETE => self.delete()?,
            STAT => self.stat()?,
            SHOWH => self.show_history()?,
            SHOWD => self.show_dividends()?,
            SHOWS => self.show_splits()?,
            EXPORT => self.export()?,
            CONSYM => self.contains_symbol()?,
            SYMS => self.list_symbols()?,
            _ => return Err(format!("Invalid ds_operation - '{}'", self.args.ds_operation()).into())
        };

        Ok(())
    }
}

impl Application {
    fn is_symbol_match(&self, expr: &str) -> bool {
        match self.args.symbol() {
            Some(symbol) => expr.contains(symbol),
            None => true
        }
    }

    fn is_dsop_reset(&self) -> bool {
        self.args.ds_operation().as_str() == RESET
    }

    fn set_symbol_dates(&mut self) {
        if self.args.is_verbose() {
            println!("Set symbol dates");
        }

        self.sym_dates = algorithms::stock_base_dates(self.config.stocks());
    }

    fn update(&self) -> Result<(), Error> {
        if self.args.is_verbose() {
            println!("Update stocks");
        }

        let sym_count = self.sym_dates.len();
        let mut upd_count: usize = 0;
        let mut err_count: usize = 0;

        for (symbol, base_date) in self.sym_dates.iter() {
            if !self.is_symbol_match(symbol) {
                continue;
            }

            if self.args.is_verbose() {
                println!("Update {}", symbol);
            }

            match self.update_stock_data(symbol, base_date) {
                Ok(_) => upd_count += 1,
                Err(err) => {
                    eprintln!("{}: {}", symbol, err);
                    err_count += 1;
                }
            };
        }

        println!("Updated {} out of {}", upd_count, misc::count_format(sym_count, "symbol"));
        if err_count == 0 {
            Ok(())
        } else {
            Err(format!("Failed to update {}", misc::count_format(err_count, "stock")).into())
        }
    }

    fn perform_update(&self, symbol: &str, base_date: &datetime::SPDate) -> Result<bool, Error> {
        self.update_stock_history(symbol, base_date)?;
        let need_div_reset = self.update_stock_dividends(symbol, base_date)?;
        let need_slt_reset = self.update_stock_splits(symbol, base_date)?;
        Ok(need_div_reset || need_slt_reset)
    }

    fn update_stock_data(&self, symbol: &str, base_date: &datetime::SPDate) -> Result<(), Error> {
        let need_reset = self.perform_update(symbol, base_date)?;
        if need_reset && self.args.is_auto_reset() {
            let count = self.perform_drop(symbol)?;
            println!("Auto Reset: dropped {} for symbol {}", misc::count_format(count, "file"), symbol);

            self.perform_update(symbol, base_date)?;
            println!("Auto Reset: Updated {}", symbol);
        }
        Ok(())
    }

    fn update_stock_history(&self, symbol: &str, base_date: &datetime::SPDate) -> Result<(), Error> {
        let hist = if self.ds.symbol_exists(history::tag(), symbol) {
            history::History::ds_select_last(&self.ds, symbol)?
        } else {
            history::History::new(symbol)
        };

        if hist.count() > 1 {
            return Err(format!("Found unexpected history query result size {}, expected 0 or 1", hist.count()).into());
        }

        let begin_date = if hist.count() == 1 {
            datetime::date_plus_days(&hist.entries()[0].date, 1)
        } else {
            *base_date
        };

        let today = datetime::today();
        if begin_date <= today {
            let mut query = query::HistoryQuery::new(
                symbol.to_string(),
                begin_date,
                datetime::date_plus_days(&today, 1),
                types::Interval::Daily,
                types::Events::History
            );

            query.execute()?;
            self.ds.insert_symbol(history::tag(), symbol, &query.result)?;
        }
        Ok(())
    }

    fn update_stock_dividends(&self, symbol: &str, base_date: &datetime::SPDate) -> Result<bool, Error> {
        let mut result = false;

        let div = if self.ds.symbol_exists(dividends::tag(), symbol) {
            dividends::Dividends::ds_select_last(&self.ds, symbol)?
        } else {
            dividends::Dividends::new(symbol)
        };

        if div.count() > 1 {
            return Err(format!("Found unexpected dividends query result size {}, expected 0 or 1", div.count()).into());
        }

        let begin_date = if div.count() == 1 {
            datetime::date_plus_days(&div.entries()[0].date, 1)
        } else {
            *base_date
        };

        let today = datetime::today();
        if begin_date <= today {
            let mut query = query::HistoryQuery::new(
                symbol.to_string(),
                begin_date,
                datetime::date_plus_days(&today, 1),
                types::Interval::Daily,
                types::Events::Dividend
            );

            query.execute()?;
            if self.ds.insert_symbol(dividends::tag(), symbol, &query.result)? > 0 && !self.is_dsop_reset() {
                if self.args.is_auto_reset() {
                    result = true;
                } else {
                    println!("Dividends updated, check if {} data reset is needed", symbol);
                }
            }
        }

        Ok(result)
    }

    fn update_stock_splits(&self, symbol: &str, base_date: &datetime::SPDate) -> Result<bool, Error> {
        let mut result = false;

        let splt = if self.ds.symbol_exists(splits::tag(), symbol) {
            splits::Splits::ds_select_last(&self.ds, symbol)?
        } else {
            splits::Splits::new(symbol)
        };

        if splt.count() > 1 {
            return Err(format!("Found unexpected splits query result size {}, expected 0 or 1", splt.count()).into());
        }

        let begin_date = if splt.count() == 1 {
            datetime::date_plus_days(&splt.entries()[0].date, 1)
        } else {
            *base_date
        };

        let today = datetime::today();
        if begin_date <= today {
            let mut query = query::HistoryQuery::new(
                symbol.to_string(),
                begin_date,
                datetime::date_plus_days(&today, 1),
                types::Interval::Daily,
                types::Events::Split
            );

            query.execute()?;
            if self.ds.insert_symbol(splits::tag(), symbol, &query.result)? > 0 && !self.is_dsop_reset() {
                if self.args.is_auto_reset() {
                    result = true;
                } else {
                    println!("Splits updated, check if {} data reset is needed", symbol);
                }
            }
        }

        Ok(result)
    }

    fn drop(&self) -> Result<(), Error> {
        if self.args.is_verbose() {
            println!("Drop symbol");
        }

        if self.args.symbol().is_none() {
            return Err("Missing symbol for drop operation".into());
        }

        let symbol = self.args.symbol().unwrap();
        let count = self.perform_drop(symbol)?;
        println!("Dropped {} for symbol {}", misc::count_format(count, "file"), symbol);
        Ok(())
    }

    fn perform_drop(&self, symbol: &str) -> Result<usize, Error> {
        let mut count: usize = 0;
        count += self.drop_symbol(history::tag(), symbol)?;
        count += self.drop_symbol(dividends::tag(), symbol)?;
        count += self.drop_symbol(splits::tag(), symbol)?;
        Ok(count)
    }

    fn drop_symbol(&self, tag: &str, symbol: &str) -> Result<usize, Error> {
        let mut count: usize = 0;
        if self.ds.symbol_exists(tag, symbol) {
            self.ds.drop_symbol(tag, symbol)?;
            count += 1;
        }
        Ok(count)
    }

    fn reset(&self) -> Result<(), Error> {
        if self.args.is_verbose() {
            println!("Reset symbol");
        }

        if self.args.symbol().is_none() {
            return Err("Missing symbol for reset operation".into());
        }

        self.drop()?;
        self.update()?;
        Ok(())
    }

    fn show_data(&self, tag: &str) -> Result<(), Error> {
        if self.args.is_verbose() {
            println!("Show {}", tag);
        }

        if self.args.symbol().is_none() {
            return Err(format!("Missing symbol for show {} operation", tag).into());
        }

        let symbol = self.args.symbol().unwrap();
        if self.ds.symbol_exists(tag, symbol) {
            self.ds.show_symbol(tag, symbol)?;
        }

        Ok(())
    }

    fn show_history(&self) -> Result<(), Error> {
        self.show_data(history::tag())
    }

    fn show_dividends(&self) -> Result<(), Error> {
        self.show_data(dividends::tag())
    }

    fn show_splits(&self) -> Result<(), Error> {
        self.show_data(splits::tag())
    }

    fn export(&self) -> Result<(), Error> {
        if self.args.is_verbose() {
            println!("Export datastore");
        }

        if self.args.symbol().is_none() {
            return Err("Missing symbol for export operation".into());
        }

        if self.args.export_file().is_none() {
            return Err("Missing export file for export operation".into());
        }

        let symbol = self.args.symbol().unwrap();
        let export_file = self.args.export_file().unwrap();
        let count = export::export_symbol(&self.ds, symbol, export_file)?;

        println!("Exported {} for symbol {}", misc::count_format(count, "row"), symbol);
        Ok(())
    }

    fn contains_symbol(&self) -> Result<(), Error> {
        if self.args.is_verbose() {
            println!("Check datastore contains symbol");
        }

        match self.args.symbol() {
            Some(sym) => {
                let inds = if self.ds.symbol_exists(history::tag(), sym) { "" } else { "not " };
                println!("Symbol {} {}in datastore", sym, inds);
            }
            None => println!("Missing symbol for consym operation")
        };

        Ok(())
    }

    fn list_symbols(&self) -> Result<(), Error> {
        if self.args.is_verbose() {
            println!("List datastore symbols");
        }

        let stocks: HashSet<&str> = self.config.stocks().iter()
            .map(|s| s.symbol.as_str())
            .collect();
        let positions: HashSet<&str> = self.config.closed_positions().iter()
            .map(|p| p.symbol.as_str())
            .collect();
        for s in stocks.union(&positions) {
            println!("{}", s);
        }

        Ok(())
    }

    fn check(&self) -> Result<(), Error> {
        if self.args.is_verbose() {
            println!("Check datastore");
        }

        let (_, itm_count, err_count) = self.ds.foreach_entry(
            (),
            |entry, _| {
                if self.args.is_verbose() {
                    println!("Check entry {}", misc::direntry_filename(entry));
                }
                self.check_entry(&entry.path())?;
                Ok(())
            },
            |entry_str| self.is_symbol_match(entry_str),
            |entry, err| {
                eprintln!("{}: {}", misc::direntry_filename(entry), err);
                Ok(())
            }
        )?;

        println!(
            "Checked {} found {}",
            misc::count_format(itm_count, "item"),
            misc::count_format(err_count, "error")
        );
        Ok(())
    }

    fn check_entry(&self, entry_path: &Path) -> Result<(), Error> {
        let content = self.ds.read_file(entry_path)?;

        let fname = misc::path_basename(entry_path)?;
        if fname.starts_with(history::tag()) {
            history::History::check_csv(&content)?;
        } else if fname.starts_with(dividends::tag()) {
            dividends::Dividends::check_csv(&content)?;
        } else if fname.starts_with(splits::tag()) {
            splits::Splits::check_csv(&content)?;
        } else {
            return Err("Unknown entry name".into());
        }

        Ok(())
    }

    fn create(&self) -> Result<(), Error> {
        if self.args.is_verbose() {
            println!("Create datastore");
        }

        self.ds.create()?;

        println!("Datastore {} created", self.ds);
        Ok(())
    }

    fn delete(&self) -> Result<(), Error> {
        if self.args.is_verbose() {
            println!("Delete datastore");
        }

        self.ds.delete()?;

        println!("Datastore {} deleted", self.ds);
        Ok(())
    }

    fn stat(&self) -> Result<(), Error> {
        if self.args.is_verbose() {
            println!("Stat datastore");
        }

        let (stat_agg, itm_count, err_count) = self.ds.foreach_entry(
            StatAgg {
                tot_size: 0,
                hist_size: 0,
                div_size: 0,
                splt_size: 0,
                hist_count: 0,
                div_count: 0,
                splt_count: 0
            },
            |entry, agg| {
                let size = fs::metadata(entry.path())?.len();
                agg.tot_size += size;

                let filename = misc::direntry_filename(entry);
                if filename.starts_with(history::tag()) {
                    agg.hist_count += 1;
                    agg.hist_size += size;
                } else if filename.starts_with(dividends::tag()) {
                    agg.div_count += 1;
                    agg.div_size += size;
                } else if filename.starts_with(splits::tag()) {
                    agg.splt_count += 1;
                    agg.splt_size += size;
                }

                if self.args.is_verbose() {
                    println!("{}\t{}", size, filename);
                }
                Ok(())
            },
            |entry_str| self.is_symbol_match(entry_str),
            |entry, err| {
                eprintln!("{}: {}", misc::direntry_filename(entry), err);
                Ok(())
            }
        )?;

        println!("Total Cnt: {}", misc::count_format(itm_count, "file"));
        println!(" Hist Cnt: {}", misc::count_format(stat_agg.hist_count, "file"));
        println!("  Div Cnt: {}", misc::count_format(stat_agg.div_count, "file"));
        println!(" Splt Cnt: {}", misc::count_format(stat_agg.splt_count, "file"));
        println!("Total Siz: {}", misc::count_format(stat_agg.tot_size as usize, "byte"));
        println!(" Hist Siz: {}", misc::count_format(stat_agg.hist_size as usize, "byte"));
        println!("  Div Siz: {}", misc::count_format(stat_agg.div_size as usize, "byte"));
        println!(" Splt Siz: {}", misc::count_format(stat_agg.splt_size as usize, "byte"));
        if err_count > 0 {
            println!("Error Cnt: {}", misc::count_format(err_count, "error"));
        }
        Ok(())
    }
}
