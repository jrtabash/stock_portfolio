#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use chrono::{Date, Local};
    use crate::sputil::datetime::*;
    use crate::portfolio::stock::*;
    use crate::portfolio::algorithms::*;

    #[test]
    fn test_stock() {
        let stock = make_stock("AAPL", Local::today(), 100, 120.25, 129.50);
        assert_eq!(stock.symbol, "AAPL");
        assert_eq!(stock.date, Local::today());
        assert_eq!(stock.quantity, 100);
        assert!(price_equal(stock.base_price, 120.25));
        assert!(price_equal(stock.current_price, 129.50));
        assert!(price_equal(stock.net_price(), 9.25));
        assert!(price_equal(stock.base_notional(), 12025.0));
        assert!(price_equal(stock.current_notional(), 12950.0));
        assert!(price_equal(stock.net_notional(), 925.0));
    }

    #[test]
    fn test_stock_list() {
        let mut list = StockList::new();
        list.push(make_stock("AAPL", today_plus_days(-3), 100, 120.25, 125.25));
        list.push(make_stock("DELL", today_plus_days(-2), 100, 79.21, 79.71));
        assert_eq!(list.len(), 2);
        assert!(price_equal(net_notional(&list), 550.0));
        assert!(price_equal(current_notional(&list), 20496.0));

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

    fn make_stock(sym: &str, date: Date<Local>, qty: u32, base: Price, current: Price) -> Stock {
        let symbol = String::from(sym);
        let mut stock = Stock::new(symbol, date, qty, base);
        stock.set_current_price(current);
        stock
    }

    fn price_equal(lhs: Price, rhs: Price) -> bool {
        format!("{:.2}", lhs) == format!("{:.2}", rhs)
    }
}
