#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use chrono::{Date, Local};
    use stock_portfolio::sputil::datetime::*;
    use stock_portfolio::portfolio::stock::*;
    use stock_portfolio::portfolio::algorithms::*;
    use stock_portfolio::portfolio::stocks_update::*;
    use stock_portfolio::portfolio::stocks_cache::*;

    #[test]
    fn test_stock() {
        let stock = make_stock("AAPL", Local::today(), 100, 120.25, 129.50);
        assert_eq!(stock.symbol, "AAPL");
        assert_eq!(stock.date, Local::today());
        assert_eq!(stock.quantity, 100);
        assert!(price_equal(stock.base_price, 120.25));
        assert!(price_equal(stock.latest_price, 129.50));
        assert_eq!(stock.latest_date, today_plus_days(-1));
        assert!(price_equal(stock.net_price(), 9.25));
        assert!(price_equal(stock.base_notional(), 12025.0));
        assert!(price_equal(stock.latest_notional(), 12950.0));
        assert!(price_equal(stock.net_notional(), 925.0));
    }

    #[test]
    fn test_stock_list() {
        let mut list = StockList::new();
        list.push(make_stock("AAPL", today_plus_days(-3), 100, 120.25, 125.25));
        list.push(make_stock("DELL", today_plus_days(-2), 100, 79.21, 79.71));
        assert_eq!(list.len(), 2);
        assert!(price_equal(net_notional(&list), 550.0));
        assert!(price_equal(latest_notional(&list), 20496.0));
        assert!(price_equal(base_notional(&list), 19946.0));

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
        list.push(make_stock("AAPL", today_plus_days(-3), 100, 120.25, 125.25));
        list.push(make_stock("DELL", today_plus_days(-2), 100, 79.21, 79.71));
        list.push(make_stock("AAPL", today_plus_days(-2), 100, 122.0, 125.25));

        let gby = stock_groupby(&list);
        assert_eq!(gby.len(), 2);
        test(&gby, "AAPL", 200, 25050.0);
        test(&gby, "DELL", 100, 7971.0);
    }

    #[test]
    fn test_stock_update_from_csv() {
        let csv = "Date,Open,High,Low,Close,Adj Close,Volume\n\
                   2021-02-26,24.90,32.0,24.0,28.0,28.25,11000";
        let mut stock = Stock::new(String::from("STCK"), parse_date("2021-02-01").unwrap(), 100, 24.0);
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
        let mut stock = Stock::new(String::from("STCK"), parse_date("2021-02-01").unwrap(), 100, 24.0);
        assert!(update_stock_from_csv(&mut stock, &csv));
        assert!(price_equal(stock.latest_price, 28.25));
        assert_eq!(stock.latest_date, parse_date("2021-02-26").unwrap());
    }

    #[test]
    fn test_stock_cache_entry() {
        let cache_entry = CacheEntry::new(10.25, Local::today());
        assert_eq!(cache_entry.latest_price, 10.25);
        assert_eq!(cache_entry.latest_date, Local::today());
    }

    #[test]
    fn test_stocks_cache() {
        let mut cache = StocksCache::new();
        assert_eq!(cache.size(), 0);

        cache.add(String::from("AAPL"), CacheEntry::new(125.0, Local::today()));
        assert_eq!(cache.size(), 1);

        match cache.get("AAPL") {
            Some(entry) => {
                assert_eq!(entry.latest_price, 125.0);
                assert_eq!(entry.latest_date, Local::today());
            },
            None => { assert!(false); }
        }

        cache.add(String::from("DELL"), CacheEntry::new(80.0, today_plus_days(-1)));
        assert_eq!(cache.size(), 2);

        match cache.get_mut("DELL") {
            Some(entry) => {
                entry.latest_price = 81.0;
                entry.latest_date = Local::today();
            },
            None => { assert!(false); }
        }

        match cache.get("DELL") {
            Some(entry) => {
                assert_eq!(entry.latest_price, 81.0);
                assert_eq!(entry.latest_date, Local::today());
            },
            None => { assert!(false); }
        }
    }

    #[test]
    fn test_stocks_cache_from_csv() {
        fn test_cache_entry(entry: Option<&CacheEntry>, price: Price, date: &Date<Local>) -> bool {
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
        list.push(make_stock("DELL", today_plus_days(-2), 100, 79.21, 79.71));
        list.push(make_stock("AAPL", today_plus_days(-3), 100, 120.25, 125.25));
        list.push(make_stock("ICLN", today_plus_days(0), 100, 24.10, 24.12));

        let asc = false;
        let desc = true;

        test_sort(&mut list, "symbol", asc, "AAPL", "DELL", "ICLN");
        test_sort(&mut list, "symbol", desc, "ICLN", "DELL", "AAPL");

        test_sort(&mut list, "date", asc, "AAPL", "DELL", "ICLN");
        test_sort(&mut list, "date", desc, "ICLN", "DELL", "AAPL");

        test_sort(&mut list, "value", desc, "AAPL", "DELL", "ICLN");
        test_sort(&mut list, "value", asc, "ICLN", "DELL", "AAPL");
    }

    fn make_stock(sym: &str, date: Date<Local>, qty: u32, base: Price, latest: Price) -> Stock {
        let symbol = String::from(sym);
        let mut stock = Stock::new(symbol, date, qty, base);
        stock.set_latest_price(latest, today_plus_days(-1));
        stock
    }

    fn price_equal(lhs: Price, rhs: Price) -> bool {
        format!("{:.2}", lhs) == format!("{:.2}", rhs)
    }
}
