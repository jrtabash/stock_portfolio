use std::collections::HashSet;
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
        let groupby = params.groupby();

        let cum_div = algorithms::cumulative_dividend(stocks);
        let bas_val = algorithms::base_notional(stocks);

        println!("Stocks Dividend Report");
        println!("----------------------");
        println!("            Date: {}", datetime::today().format("%Y-%m-%d"));
        println!("Number of Stocks: {}", stocks.len());
        println!(" Latest Dividend: {:.2}", algorithms::latest_dividend(stocks));
        println!("    Cum Dividend: {:.2}", cum_div);
        println!("Cum Dividend Ret: {:.2}", 100.0 * cum_div / bas_val);
        println!();

        println!("{:8} {:10} {:10} {:6} {:8} {:10} {:10} {:10} {:10} {:12} {:11}",
                 "Symbol",
                 "Buy Date",
                 "Upd Date",
                 "Days",
                 "Size",
                 "Latest DDt",
                 "Latest Div",
                 "Cum Div",
                 "Yr Div Est",
                 "Day Unit Div",
                 "Cum Div Ret");
        println!("{:8} {:10} {:10} {:6} {:8} {:10} {:10} {:10} {:10} {:12} {:11}",
                 "------",
                 "--------",
                 "--------",
                 "----",
                 "----",
                 "----------",
                 "----------",
                 "-------",
                 "----------",
                 "------------",
                 "-----------");
        for stock in stocks.iter() {
            println!("{:8} {:10} {:10} {:6} {:8} {:10} {:10.2} {:10.2} {:10.2} {:12.6} {:11.2}",
                     stock.symbol,
                     stock.date.format("%Y-%m-%d"),
                     stock.latest_date.format("%Y-%m-%d"),
                     stock.days_held,
                     stock.quantity,
                     stock.latest_div_date.format("%Y-%m-%d"),
                     stock.latest_dividend(),
                     stock.cum_dividend,
                     stock.yearly_dividend(),
                     stock.daily_unit_dividend(),
                     stock.cum_dividend_return());
        }

        if groupby {
            println!();
            println!("{:8} {:8} {:10} {:11}", "GroupBy", "Size", "Cum Duv", "Cum Div Ret");
            println!("{:8} {:8} {:10} {:11}", "-------", "----", "-------", "-----------");

            let groupby = algorithms::dividend_aggregate(stocks);

            let mut seen = HashSet::new();
            for stock in stocks.iter() {
                if seen.contains(&stock.symbol) { continue; }
                seen.insert(&stock.symbol);

                let size_prices = groupby.get(&stock.symbol).unwrap();
                println!("{:8} {:8} {:10.2} {:11.2}", stock.symbol, size_prices.0, size_prices.1, 100.0 * size_prices.1 / size_prices.2);
            }
        }
    }

    fn export(&self, params: &ReportParams, filename: &str) -> Result<(), Error> {
        let stocks = params.stocks();
        let mut file = File::create(filename)?;
        writeln!(file, "Symbol,Buy Date,Upd Date,Days,Size,Latest DDt,Latest Div,Cum Div,Yr Div Est,Day Unit Div,Cum Div Ret")?;
        for stock in stocks.iter() {
            writeln!(file, "{},{},{},{},{},{},{:.2},{:.2},{:.2},{:.6},{:.2}",
                     stock.symbol,
                     stock.date.format("%Y-%m-%d"),
                     stock.latest_date.format("%Y-%m-%d"),
                     stock.days_held,
                     stock.quantity,
                     stock.latest_div_date.format("%Y-%m-%d"),
                     stock.latest_dividend(),
                     stock.cum_dividend,
                     stock.yearly_dividend(),
                     stock.daily_unit_dividend(),
                     stock.cum_dividend_return())?;
        }
        Ok(())
    }
}
