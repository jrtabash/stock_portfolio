use std::error::Error;
use std::path::Path;
use std::fs;
use std::collections::HashMap;
use sp_lib::util::{datetime, misc};
use sp_lib::portfolio::{stocks_reader, algorithms};
use sp_lib::yfinance::{types, query};
use sp_lib::datastore::{datastore, history, dividends, splits, export};
use crate::arguments::Arguments;

const UPDATE: &str = "update";
const DROP: &str = "drop";
const CHECK: &str = "check";
const CREATE: &str = "create";
const DELETE: &str = "delete";
const STAT: &str = "stat";
const SHOWH: &str = "showh";
const SHOWD: &str = "showd";
const EXPORT: &str = "export";

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
    sym_dates: HashMap<String, datetime::LocalDate>,
    ds: datastore::DataStore
}

impl Application {
    pub fn new() -> Self {
        let args = Arguments::new();
        let ds = datastore::DataStore::new(args.ds_root(), args.ds_name());
        Application {
            args: args,
            sym_dates: HashMap::new(),
            ds: ds
        }
    }

    pub fn run(self: &mut Self) -> Result<(), Box<dyn Error>> {
        if !self.ds.exists() && self.args.ds_operation() != CREATE {
            return Err(format!("Datastore {} does not exist", self.ds).into());
        }

        println!("Run {} on {}", self.args.ds_operation(), self.ds);
        if self.args.is_verbose() {
            println!("stocks: {}", if let Some(file) = self.args.stocks_file() { file } else { "" });
            println!("symbol: {}", if let Some(symbol) = self.args.symbol() { symbol } else { "" });
            println!("export: {}", if let Some(export) = self.args.export_file() { export } else { "" });
            println!("----------");
        }

        self.read_stocks()?;

        match self.args.ds_operation().as_str() {
            UPDATE => self.update()?,
            DROP => self.drop()?,
            CHECK => self.check()?,
            CREATE => self.create()?,
            DELETE => self.delete()?,
            STAT => self.stat()?,
            SHOWH => self.show_history()?,
            SHOWD => self.show_dividends()?,
            EXPORT => self.export()?,
            _ => return Err(format!("Invalid ds_operation - '{}'", self.args.ds_operation()).into())
        };

        Ok(())
    }

    // --------------------------------------------------------------------------------
    // Private

    fn is_symbol_match(self: &Self, expr: &str) -> bool {
        match self.args.symbol() {
            Some(symbol) => expr.contains(symbol),
            None => true
        }
    }

    fn read_stocks(self: &mut Self) -> Result<(), Box<dyn Error>> {
        if self.args.is_verbose() { println!("Read stocks file"); }

        if let Some(file) = self.args.stocks_file() {
            let reader = stocks_reader::StocksReader::new(String::from(file));
            self.sym_dates = algorithms::stock_base_dates(&reader.read()?);
        }
        Ok(())
    }

    fn update(self: &Self) -> Result<(), Box<dyn Error>> {
        if self.args.is_verbose() { println!("Update stocks"); }

        if self.args.stocks_file().is_none() {
            return Err("Missing stocks file for update operation".into());
        }

        let sym_count = self.sym_dates.len();
        let mut upd_count: usize = 0;
        let mut err_count: usize = 0;

        for (symbol, base_date) in self.sym_dates.iter() {
            if !self.is_symbol_match(symbol) {
                continue;
            }

            if self.args.is_verbose() { println!("Update {}", symbol); }

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
        }
        else {
            Err(format!("Failed to update {}", misc::count_format(err_count, "stock")).into())
        }
    }

    fn update_stock_data(self: &Self, symbol: &str, base_date: &datetime::LocalDate) -> Result<(), Box<dyn Error>> {
        self.update_stock_history(symbol, base_date)?;
        self.update_stock_dividends(symbol, base_date)?;
        self.update_stock_splits(symbol, base_date)?;
        Ok(())
    }

    fn update_stock_history(self: &Self, symbol: &str, base_date: &datetime::LocalDate) -> Result<(), Box<dyn Error>> {
        let hist =
            if self.ds.symbol_exists(history::tag(), symbol) {
                history::History::ds_select_last(&self.ds, symbol)?
            } else {
                history::History::new(symbol)
            };

        if hist.count() > 1 {
            return Err(format!("Found unexpected history query result size {}, expected 0 or 1", hist.count()).into());
        }

        let begin_date =
            if hist.count() == 1 {
                datetime::date_plus_days(&hist.entries()[0].date, 1)
            } else {
                base_date.clone()
            };

        let today = datetime::today();
        if begin_date <= today {
            let mut query = query::HistoryQuery::new(
                symbol.to_string(),
                begin_date,
                datetime::date_plus_days(&today, 1),
                types::Interval::Daily,
                types::Events::History);

            query.execute()?;
            self.ds.insert_symbol(history::tag(), symbol, &query.result)?;
        }
        Ok(())
    }

    fn update_stock_dividends(self: &Self, symbol: &str, base_date: &datetime::LocalDate) -> Result<(), Box<dyn Error>> {
        let div =
            if self.ds.symbol_exists(dividends::tag(), symbol) {
                dividends::Dividends::ds_select_last(&self.ds, symbol)?
            } else {
                dividends::Dividends::new(symbol)
            };

        if div.count() > 1 {
            return Err(format!("Found unexpected dividends query result size {}, expected 0 or 1", div.count()).into());
        }

        let begin_date =
            if div.count() == 1 {
                datetime::date_plus_days(&div.entries()[0].date, 1)
            } else {
                base_date.clone()
            };

        let today = datetime::today();
        if begin_date <= today {
            let mut query = query::HistoryQuery::new(
                symbol.to_string(),
                begin_date,
                datetime::date_plus_days(&today, 1),
                types::Interval::Daily,
                types::Events::Dividend);

            query.execute()?;
            if self.ds.insert_symbol(dividends::tag(), symbol, &query.result)? > 0 {
                println!("Dividends updated, check if {} data reset is needed", symbol);
            }
        }
        Ok(())
    }

    fn update_stock_splits(self: &Self, symbol: &str, base_date: &datetime::LocalDate) -> Result<(), Box<dyn Error>> {
        let splt =
            if self.ds.symbol_exists(splits::tag(), symbol) {
                splits::Splits::ds_select_last(&self.ds, symbol)?
            } else {
                splits::Splits::new(symbol)
            };

        if splt.count() > 1 {
            return Err(format!("Found unexpected splits query result size {}, expected 0 or 1", splt.count()).into());
        }

        let begin_date =
            if splt.count() == 1 {
                datetime::date_plus_days(&splt.entries()[0].date, 1)
            } else {
                base_date.clone()
            };

        let today = datetime::today();
        if begin_date <= today {
            let mut query = query::HistoryQuery::new(
                symbol.to_string(),
                begin_date,
                datetime::date_plus_days(&today, 1),
                types::Interval::Daily,
                types::Events::Split);

            query.execute()?;
            if self.ds.insert_symbol(splits::tag(), symbol, &query.result)? > 0 {
                println!("Splits updated, check if {} data reset is needed", symbol);
            }
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
        count += self.drop_symbol(splits::tag(), &symbol)?;
        println!("Dropped {} for symbol {}", misc::count_format(count, "file"), symbol);
        Ok(())
    }

    fn drop_symbol(self: &Self, tag: &str, symbol: &str) -> Result<usize, Box<dyn Error>> {
        let mut count: usize = 0;
        if self.ds.symbol_exists(tag, symbol) {
            self.ds.drop_symbol(tag, &symbol)?;
            count += 1;
        }
        Ok(count)
    }

    fn show_history(self: &Self) -> Result<(), Box<dyn Error>> {
        if self.args.is_verbose() { println!("Show history"); }

        if self.args.symbol().is_none() {
            return Err("Missing symbol for show history operation".into())
        }

        let symbol = self.args.symbol().unwrap();
        if self.ds.symbol_exists(history::tag(), &symbol) {
            self.ds.show_symbol(history::tag(), &symbol)?;
        }

        Ok(())
    }

    fn show_dividends(self: &Self) -> Result<(), Box<dyn Error>> {
        if self.args.is_verbose() { println!("Show dividends"); }

        if self.args.symbol().is_none() {
            return Err("Missing symbol for show dividends operation".into())
        }

        let symbol = self.args.symbol().unwrap();
        if self.ds.symbol_exists(dividends::tag(), &symbol) {
            self.ds.show_symbol(dividends::tag(), &symbol)?;
        }

        Ok(())
    }

    fn export(self: &Self) -> Result<(), Box<dyn Error>> {
        if self.args.is_verbose() { println!("Export datastore"); }

        if self.args.symbol().is_none() {
            return Err("Missing symbol for export operation".into());
        }

        if self.args.export_file().is_none() {
            return Err("Missing export file for export operation".into());
        }

        let symbol = self.args.symbol().unwrap();
        let export_file = self.args.export_file().unwrap();
        let count = export::export_symbol(&self.ds, &symbol, &export_file)?;

        println!("Exported {} for symbol {}", misc::count_format(count, "row"), symbol);
        Ok(())
    }

    fn check(self: &Self) -> Result<(), Box<dyn Error>> {
        if self.args.is_verbose() { println!("Check datastore"); }

        let (_, itm_count, err_count) =
            self.ds.foreach_entry(
                (),
                |entry, _| {
                    if self.args.is_verbose() {
                        println!("Check entry {}", misc::direntry_filename(entry));
                    }
                    self.check_entry(&entry.path())?;
                    Ok(())
                },
                |entry_str| {
                    return self.is_symbol_match(entry_str)
                },
                |entry, err| {
                    eprintln!("{}: {}", misc::direntry_filename(entry), err);
                    Ok(())
                })?;

        println!("Checked {} found {}", misc::count_format(itm_count, "item"), misc::count_format(err_count, "error"));
        Ok(())
    }

    fn check_entry(self: &Self, entry_path: &Path) -> Result<(), Box<dyn Error>> {
        let content = self.ds.read_file(&entry_path)?;

        let fname = misc::path_basename(entry_path)?;
        if fname.starts_with(history::tag()) {
            history::History::check_csv(&content)?;
        }
        else if fname.starts_with(dividends::tag()) {
            dividends::Dividends::check_csv(&content)?;
        }
        else if fname.starts_with(splits::tag()) {
            splits::Splits::check_csv(&content)?;
        }
        else {
            return Err(format!("Unknown entry name").into())
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

    fn stat(self: &Self) -> Result<(), Box<dyn Error>> {
        if self.args.is_verbose() { println!("Stat datastore"); }

        let (stat_agg, itm_count, err_count) =
            self.ds.foreach_entry(
                StatAgg { tot_size: 0, hist_size: 0, div_size: 0, splt_size: 0, hist_count: 0, div_count: 0, splt_count: 0 },
                |entry, agg| {
                    let mut agg_ref = &mut *agg;
                    let size = fs::metadata(entry.path())?.len();
                    agg_ref.tot_size += size;

                    let filename = misc::direntry_filename(entry);
                    if filename.starts_with(history::tag()) {
                        agg_ref.hist_count += 1;
                        agg_ref.hist_size += size;
                    }
                    else if filename.starts_with(dividends::tag()) {
                        agg_ref.div_count += 1;
                        agg_ref.div_size += size;
                    }
                    else if filename.starts_with(splits::tag()) {
                        agg_ref.splt_count += 1;
                        agg_ref.splt_size += size;
                    }

                    if self.args.is_verbose() {
                        println!("{}\t{}", size, filename);
                    }
                    Ok(())
                },
                |entry_str| {
                    return self.is_symbol_match(entry_str)
                },
                |entry, err| {
                    eprintln!("{}: {}", misc::direntry_filename(entry), err);
                    Ok(())
                })?;

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
