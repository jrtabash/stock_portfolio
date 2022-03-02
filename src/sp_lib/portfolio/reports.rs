use std::io::prelude::*;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;

use crate::util::{price_type, datetime};
use crate::portfolio::stock::{Price, Stock, StockList};
use crate::portfolio::algorithms;

// --------------------------------------------------------------------------------
// Portfolio Value (Gain & Loss) Report and Export

pub fn value_report(stocks: &StockList, groupby: bool) {
    println!("Stocks Value Report");
    println!("-------------------");
    println!("            Date: {}", datetime::today().format("%Y-%m-%d"));
    println!("Number of Stocks: {}", stocks.len());
    println!("      Base Value: {:.2}", algorithms::base_notional(&stocks));
    println!("    Latest Value: {:.2}", algorithms::latest_notional(&stocks));
    println!("       Net Value: {:.2}", algorithms::net_notional(&stocks));
    println!("  Percent Change: {:.2}", algorithms::pct_change(&stocks));
    println!("    Cum Dividend: {:.2}", algorithms::cumulative_dividend(&stocks));
    println!("");

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
        println!("");
        println!("{:8} {:8} {:12}", "GroupBy", "Size", "Cur Value");
        println!("{:8} {:8} {:12}", "-------", "----", "---------");

        let groupby = algorithms::stock_aggregate(&stocks);

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
    write!(file, "Symbol,Buy Date,Upd Date,Days Held,Size,Base,Cur,Net,Pct,Base Value,Cur Value,Net Value,Cum Div\n")?;
    for stock in stocks.iter() {
        write!(file, "{},{},{},{},{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2}\n",
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

// --------------------------------------------------------------------------------
// Top/Bottom Performing Stocks Report and Export

type TopTuple<'a> = (&'a str, Price, Price, Price, Price, Price, Price);

fn make_top_tuple(stock: &Stock) -> TopTuple {
    (stock.symbol.as_str(),
     stock.pct_change(),
     stock.net_price(),
     stock.cum_dividend,
     stock.pct_change() / stock.days_held as Price,
     stock.net_price() / stock.days_held as Price,
     stock.cum_dividend / stock.days_held as Price
    )
}

pub fn top_report(stocks: &StockList, _groupby: bool) {
    fn prt_row(name: &str, d: &mut Vec<TopTuple>, ftn: fn (&TopTuple) -> Price) {
        d.sort_by(|lhs, rhs| price_type::price_cmp(ftn(lhs), ftn(rhs)));
        println!("{:18} {:8} {:8}", name, d[d.len() - 1].0, d[0].0);
    }

    println!("Stocks Top/Bottom Performing Report");
    println!("-----------------------------------");
    println!("            Date: {}", datetime::today().format("%Y-%m-%d"));
    println!("Number of Stocks: {}", stocks.len());
    println!("");

    println!("{:18} {:8} {:8}",
             "Category",
             "Top",
             "Bottom");
    println!("{:18} {:8} {:8}",
             "--------",
             "---",
             "------");

    let mut data: Vec<TopTuple> = stocks.iter().map(make_top_tuple).collect();
    if data.len() > 0 {
        prt_row("Pct Change", &mut data, |t| t.1);
        prt_row("Net Change", &mut data, |t| t.2);
        prt_row("Cum Div", &mut data, |t| t.3);
        prt_row("Pct Change / Day", &mut data, |t| t.4);
        prt_row("Net Change / Day", &mut data, |t| t.5);
        prt_row("Cum Div / Day", &mut data, |t| t.6);
    }

    // TODO: groupby support
}

pub fn top_export(stocks: &StockList, filename: &str) -> Result<(), Box<dyn Error>> {
    fn write_row(file: &mut File, name: &str, d: &mut Vec<TopTuple>, ftn: fn (&TopTuple) -> Price) -> Result<(), Box<dyn Error>>{
        d.sort_by(|lhs, rhs| price_type::price_cmp(ftn(lhs), ftn(rhs)));
        write!(file, "{},{},{}\n", name, d[d.len() - 1].0, d[0].0)?;
        Ok(())
    }

    let mut file = File::create(&filename)?;
    write!(file, "Category,Top,Bottom\n")?;

    let mut data: Vec<TopTuple> = stocks.iter().map(make_top_tuple).collect();
    if data.len() > 0 {
        write_row(&mut file, "Pct Change", &mut data, |t| t.1)?;
        write_row(&mut file, "Net Change", &mut data, |t| t.2)?;
        write_row(&mut file, "Cum Div", &mut data, |t| t.3)?;
        write_row(&mut file, "Pct Change / Day", &mut data, |t| t.4)?;
        write_row(&mut file, "Net Change / Day", &mut data, |t| t.5)?;
        write_row(&mut file, "Cum Div / Day", &mut data, |t| t.6)?;
    }
    Ok(())
}
