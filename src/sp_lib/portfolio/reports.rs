use std::io::prelude::*;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;

use crate::util::{price_type, datetime};
use crate::datastore::datastore::DataStore;
use crate::datastore::history::History;
use crate::stats::hist_ftns;
use crate::portfolio::stock::{Price, Stock, StockList};
use crate::portfolio::algorithms;
use crate::portfolio::report_type::ReportType;

pub struct ReportParams<'a, 'b> {
    rtype: ReportType,
    stocks: &'a StockList,
    ds: Option<&'b DataStore>,
    groupby: bool
}

impl<'a, 'b> ReportParams<'a, 'b> {
    pub fn new(rtype: ReportType, stocks: &'a StockList) -> Self {
        ReportParams { rtype: rtype, stocks: stocks, ds: None, groupby: false }
    }

    pub fn show_groupby(mut self, grpby: bool) -> Self {
        self.groupby = grpby;
        self
    }

    pub fn with_datastore(mut self, ds: &'b DataStore) -> Self {
        self.ds = Some(ds);
        self
    }

    #[inline(always)]
    pub fn rtype(&self) -> ReportType { self.rtype }

    #[inline(always)]
    pub fn stocks(&self) -> &'a StockList { self.stocks }

    #[inline(always)]
    pub fn datastore(&self) -> Option<&'b DataStore> { self.ds }

    #[inline(always)]
    pub fn groupby(&self) -> bool { self.groupby }
}

pub fn print_report(params: ReportParams) {
    match params.rtype() {
        ReportType::Value => value_report(&params),
        ReportType::Top => top_report(&params),
        ReportType::Volat => volat_report(&params)
    }
}

pub fn export_report(params: ReportParams, filename: &str) -> Result<(), Box<dyn Error>> {
    match params.rtype() {
        ReportType::Value => value_export(&params, filename),
        ReportType::Top => top_export(&params, filename),
        ReportType::Volat => volat_export(&params, filename)
    }
}

// --------------------------------------------------------------------------------
// Portfolio Value (Gain & Loss) Report and Export

fn value_report(params: &ReportParams) {
    let stocks = params.stocks();
    let groupby = params.groupby();

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

fn value_export(params: &ReportParams, filename: &str) -> Result<(), Box<dyn Error>> {
    let stocks = params.stocks();
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

const PCT_CHG: &str = "Total Pct Change";
const NET_CHG: &str = "Total Net Change";
const CUM_DIV: &str = "Total Cum Div";
const PCT_CHG_DAY: &str = "Daily Pct Change";
const NET_CHG_DAY: &str = "Daily Net Change";
const CUM_DIV_DAY: &str = "Daily Cum Div";

type TopTuple<'a> = (&'a str, Price, Price, Price, Price, Price, Price);
type TopBottom<'a> = (&'a str, &'a str);

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

fn calc_top_bottom<'a>(data: &'a mut Vec<TopTuple>, ftn: fn(&TopTuple) -> Price) -> TopBottom<'a> {
    data.sort_by(|lhs, rhs| price_type::price_cmp(ftn(lhs), ftn(rhs)));
    (data[data.len() - 1].0, data[0].0)
}

fn tb_pct_chg<'a>(data: &'a mut Vec<TopTuple>) -> TopBottom<'a> { calc_top_bottom(data, |t| t.1) }
fn tb_net_chg<'a>(data: &'a mut Vec<TopTuple>) -> TopBottom<'a> { calc_top_bottom(data, |t| t.2) }
fn tb_cum_div<'a>(data: &'a mut Vec<TopTuple>) -> TopBottom<'a> { calc_top_bottom(data, |t| t.3) }
fn tb_pct_chg_day<'a>(data: &'a mut Vec<TopTuple>) -> TopBottom<'a> { calc_top_bottom(data, |t| t.4) }
fn tb_net_chg_day<'a>(data: &'a mut Vec<TopTuple>) -> TopBottom<'a> { calc_top_bottom(data, |t| t.5) }
fn tb_cum_div_day<'a>(data: &'a mut Vec<TopTuple>) -> TopBottom<'a> { calc_top_bottom(data, |t| t.6) }

fn top_report(params: &ReportParams) {
    fn print_row(name: &str, top_bottom: &TopBottom) {
        println!("{:18} {:8} {:8}", name, (*top_bottom).0, (*top_bottom).1);
    }

    let stocks = params.stocks();

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
        print_row(PCT_CHG, &tb_pct_chg(&mut data));
        print_row(NET_CHG, &tb_net_chg(&mut data));
        print_row(CUM_DIV, &tb_cum_div(&mut data));
        print_row(PCT_CHG_DAY, &tb_pct_chg_day(&mut data));
        print_row(NET_CHG_DAY, &tb_net_chg_day(&mut data));
        print_row(CUM_DIV_DAY, &tb_cum_div_day(&mut data));
    }
}

fn top_export(params: &ReportParams, filename: &str) -> Result<(), Box<dyn Error>> {
    fn write_row(file: &mut File, name: &str, top_bottom: &TopBottom) -> Result<(), Box<dyn Error>> {
        write!(file, "{},{},{}\n", name, (*top_bottom).0, (*top_bottom).1)?;
        Ok(())
    }

    let stocks = params.stocks();
    let mut file = File::create(&filename)?;
    write!(file, "Category,Top,Bottom\n")?;

    let mut data: Vec<TopTuple> = stocks.iter().map(make_top_tuple).collect();
    if data.len() > 0 {
        write_row(&mut file, PCT_CHG, &tb_pct_chg(&mut data))?;
        write_row(&mut file, NET_CHG, &tb_net_chg(&mut data))?;
        write_row(&mut file, CUM_DIV, &tb_cum_div(&mut data))?;
        write_row(&mut file, PCT_CHG_DAY, &tb_pct_chg_day(&mut data))?;
        write_row(&mut file, NET_CHG_DAY, &tb_net_chg_day(&mut data))?;
        write_row(&mut file, CUM_DIV_DAY, &tb_cum_div_day(&mut data))?;
    }
    Ok(())
}

// --------------------------------------------------------------------------------
// Stocks Volatility Report and Export

const VOLAT_WIN: usize = 22;

fn calc_volat(stock: &Stock, ds: &DataStore) -> Price {
    if let Ok(hist) = History::ds_select_if(ds, &stock.symbol, |e| e.date >= stock.date) {
        if let Ok(volat) = hist_ftns::hist_volatility(&hist) {
            return volat
        }
    }
    return 0.0
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
    return 0.0
}

fn volat_report(params: &ReportParams) {
    let stocks = params.stocks();
    let ds = params.datastore().expect("Volat report missing datastore");

    println!("Stocks Volatility Report");
    println!("------------------------");
    println!("            Date: {}", datetime::today().format("%Y-%m-%d"));
    println!("Number of Stocks: {}", stocks.len());
    println!("");

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
             "-------",);

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

fn volat_export(params: &ReportParams, filename: &str) -> Result<(), Box<dyn Error>> {
    let stocks = params.stocks();
    let ds = params.datastore().expect("Volat export missing datastore");

    let mut file = File::create(&filename)?;
    write!(file, "Symbol,Buy Date,Upd Date,Days Held,Volat,Volat22\n")?;
    for stock in stocks.iter() {
        write!(file, "{},{},{},{},{:.2},{:.2}\n",
               stock.symbol,
               stock.date.format("%Y-%m-%d"),
               stock.latest_date.format("%Y-%m-%d"),
               stock.days_held,
               calc_volat(stock, ds),
               calc_volat22(stock, ds))?;
    }
    Ok(())
}
