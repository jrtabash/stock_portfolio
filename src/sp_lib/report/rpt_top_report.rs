use std::fs::File;
use std::io::prelude::*;

use crate::portfolio::stock::{Price, Stock};
use crate::report::report_params::ReportParams;
use crate::report::report_trait::Report;
use crate::util::{datetime, price_type};
use crate::util::error::Error;

pub struct TopReport {}

impl Report for TopReport {
    fn write(&self, params: &ReportParams) {
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

    fn export(&self, params: &ReportParams, filename: &str) -> Result<(), Error> {
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
}

// --------------------------------------------------------------------------------
// Private

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
