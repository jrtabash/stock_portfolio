use std::io::prelude::*;
use std::fs::File;

use crate::util::error::Error;
use crate::datastore::{datastore, history, dividends};

pub fn export_symbol(ds: &datastore::DataStore, symbol: &str, filename: &str) -> Result<usize, Error> {
    let hist_data = history::History::ds_select_all(ds, symbol)?;
    let div_data =
        if ds.symbol_exists(dividends::tag(), symbol) {
            dividends::Dividends::ds_select_all(ds, symbol)?
        } else {
            dividends::Dividends::new(symbol)
        };

    let mut file = File::create(filename)?;
    writeln!(file, "date,open,high,low,close,adj_close,volume,dividend")?;

    let div_entries = div_data.entries();
    let div_size = div_entries.len();
    let mut idx = 0;

    let mut count: usize = 0;

    for hist_entry in hist_data.entries() {
        count += 1;

        write!(file, "{},{:.2},{:.2},{:.2},{:.2},{:.2},{},",
               hist_entry.date.format("%Y-%m-%d"),
               hist_entry.open,
               hist_entry.high,
               hist_entry.low,
               hist_entry.close,
               hist_entry.adj_close,
               hist_entry.volume)?;
        if idx < div_size && div_entries[idx].date == hist_entry.date {
            writeln!(file, "{:.2}", div_entries[idx].price)?;
            idx += 1;
        }
        else {
            writeln!(file, "0.00")?;
        }
    }

    Ok(count)
}
