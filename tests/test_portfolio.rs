#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::env;
    use std::fs;
    use stock_portfolio::sputil::datetime::*;
    use stock_portfolio::portfolio::stock_type::*;
    use stock_portfolio::portfolio::stock::*;
    use stock_portfolio::portfolio::algorithms::*;
    use stock_portfolio::portfolio::stocks_update::*;
    use stock_portfolio::portfolio::stocks_cache::*;
    use stock_portfolio::portfolio::reports::value_export;

    #[test]
    fn test_stock_type() {
        let stock = StockType::Stock;
        let etf = StockType::ETF;
        let stock_str = "stock";
        let etf_str = "etf";

        assert_eq!(stocktype2str(stock), stock_str);
        assert_eq!(stocktype2str(etf), etf_str);
        assert!(str2stocktype(&stock_str).unwrap() == stock);
        assert!(str2stocktype(&etf_str).unwrap() == etf);

        match str2stocktype("foobar") {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(format!("{}", err), "Unknown stock type 'foobar'")
        };
    }

    #[test]
    fn test_stock() {
        let stock = make_stock("AAPL", StockType::Stock, today(), 100, 120.25, 129.50);
        assert_eq!(stock.symbol, "AAPL");
        assert_eq!(stock.date, today());
        assert_eq!(stock.quantity, 100);
        assert!(price_equal(stock.base_price, 120.25));
        assert!(price_equal(stock.latest_price, 129.50));
        assert_eq!(stock.latest_date, today_plus_days(0));
        assert!(price_equal(stock.net_price(), 9.25));
        assert!(price_equal(stock.base_notional(), 12025.0));
        assert!(price_equal(stock.latest_notional(), 12950.0));
        assert!(price_equal(stock.net_notional(), 925.0));
        assert!(price_equal(stock.pct_change(), 7.69));
    }

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
            if let Some(size_price) = groupby.get(symbol) {
                if size_price.0 != size {
                    println!("Symbol {} size actual={} expected={}", symbol, size_price.0, size);
                    assert!(false);
                }
                if !price_equal(size_price.1, price) {
                    println!("Symbol {} price actual={:.2} expected={:.2}", symbol, size_price.1, price);
                    assert!(false);
                }
            }
            else {
                println!("Symbol {} not in groupby", symbol);
                assert!(false);
            }
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
        let mut stock = Stock::new(String::from("STCK"), StockType::Stock, parse_date("2021-02-01").unwrap(), 100, 24.0);
        assert!(update_stock_from_csv(&mut stock, &csv));
        assert!(price_equal(stock.latest_price, 28.25));
        assert_eq!(stock.latest_date, parse_date("2021-02-26").unwrap());
    }

    #[test]
    fn test_stock_update_from_csv2() {
        let csv = "Date,Open,High,Low,Close,Adj Close,Volume\n\
                   2021-02-24,25.0,30.0,20.0,26.0,26.0,10000\n\
                   2021-02-25,26.10,31.0,22.0,24.0,24.0,9000\n\
                   2021-02-26,24.90,32.0,24.0,28.0,28.25,11000";
        let mut stock = Stock::new(String::from("STCK"), StockType::Stock, parse_date("2021-02-01").unwrap(), 100, 24.0);
        assert!(update_stock_from_csv(&mut stock, &csv));
        assert!(price_equal(stock.latest_price, 28.25));
        assert_eq!(stock.latest_date, parse_date("2021-02-26").unwrap());
    }

    #[test]
    fn test_stock_cache_entry() {
        let mut cache_entry = CacheEntry::new(10.25, today());
        assert_eq!(cache_entry.latest_price, 10.25);
        assert_eq!(cache_entry.latest_date, today());

        let new_price = 20.52;
        let new_date = today_plus_days(1);

        cache_entry.update(new_price, &new_date);
        assert_eq!(cache_entry.latest_price, new_price);
        assert_eq!(cache_entry.latest_date, new_date);
    }

    #[test]
    fn test_stock_cache_entry_is_updated() {
        let thu = make_date(2021, 3, 18);
        let fri = make_date(2021, 3, 19);
        let sat = make_date(2021, 3, 20);
        let sun = make_date(2021, 3, 21);
        let mon = make_date(2021, 3, 22);

        let mut cache_entry = CacheEntry::new(10.25, thu);
        assert!(cache_entry.is_updated(&thu));
        assert!(!cache_entry.is_updated(&fri));

        cache_entry.latest_date = fri.clone();
        assert!(cache_entry.is_updated(&fri));
        assert!(cache_entry.is_updated(&sat));
        assert!(cache_entry.is_updated(&sun));
        assert!(!cache_entry.is_updated(&mon));

        cache_entry.latest_date = mon.clone();
        assert!(cache_entry.is_updated(&mon));
    }

    #[test]
    fn test_stocks_cache() {
        let mut cache = StocksCache::new();
        assert_eq!(cache.size(), 0);

        cache.add(String::from("AAPL"), CacheEntry::new(125.0, today()));
        assert_eq!(cache.size(), 1);

        match cache.get("AAPL") {
            Some(entry) => {
                assert_eq!(entry.latest_price, 125.0);
                assert_eq!(entry.latest_date, today());
            },
            None => { assert!(false); }
        }

        cache.add(String::from("DELL"), CacheEntry::new(80.0, today_plus_days(-1)));
        assert_eq!(cache.size(), 2);

        match cache.get_mut("DELL") {
            Some(entry) => {
                entry.latest_price = 81.0;
                entry.latest_date = today();
            },
            None => { assert!(false); }
        }

        match cache.get("DELL") {
            Some(entry) => {
                assert_eq!(entry.latest_price, 81.0);
                assert_eq!(entry.latest_date, today());
            },
            None => { assert!(false); }
        }
    }

    #[test]
    fn test_stocks_cache_from_csv() {
        fn test_cache_entry(entry: Option<&CacheEntry>, price: Price, date: &LocalDate) -> bool {
            match entry {
                Some(ce) => price_equal(ce.latest_price, price) && ce.latest_date == *date,
                None => false
            }
        }

        let csv_data = "AAPL,2021-02-26,125.0\n\
                        DELL,2021-02-26,80.0\n";

        match StocksCache::from_csv(&csv_data) {
            Ok(cache) => {
                let date = parse_date("2021-02-26").unwrap();
                assert_eq!(cache.size(), 2);
                assert!(test_cache_entry(cache.get("AAPL"), 125.0, &date));
                assert!(test_cache_entry(cache.get("DELL"), 80.0, &date));
            },
            Err(e) => {
                println!("{}", e);
                assert!(false);
            }
        }

        match StocksCache::from_csv("bad csv data") {
            Ok(_) => { assert!(false); },
            Err(_) => {}
        }
    }

    #[test]
    fn test_stocks_cache_from_cache_file() {
        match StocksCache::from_cache_file() {
            Ok(_) => {},
            Err(e) => {
                println!("{}", e);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_sort_stocks() {
        fn test_sort(stocks: &mut StockList, field: &str, desc: bool, first: &str, second: &str, third: &str) {
            if let Err(e) = sort_stocks(stocks, field, desc) {
                println!("{}", e);
                assert!(false);
            }
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
}
