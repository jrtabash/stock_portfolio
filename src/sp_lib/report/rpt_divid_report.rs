use std::fs::File;
use std::io::prelude::*;

use crate::portfolio::algorithms;
use crate::report::report_params::ReportParams;
use crate::report::report_trait::Report;
use crate::util::datetime;
use crate::util::error::Error;

pub struct DividReport {}

impl Report for DividReport {
    fn write(&self, params: &ReportParams) {
        let stocks = params.stocks();

        println!("Stocks Dividend Report");
        println!("----------------------");
        println!("            Date: {}", datetime::today().format("%Y-%m-%d"));
        println!("Number of Stocks: {}", stocks.len());
        println!("    Cum Dividend: {:.2}", algorithms::cumulative_dividend(stocks));
        println!();

        println!("{:8} {:10} {:10} {:6} {:8} {:8} {:10} {:12}",
                 "Symbol",
                 "Buy Date",
                 "Upd Date",
                 "Days",
                 "Size",
                 "Cum Div",
                 "Yearly Div",
                 "Day Unit Div");
        println!("{:8} {:10} {:10} {:6} {:8} {:8} {:10} {:12}",
                 "------",
                 "--------",
                 "--------",
                 "----",
                 "----",
                 "-------",
                 "----------",
                 "------------");
        for stock in stocks.iter() {
            println!("{:8} {:10} {:10} {:6} {:8} {:8.2} {:10.2} {:12.6}",
                     stock.symbol,
                     stock.date.format("%Y-%m-%d"),
                     stock.latest_date.format("%Y-%m-%d"),
                     stock.days_held,
                     stock.quantity,
                     stock.cum_dividend,
                     stock.yearly_dividend(),
                     stock.daily_unit_dividend());
        }
    }

    fn export(&self, params: &ReportParams, filename: &str) -> Result<(), Error> {
        let stocks = params.stocks();
        let mut file = File::create(filename)?;
        writeln!(file, "Symbol,Buy Date,Upd Date,Days,Size,Cum Div,Yearly Div,Day Unit Div")?;
        for stock in stocks.iter() {
            writeln!(file, "{},{},{},{},{},{:.2},{:.2},{:.6}",
                     stock.symbol,
                     stock.date.format("%Y-%m-%d"),
                     stock.latest_date.format("%Y-%m-%d"),
                     stock.days_held,
                     stock.quantity,
                     stock.cum_dividend,
                     stock.yearly_dividend(),
                     stock.daily_unit_dividend())?;
        }
        Ok(())
    }
}
