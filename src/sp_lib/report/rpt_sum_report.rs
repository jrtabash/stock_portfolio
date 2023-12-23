use std::fs::File;
use std::io::prelude::*;

use crate::portfolio::algorithms;
use crate::portfolio::stock::{Price, Stock, StockList};
use crate::report::report_params::ReportParams;
use crate::report::report_trait::Report;
use crate::util::datetime;
use crate::util::error::Error;

pub struct SumReport {}

impl Report for SumReport {
    fn write(&self, params: &ReportParams) {
        let stocks = params.stocks();

        println!("Stocks Summary Report");
        println!("---------------------");
        println!("            Date: {}", datetime::today().format("%Y-%m-%d"));
        println!("Number of Stocks: {}", stocks.len());
        println!();

        println!("{:>11} {:>12} {:>12} {:>12} {:>12}", "Name", "Value", "Minimum", "Average", "Maximum");
        println!("{:>11} {:>12} {:>12} {:>12} {:>12}", "----", "-----", "-------", "-------", "-------");

        write_table(
            stocks,
            |name, value, min, avg, max| {
                println!("{:>11} {:>12.2} {:>12.2} {:>12.2} {:>12.2}", name, value, min, avg, max);
                Ok(())
            }).unwrap();
    }

    fn export(&self, params: &ReportParams, filename: &str) -> Result<(), Error> {
        let stocks = params.stocks();
        let mut file = File::create(filename)?;

        writeln!(file, "Name,Value,Minimum,Average,Maximum")?;

        write_table(
            stocks,
            |name, value, min, avg, max| {
                writeln!(file, "{},{:.2},{:.2},{:.2},{:.2}", name, value, min, avg, max)?;
                Ok(())
            })?;

        Ok(())
    }
}

// --------------------------------------------------------------------------------
// Private

struct Stats {
    pub cnt: usize,
    pub sum: Price,
    pub min: Price,
    pub max: Price
}

impl Stats {
    #[inline(always)]
    pub fn mean(&self) -> Price {
        if self.cnt > 0 {
            self.sum / self.cnt as Price
        } else {
            0.0
        }
    }
}

fn calc_stats(stocks: &StockList, extract: impl Fn(&Stock) -> Price) -> Stats {
    let values: Vec<Price> = stocks.iter().map(extract).collect();
    Stats {
        cnt: values.len(),
        sum: values.iter().sum(),
        min: *values.iter().reduce(|acc, val| if acc < val { acc } else { val }).unwrap_or(&0.0),
        max: *values.iter().reduce(|acc, val| if acc > val { acc } else { val }).unwrap_or(&0.0)
    }
}

fn write_table(stocks: &StockList, mut writer: impl FnMut(&str, Price, Price, Price, Price) -> Result<(), Error>) -> Result<(), Error> {
    let base_stat = calc_stats(stocks, |s| s.base_notional());
    writer("Base Value", base_stat.sum, base_stat.min, base_stat.mean(), base_stat.max)?;

    let latest_stat = calc_stats(stocks, |s| s.latest_notional());
    writer("Last Value", latest_stat.sum, latest_stat.min, latest_stat.mean(), latest_stat.max)?;

    let net_stat = calc_stats(stocks, |s| s.net_notional());
    writer("Net Value", net_stat.sum, net_stat.min, net_stat.mean(), net_stat.max)?;

    let pct_chg = algorithms::pct_change(stocks);
    let pct_stat = calc_stats(stocks, |s| s.pct_change());
    writer("Pct Change", pct_chg, pct_stat.min, pct_stat.mean(), pct_stat.max)?;

    Ok(())
}
