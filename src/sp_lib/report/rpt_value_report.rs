use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

use crate::portfolio::algorithms;
use crate::report::report_params::ReportParams;
use crate::report::report_trait::Report;
use crate::util::datetime;
use crate::util::error::Error;

pub struct ValueReport {}

impl Report for ValueReport {
    fn write(&self, params: &ReportParams) {
        let stocks = params.stocks();
        let groupby = params.groupby();

        let (pct_chg, pct_chg_wd) = algorithms::calc_pct_change(stocks);
        let latest_value = algorithms::latest_notional(stocks);

        println!("Stocks Value Report");
        println!("-------------------");
        println!("            Date: {}", datetime::today().format("%Y-%m-%d"));
        println!("Number of Stocks: {}", stocks.len());
        println!("      Base Value: {:.2}", algorithms::base_notional(stocks));
        println!("    Latest Value: {:.2}", latest_value);
        println!("       Net Value: {:.2}", algorithms::net_notional(stocks));
        println!("    Cum Dividend: {:.2}", algorithms::cumulative_dividend(stocks));
        println!("  Percent Change: {:.2}", pct_chg);
        println!("  Pct Chg w/ Div: {:.2}", pct_chg_wd);
        println!("            Cash: {:.2}", params.config().cash());
        println!("   Value w/ Cash: {:.2}", latest_value + params.config().cash());
        println!();

        println!("{:8} {:10} {:10} {:6} {:8} {:8} {:8} {:8} {:8} {:12} {:12} {:10} {:8}",
                 "Symbol",
                 "Buy Date",
                 "Upd Date",
                 "Days",
                 "Size",
                 "Base",
                 "Cur",
                 "Net",
                 "Pct",
                 "Base Value",
                 "Cur Value",
                 "Net Value",
                 "Cum Div");
        println!("{:8} {:10} {:10} {:6} {:8} {:8} {:8} {:8} {:8} {:12} {:12} {:10} {:8}",
                 "------",
                 "--------",
                 "--------",
                 "----",
                 "----",
                 "----",
                 "---",
                 "---",
                 "---",
                 "----------",
                 "---------",
                 "---------",
                 "-------");
        for stock in stocks.iter() {
            println!("{:8} {:10} {:10} {:6} {:8} {:8.2} {:8.2} {:8.2} {:8.2} {:12.2} {:12.2} {:10.2} {:8.2}",
                     stock.symbol,
                     stock.date.format("%Y-%m-%d"),
                     stock.latest_date.format("%Y-%m-%d"),
                     stock.days_held,
                     stock.quantity,
                     stock.base_price,
                     stock.latest_price,
                     stock.net_price(),
                     stock.pct_change(),
                     stock.base_notional(),
                     stock.latest_notional(),
                     stock.net_notional(),
                     stock.cum_dividend);
        }

        if groupby {
            println!();
            println!("{:8} {:8} {:12} {:12}", "GroupBy", "Size", "Base Value", "Cur Value");
            println!("{:8} {:8} {:12} {:12}", "-------", "----", "----------", "---------");

            let groupby = algorithms::stock_aggregate(stocks);

            let mut seen = HashSet::new();
            for stock in stocks.iter() {
                if seen.contains(&stock.symbol) { continue; }
                seen.insert(&stock.symbol);

                let size_values = groupby.get(&stock.symbol).unwrap();
                println!("{:8} {:8} {:12.2} {:12.2}", stock.symbol, size_values.0, size_values.1, size_values.2);
            }
        }
    }

    fn export(&self, params: &ReportParams, filename: &str) -> Result<(), Error> {
        let stocks = params.stocks();
        let mut file = File::create(filename)?;
        writeln!(file, "Symbol,Buy Date,Upd Date,Days Held,Size,Base,Cur,Net,Pct,Base Value,Cur Value,Net Value,Cum Div")?;
        for stock in stocks.iter() {
            writeln!(file, "{},{},{},{},{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2}",
                     stock.symbol,
                     stock.date.format("%Y-%m-%d"),
                     stock.latest_date.format("%Y-%m-%d"),
                     stock.days_held,
                     stock.quantity,
                     stock.base_price,
                     stock.latest_price,
                     stock.net_price(),
                     stock.pct_change(),
                     stock.base_notional(),
                     stock.latest_notional(),
                     stock.net_notional(),
                     stock.cum_dividend)?;
        }
        Ok(())
    }
}
