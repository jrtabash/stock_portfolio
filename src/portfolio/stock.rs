use std::fmt;

use crate::sputil::datetime;
use crate::sputil::datetime::LocalDate;
use crate::sputil::price_type::PriceType;
use crate::portfolio::stock_type::StockType;

pub type Price = PriceType;

pub struct Stock {
    pub symbol: String,          // Name
    pub stype: StockType,        // Stock Type
    pub date: LocalDate,         // Buy Date
    pub quantity: u32,           // Buy Quantity
    pub base_price: Price,       // Buy Price
    pub latest_price: Price,     // Latest Price
    pub latest_date: LocalDate,  // Latest Date
    pub days_held: i64           // Days Held
}

pub type StockList = Vec<Stock>;

impl Stock {
    pub fn new(symbol: String,
               stype: StockType,
               date: LocalDate,
               quantity: u32,
               base_price: Price) -> Stock {
        let latest_price: Price = 0.0;
        let latest_date = datetime::earliest_date();
        let days_held: i64 = 0;
        Stock { symbol, stype, date, quantity, base_price, latest_price, latest_date, days_held }
    }

    #[inline(always)]
    pub fn set_latest_price(self: &mut Stock, price: Price, date: LocalDate) {
        self.latest_price = price;
        self.latest_date = date;
        self.days_held = datetime::count_days(&self.date, &self.latest_date)
    }

    #[inline(always)]
    pub fn net_price(self: &Stock) -> Price {
        self.latest_price - self.base_price
    }

    #[inline(always)]
    pub fn base_notional(self: &Stock) -> Price {
        self.quantity as Price * self.base_price
    }

    #[inline(always)]
    pub fn latest_notional(self: &Stock) -> Price {
        self.quantity as Price * self.latest_price
    }

    #[inline(always)]
    pub fn net_notional(self: &Stock) -> Price {
        self.quantity as Price * self.net_price()
    }

    #[inline(always)]
    pub fn pct_change(self: &Stock) -> f64 {
        100.0 * self.net_price() / self.base_price
    }
}

impl fmt::Display for Stock {
    fn fmt(self: &Stock, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stock({} {}@{:.2})", self.symbol, self.quantity, self.latest_price)
    }
}

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stock_new() {
        let stock = Stock::new(String::from("AAPL"), StockType::Stock, datetime::today(), 100, 120.25);
        assert_eq!(stock.symbol, "AAPL");
        assert!(stock.stype == StockType::Stock);
        assert_eq!(stock.date, datetime::today());
        assert_eq!(stock.quantity, 100);
        assert_eq!(stock.base_price, 120.25);
        assert_eq!(stock.latest_price, 0.0);
        assert_eq!(stock.latest_date, datetime::earliest_date());
    }

    #[test]
    fn test_stock_set_latest_price() {
        let mut stock = Stock::new(String::from("AAPL"), StockType::Stock, datetime::today(), 100, 120.25);
        assert_eq!(stock.latest_price, 0.0);
        assert_eq!(stock.latest_date, datetime::earliest_date());

        stock.set_latest_price(125.50, datetime::today());
        assert_eq!(stock.latest_price, 125.50);
        assert_eq!(stock.latest_date, datetime::today());
        assert_eq!(stock.days_held, 0);

        stock.set_latest_price(125.0, datetime::today_plus_days(10));
        assert_eq!(stock.latest_price, 125.0);
        assert_eq!(stock.latest_date, datetime::today_plus_days(10));
        assert_eq!(stock.days_held, 10);
    }

    #[test]
    fn test_stock_getters() {
        let mut stock = Stock::new(String::from("AAPL"), StockType::Stock, datetime::today(), 100, 120.25);
        stock.set_latest_price(125.50, datetime::today());

        assert_eq!(stock.net_price(), 5.25);
        assert_eq!(stock.base_notional(), 12025.0);
        assert_eq!(stock.latest_notional(), 12550.0);
        assert_eq!(stock.net_notional(), 525.0);
        assert!((stock.pct_change() - 4.36) <= 0.009);
    }

    #[test]
    fn test_stock_display() {
        let mut stock = Stock::new(String::from("AAPL"), StockType::Stock, datetime::today(), 100, 120.25);
        stock.set_latest_price(125.50, datetime::today());

        assert_eq!(format!("{}", stock), "Stock(AAPL 100@125.50)");
    }
}
