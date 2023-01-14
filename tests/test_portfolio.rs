use std::collections::HashMap;
use std::fs;
use std::env;
use sp_lib::util::datetime::*;
use sp_lib::util::temp_file;
use sp_lib::util::price_type::price_eql;
use sp_lib::portfolio::stock_type::*;
use sp_lib::portfolio::stock::*;
use sp_lib::portfolio::algorithms::*;
use sp_lib::portfolio::stocks_update::*;
use sp_lib::portfolio::stocks_config::*;
use sp_lib::portfolio::stocks_reader::*;
use sp_lib::portfolio::report_type::ReportType;
use sp_lib::portfolio::reports;

#[test]
fn test_stock_list() {
    let mut list = StockList::new();
    list.push(make_stock("AAPL", StockType::Stock, today_plus_days(-3), 100, 120.25, 125.25));
    list.push(make_stock("DELL", StockType::Stock, today_plus_days(-2), 100, 79.21, 79.71));
    assert_eq!(list.len(), 2);
    assert!(price_eql(net_notional(&list), 550.0));
    assert!(price_eql(latest_notional(&list), 20496.0));
    assert!(price_eql(base_notional(&list), 19946.0));
    assert!(price_eql(pct_change(&list), 2.757445));

    list[0].cum_dividend = 100.25;
    list[1].cum_dividend = 10.50;
    assert!(price_eql(cumulative_dividend(&list), 110.75));

    let total_size: u32 = list.iter().map(|stock| stock.quantity).sum();
    assert_eq!(total_size, 200);
}

#[test]
fn test_stock_aggregate() {
    fn test(groupby: &HashMap<String, (u32, Price)>, symbol: &str, size: u32, price: Price) {
        let size_price = groupby.get(symbol).unwrap();
        assert_eq!(size_price.0, size);
        assert!(price_eql(size_price.1, price));
    }

    let mut list = StockList::new();
    list.push(make_stock("AAPL", StockType::Stock, today_plus_days(-3), 100, 120.25, 125.25));
    list.push(make_stock("DELL", StockType::Stock, today_plus_days(-2), 100, 79.21, 79.71));
    list.push(make_stock("AAPL", StockType::Stock, today_plus_days(-2), 100, 122.0, 125.25));

    let gby = stock_aggregate(&list);
    assert_eq!(gby.len(), 2);
    test(&gby, "AAPL", 200, 25050.0);
    test(&gby, "DELL", 100, 7971.0);
}

#[test]
fn test_stock_groupby() {
    let mut list = StockList::new();
    list.push(make_stock("DELL", StockType::Stock, today_plus_days(-2), 100, 79.21,  79.71));
    list.push(make_stock("AAPL", StockType::Stock, today_plus_days(-3), 200, 120.25, 125.25));
    list.push(make_stock("ICLN", StockType::ETF,   today_plus_days(0),  400, 24.10,  24.12));
    list.push(make_stock("AAPL", StockType::Stock, today_plus_days(0),  100, 125.50, 125.75));

    let sym_sizes = stock_groupby(&list, |_| 0, |s, q| s.quantity + q);
    assert_eq!(sym_sizes.len(), 3);
    assert_eq!(*sym_sizes.get("AAPL").unwrap(), 300);
    assert_eq!(*sym_sizes.get("DELL").unwrap(), 100);
    assert_eq!(*sym_sizes.get("ICLN").unwrap(), 400);
}

#[test]
fn test_stock_update_from_csv() {
    let csv = "Date,Open,High,Low,Close,Adj Close,Volume\n\
               2021-02-26,24.90,32.0,24.0,28.0,28.25,11000";
    let mut stock = Stock::new(String::from("STCK"), StockType::Stock, make_date(2021, 2, 1), 100, 24.0);
    assert!(update_stock_from_csv(&mut stock, &csv).unwrap());
    assert!(price_eql(stock.latest_price, 28.25));
    assert_eq!(stock.latest_date, make_date(2021, 2, 26));
}

#[test]
fn test_stock_update_from_csv2() {
    let csv = "Date,Open,High,Low,Close,Adj Close,Volume\n\
               2021-02-24,25.0,30.0,20.0,26.0,26.0,10000\n\
               2021-02-25,26.10,31.0,22.0,24.0,24.0,9000\n\
               2021-02-26,24.90,32.0,24.0,28.0,28.25,11000";
    let mut stock = Stock::new(String::from("STCK"), StockType::Stock, make_date(2021, 2, 1), 100, 24.0);
    assert!(update_stock_from_csv(&mut stock, &csv).unwrap());
    assert!(price_eql(stock.latest_price, 28.25));
    assert_eq!(stock.latest_date, make_date(2021, 2, 26));
}

#[test]
fn test_stock_update_from_csv_zero_price() {
    let csv = "Date,Open,High,Low,Close,Adj Close,Volume\n\
               2021-02-24,25.0,30.0,20.0,26.0,26.0,10000\n\
               2021-02-25,26.10,31.0,22.0,24.0,24.0,9000\n\
               2021-02-26,24.90,32.0,24.0,28.0,0.00,11000";
    let mut stock = Stock::new(String::from("STCK"), StockType::Stock, make_date(2021, 2, 1), 100, 24.0);
    assert!(!update_stock_from_csv(&mut stock, &csv).unwrap());
    assert!(price_eql(stock.latest_price, 0.00));
    assert_eq!(stock.latest_date, earliest_date());
}

#[test]
fn test_stock_update_from_csv_no_data() {
    let csv = "Date,Open,High,Low,Close,Adj Close,Volume";
    let mut stock = Stock::new(String::from("STCK"), StockType::Stock, make_date(2021, 2, 1), 100, 24.0);
    assert!(!update_stock_from_csv(&mut stock, &csv).unwrap());
    assert!(price_eql(stock.latest_price, 0.00));
    assert_eq!(stock.latest_date, earliest_date());
}

#[test]
fn test_stock_update_from_csv_incomplete_data() {
    let csv = "Date,Open,High,Low,Close,Adj Close,Volume\n\
               2021-02-24,25.0,30.0";
    let mut stock = Stock::new(String::from("STCK"), StockType::Stock, make_date(2021, 2, 1), 100, 24.0);
    assert!(update_stock_from_csv(&mut stock, &csv).is_err());
    assert!(price_eql(stock.latest_price, 0.00));
    assert_eq!(stock.latest_date, earliest_date());
}

#[test]
fn test_stocks_update() {
    let mut stocks = StockList::new();
    stocks.push(make_stock("DELL", StockType::Stock, today_plus_days(-2), 100, 52.21, 0.00));
    stocks.push(make_stock("AAPL", StockType::Stock, today_plus_days(-3), 200, 120.25, 0.00));

    let dt = make_date(2022, 02, 17);

    let cnt = update_stocks(&mut stocks, Some(dt)).unwrap();
    assert_eq!(cnt, 2);
    assert_eq!(stocks[0].latest_date, dt);
    assert_eq!(stocks[1].latest_date, dt);
    assert!((stocks[0].latest_price - 57.836052).abs() < 0.0001);
    assert!((stocks[1].latest_price - 168.119431).abs() < 0.0001);
}

#[test]
fn test_sort_stocks() {
    fn test_sort(stocks: &mut StockList, field: &str, desc: bool, first: &str, second: &str, third: &str) {
        sort_stocks(stocks, field, desc).unwrap();
        assert_eq!(&stocks[0].symbol, first);
        assert_eq!(&stocks[1].symbol, second);
        assert_eq!(&stocks[2].symbol, third);
    }

    let mut list = StockList::new();
    list.push(make_stock("DELL", StockType::Stock, today_plus_days(-2), 100, 79.21, 79.71));
    list.push(make_stock("AAPL", StockType::Stock, today_plus_days(-3), 200, 120.25, 125.25));
    list.push(make_stock("ICLN", StockType::ETF, today_plus_days(0), 300, 24.10, 24.12));
    list[0].cum_dividend = 0.0;
    list[1].cum_dividend = 20.15;
    list[2].cum_dividend = 15.25;

    let asc = false;
    let desc = true;

    test_sort(&mut list, "symbol", asc, "AAPL", "DELL", "ICLN");
    test_sort(&mut list, "symbol", desc, "ICLN", "DELL", "AAPL");

    test_sort(&mut list, "date", asc, "AAPL", "DELL", "ICLN");
    test_sort(&mut list, "date", desc, "ICLN", "DELL", "AAPL");

    test_sort(&mut list, "value", desc, "AAPL", "DELL", "ICLN");
    test_sort(&mut list, "value", asc, "ICLN", "DELL", "AAPL");

    test_sort(&mut list, "price", desc, "AAPL", "DELL", "ICLN");
    test_sort(&mut list, "price", asc, "ICLN", "DELL", "AAPL");

    test_sort(&mut list, "net", desc, "AAPL", "DELL", "ICLN");
    test_sort(&mut list, "net", asc, "ICLN", "DELL", "AAPL");

    test_sort(&mut list, "size", asc, "DELL", "AAPL", "ICLN");
    test_sort(&mut list, "size", desc, "ICLN", "AAPL", "DELL");

    test_sort(&mut list, "type", asc, "AAPL", "DELL", "ICLN");
    test_sort(&mut list, "type", desc, "ICLN", "AAPL", "DELL");

    test_sort(&mut list, "pct", desc, "AAPL", "DELL", "ICLN");
    test_sort(&mut list, "pct", asc, "ICLN", "DELL", "AAPL");

    test_sort(&mut list, "days", desc, "AAPL", "DELL", "ICLN");
    test_sort(&mut list, "days", asc, "ICLN", "DELL", "AAPL");

    test_sort(&mut list, "div", desc, "AAPL", "ICLN", "DELL");
    test_sort(&mut list, "div", asc, "DELL", "ICLN", "AAPL");
}

#[test]
fn test_sort_stocks_by_extra_ftn() {
    fn test_sort(stocks: &mut StockList, by_ftn: fn (&Stock) -> f64, desc: bool,
                 first: &str, second: &str, third: &str) {
        sort_stocks_by_extra_ftn(stocks, by_ftn, desc);
        assert_eq!(&stocks[0].symbol, first);
        assert_eq!(&stocks[1].symbol, second);
        assert_eq!(&stocks[2].symbol, third);
    }

    let mut list = StockList::new();
    list.push(make_stock("DELL", StockType::Stock, today_plus_days(-2), 100, 79.21, 79.71));
    list.push(make_stock("AAPL", StockType::Stock, today_plus_days(-3), 200, 120.25, 125.25));
    list.push(make_stock("ICLN", StockType::ETF, today_plus_days(0), 300, 24.10, 24.12));
    list[0].cum_dividend = 0.0;
    list[1].cum_dividend = 20.15;
    list[2].cum_dividend = 15.25;

    let asc = false;
    let desc = true;

    test_sort(&mut list, |s| s.cum_dividend as f64, asc, "DELL", "ICLN", "AAPL");
    test_sort(&mut list, |s| s.cum_dividend as f64, desc, "AAPL", "ICLN", "DELL");

    test_sort(&mut list, |s| (300 - s.quantity) as f64, asc, "ICLN", "AAPL", "DELL");
    test_sort(&mut list, |s| (300 - s.quantity) as f64, desc, "DELL", "AAPL", "ICLN");
}

#[test]
fn test_filter_stocks() {
    fn test_filter(expr: &str, keep: bool, symbols: &Vec<&str>) {
        let mut list = StockList::new();
        list.push(make_stock("DELL", StockType::Stock, today_plus_days(-2), 100, 79.21, 79.71));
        list.push(make_stock("AAPL", StockType::Stock, today_plus_days(-3), 200, 120.25, 125.25));
        list.push(make_stock("ICLN", StockType::ETF, today_plus_days(0), 300, 24.10, 24.12));

        filter_stocks(&mut list, expr, keep).unwrap();

        assert_eq!(list.len(), symbols.len());
        for i in 0..list.len() {
            assert_eq!(&list[i].symbol, symbols[i]);
        }
    }

    let keep = true;
    let remove = false;

    test_filter("etf", keep, &vec!["ICLN"]);
    test_filter("etf", remove, &vec!["DELL", "AAPL"]);
    test_filter("stock", keep, &vec!["DELL", "AAPL"]);
    test_filter("stock", remove, &vec!["ICLN"]);
    test_filter("AAPL", keep, &vec!["AAPL"]);
    test_filter("AAPL", remove, &vec!["DELL", "ICLN"]);
    test_filter("AAPL,DELL", keep, &vec!["DELL", "AAPL"]);
    test_filter("AAPL,DELL", remove, &vec!["ICLN"]);
    test_filter("MSFT", keep, &vec![]);
    test_filter("MSFT", remove, &vec!["DELL", "AAPL", "ICLN"]);
}

#[test]
fn test_filter_stocks_by_expr() {
    fn test_filter_by(by_expr: &str, keep: bool, sz: usize, sym1: &str, sym2: &str, sym3: &str) {
        let mut list = StockList::new();
        list.push(make_stock("DELL", StockType::Stock, today_plus_days(-2), 100, 79.21, 79.71));
        list.push(make_stock("AAPL", StockType::Stock, today_plus_days(-3), 200, 120.25, 125.25));
        list.push(make_stock("ICLN", StockType::ETF, today_plus_days(0), 300, 24.10, 24.12));

        filter_stocks(&mut list, by_expr, keep).unwrap();
        assert_eq!(list.len(), sz);
        if sz >= 1 { assert_eq!(&list[0].symbol, sym1); }
        if sz >= 2 { assert_eq!(&list[1].symbol, sym2); }
        if sz >= 3 { assert_eq!(&list[2].symbol, sym3); }
    }

    test_filter_by("days = 3",         true, 1, "AAPL", "", "");
    test_filter_by("days != 3",        true, 2, "DELL", "ICLN", "");
    test_filter_by("days < 3",         true, 2, "DELL", "ICLN", "");
    test_filter_by("days > 2",         true, 1, "AAPL", "", "");
    test_filter_by("days <= 2",        true, 2, "DELL", "ICLN", "");
    test_filter_by("days >= 3",        true, 1, "AAPL", "", "");
    test_filter_by("price = 79.71",    true, 1, "DELL", "", "");
    test_filter_by("price > 90.00",    true, 1, "AAPL", "", "");
    test_filter_by("net < 0.10",       true, 1, "ICLN", "", "");
    test_filter_by("pct > 3.0",        true, 1, "AAPL", "", "");
    test_filter_by("div = 0.00",       true, 3, "DELL", "AAPL", "ICLN");
    test_filter_by("size >= 200",      true, 2, "AAPL", "ICLN", "");
    test_filter_by("value <= 7500.00", true, 1, "ICLN", "", "");

    test_filter_by("days = 3",         false, 2, "DELL", "ICLN", "");
    test_filter_by("days != 3",        false, 1, "AAPL", "", "");
    test_filter_by("days < 3",         false, 1, "AAPL", "", "");
    test_filter_by("days > 2",         false, 2, "DELL", "ICLN", "");
    test_filter_by("days <= 2",        false, 1, "AAPL", "", "");
    test_filter_by("days >= 3",        false, 2, "DELL", "ICLN", "");
    test_filter_by("price = 79.71",    false, 2, "AAPL", "ICLN", "");
    test_filter_by("price > 90.00",    false, 2, "DELL", "ICLN", "");
    test_filter_by("net < 0.10",       false, 2, "DELL", "AAPL", "");
    test_filter_by("pct > 3.0",        false, 2, "DELL", "ICLN", "");
    test_filter_by("div = 0.00",       false, 0, "", "", "");
    test_filter_by("size >= 200",      false, 1, "DELL", "", "");
    test_filter_by("value <= 7500.00", false, 2, "DELL", "AAPL", "");
}

#[test]
fn test_stock_base_dates() {
    fn test_dates(list: &StockList) {
        let sym_dates = stock_base_dates(&list);
        assert_eq!(sym_dates.len(), 3);
        assert_eq!(*sym_dates.get("AAPL").unwrap(), today_plus_days(-3));
        assert_eq!(*sym_dates.get("DELL").unwrap(), today_plus_days(-2));
        assert_eq!(*sym_dates.get("ICLN").unwrap(), today_plus_days(0));
    }

    let mut list = StockList::new();
    list.push(make_stock("DELL", StockType::Stock, today_plus_days(-2), 100, 79.21,  79.71));
    list.push(make_stock("AAPL", StockType::Stock, today_plus_days(-3), 200, 120.25, 125.25));
    list.push(make_stock("ICLN", StockType::ETF,   today_plus_days(0),  300, 24.10,  24.12));
    list.push(make_stock("AAPL", StockType::Stock, today_plus_days(0),  100, 125.50, 125.75));
    test_dates(&list);

    let mut list = StockList::new();
    list.push(make_stock("DELL", StockType::Stock, today_plus_days(-2), 100, 79.21,  79.71));
    list.push(make_stock("AAPL", StockType::Stock, today_plus_days(0),  100, 125.50, 125.75));
    list.push(make_stock("ICLN", StockType::ETF,   today_plus_days(0),  300, 24.10,  24.12));
    list.push(make_stock("AAPL", StockType::Stock, today_plus_days(-3), 200, 120.25, 125.25));
    test_dates(&list);
}

#[test]
fn test_value_export() {
    let mut list = StockList::new();
    list.push(make_stock("DELL", StockType::Stock, today_plus_days(-2), 100, 75.50, 80.0));
    list.push(make_stock("AAPL", StockType::Stock, today_plus_days(-3), 100, 120.25, 125.25));
    list.push(make_stock("ICLN", StockType::ETF, today_plus_days(0), 100, 24.10, 24.15));

    let temp_name = "sp_test_value_export.csv";
    let csv_filename = temp_file::make_path(&temp_name);
    let rparams = reports::ReportParams::new(ReportType::Value, &list);
    reports::export_report(rparams, &csv_filename.to_str().unwrap()).unwrap();

    let csv_content = fs::read_to_string(&csv_filename).unwrap();
    let today_str = today_plus_days(0).format("%Y-%m-%d");
    let expected = format!("Symbol,Buy Date,Upd Date,Days Held,Size,Base,Cur,Net,Pct,Base Value,Cur Value,Net Value,Cum Div\n\
                            DELL,{},{},2,100,75.50,80.00,4.50,5.96,7550.00,8000.00,450.00,0.00\n\
                            AAPL,{},{},3,100,120.25,125.25,5.00,4.16,12025.00,12525.00,500.00,0.00\n\
                            ICLN,{},{},0,100,24.10,24.15,0.05,0.21,2410.00,2415.00,5.00,0.00\n",
                           today_plus_days(-2).format("%Y-%m-%d"),
                           today_str,
                           today_plus_days(-3).format("%Y-%m-%d"),
                           today_str,
                           today_str,
                           today_str);
    assert_eq!(csv_content, expected);

    assert!(temp_file::remove_file(&temp_name));
}

#[test]
fn test_stock_reader() {
    let temp_name = "sp_test_stocks_file.csv";
    let stocks_filename = temp_file::make_path(&temp_name);

    assert!(temp_file::create_file(&temp_name,
                                   "symbol,type,date,quantity,base_price\n\
                                    AAPL,stock,2020-09-20,100,115.00\n\
                                    AAPL,stock,2020-11-12,100,118.50\n\
                                    DELL,stock,2021-02-10,100,75.50\n"));

    let reader = StocksReader::new(String::from(stocks_filename.to_str().unwrap()));
    let list = reader.read().unwrap();
    assert_eq!(list.iter().map(|s| s.symbol.as_str()).collect::<Vec<&str>>(),
               vec!["AAPL", "AAPL", "DELL"]);
    assert_eq!(list.iter().map(|s| s.stype).collect::<Vec<StockType>>(),
               vec![StockType::Stock, StockType::Stock, StockType::Stock]);
    assert_eq!(list.iter().map(|s| s.date).collect::<Vec<SPDate>>(),
               vec![make_date(2020, 9, 20), make_date(2020, 11, 12), make_date(2021, 02, 10)]);
    assert_eq!(list.iter().map(|s| s.quantity).collect::<Vec<u32>>(),
               vec![100, 100, 100]);
    assert_eq!(list.iter().map(|s| s.base_price).collect::<Vec<f64>>(),
               vec![115.0, 118.50, 75.50]);

    assert!(temp_file::remove_file(&temp_name));
}

#[test]
fn test_stock_config_from_file() {
    let temp_name = "sp_test_stocks_config.cfg";
    let config_filename = temp_file::make_path(&temp_name);

    assert!(temp_file::create_file(&temp_name,
                                   "ds_root: sp_root\n\
                                    ds_name: sp_name\n\
                                    stocks: csv{\n\
                                    symbol,type,date,quantity,base_price\n\
                                    AAPL,stock,2020-09-20,100,115.00\n\
                                    AAPL,stock,2020-11-12,100,118.50\n\
                                    DELL,stock,2021-02-10,100,75.50\n\
                                    }\n"));

    let cfg = StocksConfig::from_file(config_filename.to_str().unwrap()).unwrap();
    assert_eq!(cfg.ds_root(), "sp_root");
    assert_eq!(cfg.ds_name(), "sp_name");
    assert_eq!(cfg.stocks().len(), 3);

    let list = cfg.stocks();
    assert_eq!(list.iter().map(|s| s.symbol.as_str()).collect::<Vec<&str>>(),
               vec!["AAPL", "AAPL", "DELL"]);
    assert_eq!(list.iter().map(|s| s.stype).collect::<Vec<StockType>>(),
               vec![StockType::Stock, StockType::Stock, StockType::Stock]);
    assert_eq!(list.iter().map(|s| s.date).collect::<Vec<SPDate>>(),
               vec![make_date(2020, 9, 20), make_date(2020, 11, 12), make_date(2021, 02, 10)]);
    assert_eq!(list.iter().map(|s| s.quantity).collect::<Vec<u32>>(),
               vec![100, 100, 100]);
    assert_eq!(list.iter().map(|s| s.base_price).collect::<Vec<f64>>(),
               vec![115.0, 118.50, 75.50]);

    assert!(temp_file::remove_file(&temp_name));
}

#[test]
fn test_stock_config_from_str() {
    let content: &str = "ds_root: sp_root\n\
                         ds_name: sp_name\n\
                         stocks: csv{\n\
                         symbol,type,date,quantity,base_price\n\
                         AAPL,stock,2020-09-20,100,115.00\n\
                         AAPL,stock,2020-11-12,100,118.50\n\
                         DELL,stock,2021-02-10,100,75.50\n\
                         }\n";

    let cfg = StocksConfig::from_str(content).unwrap();
    assert_eq!(cfg.ds_root(), "sp_root");
    assert_eq!(cfg.ds_name(), "sp_name");
    assert_eq!(cfg.stocks().len(), 3);

    let list = cfg.stocks();
    assert_eq!(list.iter().map(|s| s.symbol.as_str()).collect::<Vec<&str>>(),
               vec!["AAPL", "AAPL", "DELL"]);
    assert_eq!(list.iter().map(|s| s.stype).collect::<Vec<StockType>>(),
               vec![StockType::Stock, StockType::Stock, StockType::Stock]);
    assert_eq!(list.iter().map(|s| s.date).collect::<Vec<SPDate>>(),
               vec![make_date(2020, 9, 20), make_date(2020, 11, 12), make_date(2021, 02, 10)]);
    assert_eq!(list.iter().map(|s| s.quantity).collect::<Vec<u32>>(),
               vec![100, 100, 100]);
    assert_eq!(list.iter().map(|s| s.base_price).collect::<Vec<f64>>(),
               vec![115.0, 118.50, 75.50]);
}

#[test]
fn test_stock_config_mut() {
    let mut cfg = StocksConfig::new();
    assert_eq!(cfg.ds_root(), "");
    assert_eq!(cfg.ds_name(), "");
    assert_eq!(cfg.stocks().len(), 0);

    let stocks = cfg.stocks_mut();
    stocks.push(make_stock("AAPL", StockType::Stock, make_date(2020, 9, 20), 100, 120.25, 125.25));

    assert_eq!(cfg.ds_root(), "");
    assert_eq!(cfg.ds_name(), "");
    assert_eq!(cfg.stocks().len(), 1);

    let list = cfg.stocks();
    assert_eq!(list.iter().map(|s| s.symbol.as_str()).collect::<Vec<&str>>(), vec!["AAPL"]);
    assert_eq!(list.iter().map(|s| s.stype).collect::<Vec<StockType>>(), vec![StockType::Stock]);
    assert_eq!(list.iter().map(|s| s.date).collect::<Vec<SPDate>>(), vec![make_date(2020, 9, 20)]);
    assert_eq!(list.iter().map(|s| s.quantity).collect::<Vec<u32>>(), vec![100]);
    assert_eq!(list.iter().map(|s| s.base_price).collect::<Vec<f64>>(), vec![120.25]);
}

#[test]
fn test_stock_config_default() {
    fn check(c: &StocksConfig) {
        assert_eq!(c.ds_root(), env::var("HOME").unwrap());
        assert_eq!(c.ds_name(), "sp_datastore");
        assert_eq!(c.stocks().len(), 0);
    }

    let content: &str = "ds_root: $default\n\
                         ds_name: $default\n\
                         stocks: csv{\n\
                         }\n";
    let cfg = StocksConfig::from_str(content).unwrap();
    check(&cfg);

    let content: &str = "stocks: csv{\n\
                         }\n";
    let cfg = StocksConfig::from_str(content).unwrap();
    check(&cfg);
}

#[test]
fn test_stock_config_errors() {
    fn check(cfg: &str, err: &str) {
        match StocksConfig::from_str(cfg) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(err, format!("{}", e))
        };
    }

    fn cfg(csv: &str) -> String {
        let mut base = String::from("ds_root: $default\nds_name: $default\nstocks: ");
        base.push_str(csv);
        base
    }

    check(&cfg("csv:{\n}\n"), "StocksConfig::parse - Invalid line 'stocks: csv:{'");
    check(&cfg("csv[\n]\n"), "StocksConfig::parse - unsupported block type 'csv['");
}

// --------------------------------------------------------------------------------
// Helpers

fn make_stock(sym: &str, stype: StockType, date: SPDate, qty: u32, base: Price, latest: Price) -> Stock {
    let symbol = String::from(sym);
    let mut stock = Stock::new(symbol, stype, date, qty, base);
    stock.set_latest_price(latest, today_plus_days(0));
    stock
}
