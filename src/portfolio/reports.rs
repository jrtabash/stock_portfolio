use chrono::Local;

use crate::portfolio::stock::*;
use crate::portfolio::algorithms::*;

pub fn value_report(stocks: &StockList, groupby: bool) {
    println!("Stocks Value Report");
    println!("-------------------");
    println!("            Date: {}", Local::today().format("%Y-%m-%d"));
    println!("Number of Stocks: {}", stocks.len());
    println!("      Base Value: {:.2}", base_notional(&stocks));
    println!("    Latest Value: {:.2}", latest_notional(&stocks));
    println!("       Net Value: {:.2}", net_notional(&stocks));
    println!("");

    println!("{:8} {:10} {:10} {:8} {:8} {:8} {:8} {:12} {:12} {:10}",
             "Ticker",
             "Buy Date",
             "Upd Date",
             "Size",
             "Base",
             "Cur",
             "Net",
             "Base Value",
             "Cur Value",
             "Net Value");
    println!("{:8} {:10} {:10} {:8} {:8} {:8} {:8} {:12} {:12} {:10}",
             "------",
             "--------",
             "--------",
             "----",
             "----",
             "---",
             "---",
             "----------",
             "---------",
             "---------");
    for stock in stocks.iter() {
        println!("{:8} {:10} {:10} {:8} {:8.2} {:8.2} {:8.2} {:12.2} {:12.2} {:10.2}",
                 stock.symbol,
                 stock.date.format("%Y-%m-%d"),
                 stock.latest_update.format("%Y-%m-%d"),
                 stock.quantity,
                 stock.base_price,
                 stock.latest_price,
                 stock.net_price(),
                 stock.base_notional(),
                 stock.latest_notional(),
                 stock.net_notional());
    }

    if groupby {
        println!("");
        println!("{:8} {:8} {:12}", "GroupBy", "Size", "Cur Value");
        println!("{:8} {:8} {:12}", "-------", "----", "---------");
        for (symbol, size_value) in stock_groupby(&stocks).iter() {
            println!("{:8} {:8} {:12.2}", symbol, size_value.0, size_value.1);
        }
    }
}
