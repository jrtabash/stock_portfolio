use std::fs::File;
use std::io::prelude::*;

use crate::datastore::datastore::DataStore;
use crate::datastore::history::History;
use crate::portfolio::stock::{Price, Stock};
use crate::report::report_params::ReportParams;
use crate::report::report_trait::Report;
use crate::stats::hist_ftns;
use crate::util::datetime;
use crate::util::error::Error;

pub struct VolatReport {}

impl Report for VolatReport {
    fn write(&self, params: &ReportParams) {
        let stocks = params.stocks();
        let ds = params.datastore().expect("Volat report missing datastore");

        println!("Stocks Volatility Report");
        println!("------------------------");
        println!("            Date: {}", datetime::today().format("%Y-%m-%d"));
        println!("Number of Stocks: {}", stocks.len());
        println!();

        println!("{:8} {:10} {:10} {:6} {:8} {:10}",
                 "Symbol",
                 "Buy Date",
                 "Upd Date",
                 "Days",
                 "Volat",
                 "Volat22");
        println!("{:8} {:10} {:10} {:6} {:8} {:10}",
                 "------",
                 "--------",
                 "--------",
                 "----",
                 "-----",
                 "-------");

        for stock in stocks.iter() {
            println!("{:8} {:10} {:10} {:6} {:8.2} {:10.2}",
                     stock.symbol,
                     stock.date.format("%Y-%m-%d"),
                     stock.latest_date.format("%Y-%m-%d"),
                     stock.days_held,
                     calc_volat(stock, ds),
                     calc_volat22(stock, ds));
        }
    }

    fn export(&self, params: &ReportParams, filename: &str) -> Result<(), Error> {
        let stocks = params.stocks();
        let ds = params.datastore().expect("Volat export missing datastore");

        let mut file = File::create(filename)?;
        writeln!(file, "Symbol,Buy Date,Upd Date,Days Held,Volat,Volat22")?;
        for stock in stocks.iter() {
            writeln!(file, "{},{},{},{},{:.2},{:.2}",
                     stock.symbol,
                     stock.date.format("%Y-%m-%d"),
                     stock.latest_date.format("%Y-%m-%d"),
                     stock.days_held,
                     calc_volat(stock, ds),
                     calc_volat22(stock, ds))?;
        }
        Ok(())
    }
}

// --------------------------------------------------------------------------------
// Private

const VOLAT_WIN: usize = 22;

fn calc_volat(stock: &Stock, ds: &DataStore) -> Price {
    if let Ok(hist) = History::ds_select_if(ds, &stock.symbol, |e| e.date >= stock.date) {
        if let Ok(volat) = hist_ftns::hist_volatility(&hist) {
            return volat
        }
    }
    0.0
}

fn calc_volat22(stock: &Stock, ds: &DataStore) -> Price {
    if let Ok(hist) = History::ds_select_if(ds, &stock.symbol, |e| e.date >= stock.date) {
        if hist.count() >= VOLAT_WIN {
            let start_idx = hist.count() - VOLAT_WIN;
            if let Ok(volat) = hist_ftns::entries_volatility(&hist.entries()[start_idx..]) {
                return volat
            }
        }
    }
    0.0
}
