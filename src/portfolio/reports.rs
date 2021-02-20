use chrono::Local;

use crate::portfolio::stock::*;

pub fn value_report(stocks: &StockList) {
    let header = vec!["Ticker", "Date\t", "Qty", "Base", "Current", " Net", "NetVal", "CurVal"];
    let seprts = vec!["------", "----\t", "---", "----", "-------", " ---", "------", "------"];

    println!("Stocks Value Report");
    println!("-------------------");
    println!("            Date: {}", Local::today().format("%Y-%m-%d"));
    println!("Number of Stocks: {}", stocks.count());
    println!("       Net Value: {:.2}", stocks.net_notional());
    println!("   Current Value: {:.2}", stocks.current_notional());
    println!("");
    println!("{}", header.join("\t"));
    println!("{}", seprts.join("\t"));

    for stock in stocks.iter() {
        println!("{}\t{}\t{}\t{:.2}\t{:.2}\t {:.2}\t{:.2}\t{:.2}",
                 stock.symbol,
                 stock.date.format("%Y-%m-%d"),
                 stock.quantity,
                 stock.base_price,
                 stock.current_price,
                 stock.net_price(),
                 stock.net_notional(),
                 stock.current_notional());
    }
}
