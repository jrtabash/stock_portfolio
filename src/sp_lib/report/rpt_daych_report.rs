use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::iter::zip;

use crate::datastore::datastore::DataStore;
use crate::datastore::history::History;
use crate::portfolio::stock::{Price, Stock, StockList};
use crate::portfolio::stock_type::StockType;
use crate::report::report_params::ReportParams;
use crate::report::report_trait::Report;
use crate::util::datetime;
use crate::util::error::Error;

pub struct DaychReport {}

impl Report for DaychReport {
    fn write(&self, params: &ReportParams) {
        let stocks = params.stocks();
        let ds = params.datastore().expect("Daych report missing datastore");
        let changes: DayChangeList = stocks
            .iter()
            .map(|s| calc_daych(s, ds))
            .collect();
        let value_change: Price = changes
            .iter()
            .filter(|c| c.is_some())
            .map(|c| c.as_ref().unwrap().val_change)
            .sum::<Price>();

        println!("Stocks Day Change Report");
        println!("------------------------");
        println!("              Date: {}", datetime::today().format("%Y-%m-%d"));
        println!("  Number of Stocks: {}", stocks.len());
        println!("Total Value Change: {:0.2}", value_change);
        println!();

        println!("{:8} {:10} {:8} {:8} {:8} {:8} {:8} {:8} {:8} {:10}",
                 "Symbol",
                 "Upd Date",
                 "Prev Pr",
                 "Price",
                 "Change",
                 "Pct Chg",
                 "Val Chg",
                 "Low",
                 "High",
                 "Volume");

        println!("{:8} {:10} {:8} {:8} {:8} {:8} {:8} {:8} {:8} {:10}",
                 "------",
                 "--------",
                 "-------",
                 "-----",
                 "------",
                 "-------",
                 "-------",
                 "---",
                 "----",
                 "------");

        let agg_value_changes = calc_agg_value_changes(stocks, &changes);
        let mut seen = HashSet::new();
        for (stock, change) in zip(stocks, &changes) {
            if seen.contains(&stock.symbol) { continue; }

            if let Some(chg) = change {
                seen.insert(&stock.symbol);
                println!("{:8} {:10} {:8.2} {:8.2} {:8.2} {:8.2} {:8.2} {:8.2} {:8.2} {:10}",
                         stock.symbol,
                         stock.latest_date.format("%Y-%m-%d"),
                         chg.prev_price,
                         chg.price,
                         chg.change,
                         chg.pct_change,
                         agg_value_changes.get(&stock.symbol).unwrap_or(&0.0),
                         chg.low,
                         chg.high,
                         chg.volume);
            }
        }
    }

    fn export(&self, params: &ReportParams, filename: &str) -> Result<(), Error> {
        let stocks = params.stocks();
        let ds = params.datastore().expect("Daych report missing datastore");

        let mut file = File::create(filename)?;
        writeln!(file, "Symbol,Upd Date,Prev Pr,Price,Change,Pct Chg,Val Chg,Low,High,Volume")?;

        let changes: DayChangeList = stocks
            .iter()
            .map(|s| calc_daych(s, ds))
            .collect();
        let agg_value_changes = calc_agg_value_changes(stocks, &changes);

        let mut seen = HashSet::new();
        for (stock, change) in zip(stocks, &changes) {
            if seen.contains(&stock.symbol) { continue; }

            if let Some(chg) = change {
                seen.insert(&stock.symbol);
                writeln!(file, "{},{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{}",
                         stock.symbol,
                         stock.latest_date.format("%Y-%m-%d"),
                         chg.prev_price,
                         chg.price,
                         chg.change,
                         chg.pct_change,
                         agg_value_changes.get(&stock.symbol).unwrap_or(&0.0),
                         chg.low,
                         chg.high,
                         chg.volume)?;
            }
        }
        Ok(())
    }
}

// --------------------------------------------------------------------------------
// Private

struct DayChange {
    prev_price: Price,
    price: Price,
    change: Price,
    pct_change: Price,
    val_change: Price,
    low: Price,
    high: Price,
    volume: u64
}

type DayChangeList = Vec<Option<DayChange>>;
type AggValChanges = HashMap<String, Price>;

fn calc_daych(stock: &Stock, ds: &DataStore) -> Option<DayChange> {
    if let Ok(hist) = History::ds_select_last_n(ds, &stock.symbol, 2) {
        let entries = hist.entries();
        if entries.len() == 2 {
            let prev_price = entries[0].adj_close;
            let delta = entries[1].adj_close - prev_price;
            return Some(DayChange {
                prev_price,
                price: entries[1].adj_close,
                change: delta,
                pct_change: 100.0 * if prev_price > 0.0 { delta / prev_price } else { 0.00 },
                val_change: if stock.stype != StockType::Index { stock.quantity as Price * delta } else { 0.0 },
                low: entries[1].low,
                high: entries[1].high,
                volume: entries[1].volume
            });
        }
    }
    None
}

fn calc_agg_value_changes(stocks: &StockList, changes: &DayChangeList) -> AggValChanges {
    let mut agg_value_changes: AggValChanges = AggValChanges::new();
    for (stock, change) in zip(stocks, changes) {
        if let Some(chg) = change {
            let entry = agg_value_changes.entry(stock.symbol.to_string()).or_insert(0.00);
            *entry += chg.val_change;
        }
    }
    agg_value_changes
}
