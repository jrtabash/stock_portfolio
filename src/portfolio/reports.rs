use std::io::prelude::*;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;

use crate::sputil::datetime;
use crate::portfolio::stock::*;
use crate::portfolio::algorithms::*;

// --------------------------------------------------------------------------------
// Portfolio Value (Gain & Loss) Report and Export

pub fn value_report(stocks: &StockList, groupby: bool) {
    println!("Stocks Value Report");
    println!("-------------------");
    println!("            Date: {}", datetime::today().format("%Y-%m-%d"));
    println!("Number of Stocks: {}", stocks.len());
    println!("      Base Value: {:.2}", base_notional(&stocks));
    println!("    Latest Value: {:.2}", latest_notional(&stocks));
    println!("       Net Value: {:.2}", net_notional(&stocks));
    println!("  Percent Change: {:.2}", pct_change(&stocks));
    println!("");

    println!("{:8} {:10} {:10} {:8} {:8} {:8} {:8} {:8} {:12} {:12} {:10}",
             "Ticker",
             "Buy Date",
             "Upd Date",
             "Size",
             "Base",
             "Cur",
             "Net",
             "Pct",
             "Base Value",
             "Cur Value",
             "Net Value");
    println!("{:8} {:10} {:10} {:8} {:8} {:8} {:8} {:8} {:12} {:12} {:10}",
             "------",
             "--------",
             "--------",
             "----",
             "----",
             "---",
             "---",
             "---",
             "----------",
             "---------",
             "---------");
    for stock in stocks.iter() {
        println!("{:8} {:10} {:10} {:8} {:8.2} {:8.2} {:8.2} {:8.2} {:12.2} {:12.2} {:10.2}",
                 stock.symbol,
                 stock.date.format("%Y-%m-%d"),
                 stock.latest_date.format("%Y-%m-%d"),
                 stock.quantity,
                 stock.base_price,
                 stock.latest_price,
                 stock.net_price(),
                 stock.pct_change(),
                 stock.base_notional(),
                 stock.latest_notional(),
                 stock.net_notional());
    }

    if groupby {
        println!("");
        println!("{:8} {:8} {:12}", "GroupBy", "Size", "Cur Value");
        println!("{:8} {:8} {:12}", "-------", "----", "---------");

        let groupby = stock_groupby(&stocks);

        let mut seen = HashSet::new();
        for stock in stocks.iter() {
            if seen.contains(&stock.symbol) { continue; }
            seen.insert(&stock.symbol);

            let size_value = groupby.get(&stock.symbol).unwrap();
            println!("{:8} {:8} {:12.2}", stock.symbol, size_value.0, size_value.1);
        }
    }
}

pub fn value_export(stocks: &StockList, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(&filename)?;
    write!(file, "Ticker,Buy Date,Upd Date,Size,Base,Cur,Net,Pct,Base Value,Cur Value,Net Value\n")?;
    for stock in stocks.iter() {
        write!(file, "{},{},{},{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2}\n",
               stock.symbol,
               stock.date.format("%Y-%m-%d"),
               stock.latest_date.format("%Y-%m-%d"),
               stock.quantity,
               stock.base_price,
               stock.latest_price,
               stock.net_price(),
               stock.pct_change(),
               stock.base_notional(),
               stock.latest_notional(),
               stock.net_notional())?;
    }
    Ok(())
}
