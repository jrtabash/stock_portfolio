use std::io::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs;
use stock_portfolio::sputil::datetime::*;
use stock_portfolio::portfolio::stock_type::*;
use stock_portfolio::portfolio::stock::*;
use stock_portfolio::portfolio::algorithms::*;
use stock_portfolio::portfolio::stocks_update::*;
use stock_portfolio::portfolio::stocks_reader::*;
use stock_portfolio::portfolio::reports::value_export;

#[test]
fn test_stock_list() {
    let mut list = StockList::new();
    list.push(make_stock("AAPL", StockType::Stock, today_plus_days(-3), 100, 120.25, 125.25));
    list.push(make_stock("DELL", StockType::Stock, today_plus_days(-2), 100, 79.21, 79.71));
    assert_eq!(list.len(), 2);
    assert!(price_equal(net_notional(&list), 550.0));
    assert!(price_equal(latest_notional(&list), 20496.0));
    assert!(price_equal(base_notional(&list), 19946.0));
    assert!(price_equal(pct_change(&list), 2.76));

    let total_size: u32 = list.iter().map(|stock| stock.quantity).sum();
    assert_eq!(total_size, 200);
}

#[test]
fn test_stock_groupby() {
    fn test(groupby: &HashMap<String, (u32, Price)>, symbol: &str, size: u32, price: Price) {
        let size_price = groupby.get(symbol).unwrap();
        assert_eq!(size_price.0, size);
        assert!(price_equal(size_price.1, price));
    }

    let mut list = StockList::new();
    list.push(make_stock("AAPL", StockType::Stock, today_plus_days(-3), 100, 120.25, 125.25));
    list.push(make_stock("DELL", StockType::Stock, today_plus_days(-2), 100, 79.21, 79.71));
    list.push(make_stock("AAPL", StockType::Stock, today_plus_days(-2), 100, 122.0, 125.25));

    let gby = stock_groupby(&list);
    assert_eq!(gby.len(), 2);
    test(&gby, "AAPL", 200, 25050.0);
    test(&gby, "DELL", 100, 7971.0);
}

#[test]
fn test_stock_update_from_csv() {
    let csv = "Date,Open,High,Low,Close,Adj Close,Volume\n\
               2021-02-26,24.90,32.0,24.0,28.0,28.25,11000";
    let mut stock = Stock::new(String::from("STCK"), StockType::Stock, make_date(2021, 2, 1), 100, 24.0);
    assert!(update_stock_from_csv(&mut stock, &csv).unwrap());
    assert!(price_equal(stock.latest_price, 28.25));
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
    assert!(price_equal(stock.latest_price, 28.25));
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
    assert!(price_equal(stock.latest_price, 0.00));
    assert_eq!(stock.latest_date, earliest_date());
}

#[test]
fn test_stock_update_from_csv_no_data() {
    let csv = "Date,Open,High,Low,Close,Adj Close,Volume";
    let mut stock = Stock::new(String::from("STCK"), StockType::Stock, make_date(2021, 2, 1), 100, 24.0);
    assert!(!update_stock_from_csv(&mut stock, &csv).unwrap());
    assert!(price_equal(stock.latest_price, 0.00));
    assert_eq!(stock.latest_date, earliest_date());
}

#[test]
fn test_stock_update_from_csv_incomplete_data() {
    let csv = "Date,Open,High,Low,Close,Adj Close,Volume\n\
               2021-02-24,25.0,30.0";
    let mut stock = Stock::new(String::from("STCK"), StockType::Stock, make_date(2021, 2, 1), 100, 24.0);
    assert!(update_stock_from_csv(&mut stock, &csv).is_err());
    assert!(price_equal(stock.latest_price, 0.00));
    assert_eq!(stock.latest_date, earliest_date());
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
}

#[test]
fn test_filter_stocks() {
    fn test_filter(expr: &str, keep: bool, symbols: &Vec<&str>) {
        let mut list = StockList::new();
        list.push(make_stock("DELL", StockType::Stock, today_plus_days(-2), 100, 79.21, 79.71));
        list.push(make_stock("AAPL", StockType::Stock, today_plus_days(-3), 200, 120.25, 125.25));
        list.push(make_stock("ICLN", StockType::ETF, today_plus_days(0), 300, 24.10, 24.12));

        filter_stocks(&mut list, expr, keep);

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
fn test_value_export() {
    let mut list = StockList::new();
    list.push(make_stock("DELL", StockType::Stock, today_plus_days(-2), 100, 75.50, 80.0));
    list.push(make_stock("AAPL", StockType::Stock, today_plus_days(-3), 100, 120.25, 125.25));
    list.push(make_stock("ICLN", StockType::ETF, today_plus_days(0), 100, 24.10, 24.15));

    let csv_filename = make_temp_file("sp_test_value_export.csv");
    value_export(&list, &csv_filename).unwrap();

    let csv_content = fs::read_to_string(&csv_filename).unwrap();
    let today_str = today_plus_days(0).format("%Y-%m-%d");
    let expected = format!("Ticker,Buy Date,Upd Date,Size,Base,Cur,Net,Pct,Base Value,Cur Value,Net Value\n\
                            DELL,{},{},100,75.50,80.00,4.50,5.96,7550.00,8000.00,450.00\n\
                            AAPL,{},{},100,120.25,125.25,5.00,4.16,12025.00,12525.00,500.00\n\
                            ICLN,{},{},100,24.10,24.15,0.05,0.21,2410.00,2415.00,5.00\n",
                           today_plus_days(-2).format("%Y-%m-%d"),
                           today_str,
                           today_plus_days(-3).format("%Y-%m-%d"),
                           today_str,
                           today_str,
                           today_str);
    assert_eq!(csv_content, expected);

    fs::remove_file(&csv_filename).unwrap();
}

#[test]
fn test_stock_reader() {
    let stocks_filename = make_temp_file("sp_test_stocks_file.csv");
    let mut file = fs::File::create(stocks_filename.clone()).unwrap();
    write!(file, "symbol,type,date,quantity,base_price\n").unwrap();
    write!(file, "AAPL,stock,2020-09-20,100,115.00\n").unwrap();
    write!(file, "AAPL,stock,2020-11-12,100,118.50\n").unwrap();
    write!(file, "DELL,stock,2021-02-10,100,75.50\n").unwrap();

    let reader = StocksReader::new(stocks_filename.clone());
    let list = reader.read().unwrap();
    assert_eq!(list.iter().map(|s| s.symbol.as_str()).collect::<Vec<&str>>(),
               vec!["AAPL", "AAPL", "DELL"]);
    assert_eq!(list.iter().map(|s| s.stype).collect::<Vec<StockType>>(),
               vec![StockType::Stock, StockType::Stock, StockType::Stock]);
    assert_eq!(list.iter().map(|s| s.date).collect::<Vec<LocalDate>>(),
               vec![make_date(2020, 9, 20), make_date(2020, 11, 12), make_date(2021, 02, 10)]);
    assert_eq!(list.iter().map(|s| s.quantity).collect::<Vec<u32>>(),
               vec![100, 100, 100]);
    assert_eq!(list.iter().map(|s| s.base_price).collect::<Vec<f64>>(),
               vec![115.0, 118.50, 75.50]);

    fs::remove_file(&stocks_filename).unwrap();
}

// --------------------------------------------------------------------------------
// Helpers

fn make_stock(sym: &str, stype: StockType, date: LocalDate, qty: u32, base: Price, latest: Price) -> Stock {
    let symbol = String::from(sym);
    let mut stock = Stock::new(symbol, stype, date, qty, base);
    stock.set_latest_price(latest, today_plus_days(0));
    stock
}

fn price_equal(lhs: Price, rhs: Price) -> bool {
    format!("{:.2}", lhs) == format!("{:.2}", rhs)
}

fn make_temp_file(filename: &str) -> String {
    let mut pbuf = env::temp_dir();
    pbuf.push(filename);
    format!("{}", pbuf.to_str().unwrap())
}
