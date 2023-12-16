use std::io::prelude::*;
use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::iter::zip;

use crate::util::error::Error;
use crate::util::{price_type, datetime};
use crate::datastore::datastore::DataStore;
use crate::datastore::history::History;
use crate::stats::hist_ftns;
use crate::portfolio::closed_position::ClosedPositionList;
use crate::portfolio::stock::{Price, Stock, StockList};
use crate::portfolio::stock_type::StockType;
use crate::portfolio::stocks_config::StocksConfig;
use crate::portfolio::algorithms;
use crate::portfolio::report_type::ReportType;

pub struct ReportParams<'a, 'b> {
    rtype: ReportType,
    config: &'a StocksConfig,
    ds: Option<&'b DataStore>,
    groupby: bool
}

impl<'a, 'b> ReportParams<'a, 'b> {
    pub fn new(rtype: ReportType, config: &'a StocksConfig) -> Self {
        ReportParams {
            rtype,
            config,
            ds: None,
            groupby: false
        }
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
    pub fn config(&self) -> &'a StocksConfig { &self.config }

    #[inline(always)]
    pub fn stocks(&self) -> &'a StockList { self.config.stocks() }

    #[inline(always)]
    pub fn closed_positions(&self) -> &'a ClosedPositionList { self.config.closed_positions() }

    #[inline(always)]
    pub fn datastore(&self) -> Option<&'b DataStore> { self.ds }

    #[inline(always)]
    pub fn groupby(&self) -> bool { self.groupby }
}

pub fn print_report(params: ReportParams) {
    match params.rtype() {
        ReportType::Value => value_report(&params),
        ReportType::Top => top_report(&params),
        ReportType::Volat => volat_report(&params),
        ReportType::Daych => daych_report(&params),
        ReportType::Closed => closed_report(&params),
        ReportType::Divid => divid_report(&params)
    }
}

pub fn export_report(params: ReportParams, filename: &str) -> Result<(), Error> {
    match params.rtype() {
        ReportType::Value => value_export(&params, filename),
        ReportType::Top => top_export(&params, filename),
        ReportType::Volat => volat_export(&params, filename),
        ReportType::Daych => daych_export(&params, filename),
        ReportType::Closed => closed_export(&params, filename),
        ReportType::Divid => divid_export(&params, filename)
    }
}

// --------------------------------------------------------------------------------
// Portfolio Value (Gain & Loss) Report and Export

fn value_report(params: &ReportParams) {
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

fn value_export(params: &ReportParams, filename: &str) -> Result<(), Error> {
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

// --------------------------------------------------------------------------------
// Top/Bottom Performing Stocks Report and Export

const PCT_CHG: &str = "Total Pct Change";
const NET_CHG: &str = "Total Net Change";
const CUM_DIV: &str = "Total Cum Div";
const PCT_CHG_DAY: &str = "Daily Pct Change";
const NET_CHG_DAY: &str = "Daily Net Change";
const CUM_DIV_DAY: &str = "Daily Cum Div";
const DU_DIV_DAY: &str = "Daily Unt Div";

type TopTuple<'a> = (&'a str, Price, Price, Price, Price, Price, Price, Price);
type TopBottom<'a> = (&'a str, &'a str);

fn make_top_tuple(stock: &Stock) -> TopTuple {
    (stock.symbol.as_str(),
     stock.pct_change(),
     stock.net_price(),
     stock.cum_dividend,
     price_type::calc_daily(stock.pct_change(), stock.days_held),
     price_type::calc_daily(stock.net_price(), stock.days_held),
     price_type::calc_daily(stock.cum_dividend, stock.days_held),
     stock.daily_unit_dividend()
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
fn tb_daily_unit_div<'a>(data: &'a mut Vec<TopTuple>) -> TopBottom<'a> { calc_top_bottom(data, |t| t.7) }

fn top_report(params: &ReportParams) {
    fn print_row(name: &str, top_bottom: &TopBottom) {
        println!("{:18} {:8} {:8}", name, (top_bottom).0, (top_bottom).1);
    }

    let stocks = params.stocks();

    println!("Stocks Top/Bottom Performing Report");
    println!("-----------------------------------");
    println!("            Date: {}", datetime::today().format("%Y-%m-%d"));
    println!("Number of Stocks: {}", stocks.len());
    println!();

    println!("{:18} {:8} {:8}",
             "Category",
             "Top",
             "Bottom");
    println!("{:18} {:8} {:8}",
             "--------",
             "---",
             "------");

    let mut data: Vec<TopTuple> = stocks.iter().map(make_top_tuple).collect();
    if !data.is_empty() {
        print_row(PCT_CHG, &tb_pct_chg(&mut data));
        print_row(NET_CHG, &tb_net_chg(&mut data));
        print_row(CUM_DIV, &tb_cum_div(&mut data));
        print_row(PCT_CHG_DAY, &tb_pct_chg_day(&mut data));
        print_row(NET_CHG_DAY, &tb_net_chg_day(&mut data));
        print_row(CUM_DIV_DAY, &tb_cum_div_day(&mut data));
        print_row(DU_DIV_DAY, &tb_daily_unit_div(&mut data));
    }
}

fn top_export(params: &ReportParams, filename: &str) -> Result<(), Error> {
    fn write_row(file: &mut File, name: &str, top_bottom: &TopBottom) -> Result<(), Error> {
        writeln!(file, "{},{},{}", name, (top_bottom).0, (top_bottom).1)?;
        Ok(())
    }

    let stocks = params.stocks();
    let mut file = File::create(filename)?;
    writeln!(file, "Category,Top,Bottom")?;

    let mut data: Vec<TopTuple> = stocks.iter().map(make_top_tuple).collect();
    if !data.is_empty() {
        write_row(&mut file, PCT_CHG, &tb_pct_chg(&mut data))?;
        write_row(&mut file, NET_CHG, &tb_net_chg(&mut data))?;
        write_row(&mut file, CUM_DIV, &tb_cum_div(&mut data))?;
        write_row(&mut file, PCT_CHG_DAY, &tb_pct_chg_day(&mut data))?;
        write_row(&mut file, NET_CHG_DAY, &tb_net_chg_day(&mut data))?;
        write_row(&mut file, CUM_DIV_DAY, &tb_cum_div_day(&mut data))?;
        write_row(&mut file, DU_DIV_DAY, &tb_daily_unit_div(&mut data))?;
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

fn volat_report(params: &ReportParams) {
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

fn volat_export(params: &ReportParams, filename: &str) -> Result<(), Error> {
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

// --------------------------------------------------------------------------------
// Stocks Day Change Report and Export

struct DayChange {
    prev_price: Price,
    price: Price,
    change: Price,
    pct_change: Price,
    val_change: Price,
    low: Price,
    high: Price,
    volume: u64
}

type DayChangeList = Vec<Option<DayChange>>;
type AggValChanges = HashMap<String, Price>;

fn calc_daych(stock: &Stock, ds: &DataStore) -> Option<DayChange> {
    if let Ok(hist) = History::ds_select_last_n(ds, &stock.symbol, 2) {
        let entries = hist.entries();
        if entries.len() == 2 {
            let prev_price = entries[0].adj_close;
            let delta = entries[1].adj_close - prev_price;
            return Some(DayChange {
                prev_price,
                price: entries[1].adj_close,
                change: delta,
                pct_change: 100.0 * if prev_price > 0.0 { delta / prev_price } else { 0.00 },
                val_change: if stock.stype != StockType::Index { stock.quantity as Price * delta } else { 0.0 },
                low: entries[1].low,
                high: entries[1].high,
                volume: entries[1].volume
            });
        }
    }
    None
}

fn calc_agg_value_changes(stocks: &StockList, changes: &DayChangeList) -> AggValChanges {
    let mut agg_value_changes: AggValChanges = AggValChanges::new();
    for (stock, change) in zip(stocks, changes) {
        if let Some(chg) = change {
            let entry = agg_value_changes.entry(stock.symbol.to_string()).or_insert(0.00);
            *entry += chg.val_change;
        }
    }
    agg_value_changes
}

fn daych_report(params: &ReportParams) {
    let stocks = params.stocks();
    let ds = params.datastore().expect("Daych report missing datastore");
    let changes: DayChangeList = stocks
        .iter()
        .map(|s| calc_daych(s, ds))
        .collect();
    let value_change: Price = changes
        .iter()
        .filter(|c| c.is_some())
        .map(|c| c.as_ref().unwrap().val_change)
        .sum::<Price>();

    println!("Stocks Day Change Report");
    println!("------------------------");
    println!("              Date: {}", datetime::today().format("%Y-%m-%d"));
    println!("  Number of Stocks: {}", stocks.len());
    println!("Total Value Change: {:0.2}", value_change);
    println!();

    println!("{:8} {:10} {:8} {:8} {:8} {:8} {:8} {:8} {:8} {:10}",
             "Symbol",
             "Upd Date",
             "Prev Pr",
             "Price",
             "Change",
             "Pct Chg",
             "Val Chg",
             "Low",
             "High",
             "Volume");

    println!("{:8} {:10} {:8} {:8} {:8} {:8} {:8} {:8} {:8} {:10}",
             "------",
             "--------",
             "-------",
             "-----",
             "------",
             "-------",
             "-------",
             "---",
             "----",
             "------");

    let agg_value_changes = calc_agg_value_changes(stocks, &changes);
    let mut seen = HashSet::new();
    for (stock, change) in zip(stocks, &changes) {
        if seen.contains(&stock.symbol) { continue; }

        if let Some(chg) = change {
            seen.insert(&stock.symbol);
            println!("{:8} {:10} {:8.2} {:8.2} {:8.2} {:8.2} {:8.2} {:8.2} {:8.2} {:10}",
                     stock.symbol,
                     stock.latest_date.format("%Y-%m-%d"),
                     chg.prev_price,
                     chg.price,
                     chg.change,
                     chg.pct_change,
                     agg_value_changes.get(&stock.symbol).unwrap_or(&0.0),
                     chg.low,
                     chg.high,
                     chg.volume);
        }
    }
}

fn daych_export(params: &ReportParams, filename: &str) -> Result<(), Error> {
    let stocks = params.stocks();
    let ds = params.datastore().expect("Daych report missing datastore");

    let mut file = File::create(filename)?;
    writeln!(file, "Symbol,Upd Date,Prev Pr,Price,Change,Pct Chg,Val Chg,Low,High,Volume")?;

    let changes: DayChangeList = stocks
        .iter()
        .map(|s| calc_daych(s, ds))
        .collect();
    let agg_value_changes = calc_agg_value_changes(stocks, &changes);

    let mut seen = HashSet::new();
    for (stock, change) in zip(stocks, &changes) {
        if seen.contains(&stock.symbol) { continue; }

        if let Some(chg) = change {
            seen.insert(&stock.symbol);
            writeln!(file, "{},{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{}",
                     stock.symbol,
                     stock.latest_date.format("%Y-%m-%d"),
                     chg.prev_price,
                     chg.price,
                     chg.change,
                     chg.pct_change,
                     agg_value_changes.get(&stock.symbol).unwrap_or(&0.0),
                     chg.low,
                     chg.high,
                     chg.volume)?;
        }
    }
    Ok(())
}

// --------------------------------------------------------------------------------
// Closed Positions Value Report and Export

fn closed_report(params: &ReportParams) {
    let positions = params.closed_positions();

    let mut base_ntnl: Price = 0.0;
    let mut exit_ntnl: Price = 0.0;
    let mut net_ntnl: Price = 0.0;
    let mut tot_fees: Price = 0.0;
    let mut tot_div: Price = 0.0;

    for pos in positions.iter() {
        base_ntnl += pos.base_notional();
        exit_ntnl += pos.exit_notional();
        net_ntnl += pos.net_notional();
        tot_fees += pos.base_fee + pos.exit_fee;
        tot_div += pos.dividend;
    }

    println!("Closed Positions Report");
    println!("-----------------------");
    println!("            Date: {}", datetime::today().format("%Y-%m-%d"));
    println!("Total Base Value: {:.2}", base_ntnl);
    println!("Total Exit Value: {:.2}", exit_ntnl);
    println!(" Total Net Value: {:.2}", net_ntnl);
    println!("      Total Fees: {:.2}", tot_fees);
    println!("  Total Dividend: {:.2}", tot_div);
    println!("Net + Div - Fees: {:.2}", net_ntnl + tot_div - tot_fees);
    println!();

    println!("{:8} {:10} {:10} {:12} {:12} {:12} {:6} {:10}",
             "Symbol",
             "Base Date",
             "Exit Date",
             "Base Value",
             "Exit Value",
             "Net Value",
             "Fees",
             "Dividend");
    println!("{:8} {:10} {:10} {:12} {:12} {:12} {:6} {:10}",
             "------",
             "---------",
             "---------",
             "----------",
             "----------",
             "---------",
             "----",
             "--------");

    for pos in positions.iter() {
        println!("{:8} {:10} {:10} {:12.2} {:12.2} {:12.2} {:6.2} {:10.2}",
                 pos.symbol,
                 pos.base_date.format("%Y-%m-%d"),
                 pos.exit_date.format("%Y-%m-%d"),
                 pos.base_notional(),
                 pos.exit_notional(),
                 pos.net_notional(),
                 pos.base_fee + pos.exit_fee,
                 pos.dividend);
    }
}

fn closed_export(params: &ReportParams, filename: &str) -> Result<(), Error> {
    let positions = params.closed_positions();

    let mut file = File::create(filename)?;
    writeln!(file, "Symbol,Base Date,Exit Date,Base Value,Exit Value,Net Value,Fees,Dividend")?;

    for pos in positions.iter() {
        writeln!(file, "{},{},{},{:.2},{:.2},{:.2},{:.2},{:.2}",
                 pos.symbol,
                 pos.base_date.format("%Y-%m-%d"),
                 pos.exit_date.format("%Y-%m-%d"),
                 pos.base_notional(),
                 pos.exit_notional(),
                 pos.net_notional(),
                 pos.base_fee + pos.exit_fee,
                 pos.dividend)?;
    }
    Ok(())
}

// --------------------------------------------------------------------------------
// Dividend Report and Export

fn divid_report(params: &ReportParams) {
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

fn divid_export(params: &ReportParams, filename: &str) -> Result<(), Error> {
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
