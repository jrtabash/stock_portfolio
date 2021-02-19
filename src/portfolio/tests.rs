#[cfg(test)]
mod tests {
    use chrono::{Date, Local};
    use crate::portfolio::stock::*;

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

    fn make_stock(sym: &str, date: Date<Local>, qty: u32, base: f64, current: f64) -> Stock {
        let symbol = String::from(sym);
        let mut stock = Stock::new(symbol, date, qty, base);
        stock.set_current_price(current);
        stock
    }

    fn price_equal(lhs: f64, rhs: f64) -> bool {
        format!("{:2}", lhs) == format!("{:2}", rhs)
    }
}
