use std::fmt;

use crate::util::datetime;
use crate::util::datetime::SPDate;
use crate::util::price_type::{PriceType, calc_daily};
use crate::portfolio::stock_type::StockType;
use crate::portfolio::symbol_trait::GetSymbol;

pub type Price = PriceType;

pub struct Stock {
    pub symbol: String,          // Name
    pub stype: StockType,        // Stock Type
    pub date: SPDate,            // Buy Date
    pub quantity: u32,           // Buy Quantity
    pub base_price: Price,       // Buy Price
    pub cum_dividend: Price,     // Cumulative Dividend
    pub latest_price: Price,     // Latest Price
    pub latest_date: SPDate,     // Latest Date
    pub latest_div_price: Price, // Latest Dividend Price
    pub latest_div_date: SPDate, // Latest Dividend Date
    pub days_held: i64,          // Days Held

    // For temporary use with extra sorting and other algorithms
    pub user_data: f64
}

pub type StockList = Vec<Stock>;

impl Stock {
    pub fn new(symbol: String,
               stype: StockType,
               date: SPDate,
               quantity: u32,
               base_price: Price) -> Stock {
        Stock {
            symbol,
            stype,
            date,
            quantity,
            base_price,
            cum_dividend: 0.0,
            latest_price: 0.0,
            latest_date: datetime::earliest_date(),
            latest_div_price: 0.0,
            latest_div_date: datetime::earliest_date(),
            days_held: 0,
            user_data: 0.0
        }
    }

    #[inline(always)]
    pub fn set_latest_price(self: &mut Stock, price: Price, date: SPDate) {
        self.latest_price = price;
        self.latest_date = date;
        self.days_held = datetime::count_days(&self.date, &self.latest_date)
    }

    #[inline(always)]
    pub fn set_latest_dividend(self: &mut Stock, price: Price, date: SPDate) {
        self.latest_div_price = price;
        self.latest_div_date = date;
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

    #[inline(always)]
    pub fn latest_dividend(self: &Stock) -> Price {
        self.quantity as Price * self.latest_div_price
    }

    #[inline(always)]
    pub fn yearly_dividend(self: &Stock) -> Price {
        365.0 * calc_daily(self.cum_dividend, self.days_held)
    }

    #[inline(always)]
    pub fn daily_unit_dividend(self: &Stock) -> Price {
        if self.quantity > 0 {
            calc_daily(self.cum_dividend / self.quantity as Price, self.days_held)
        } else {
            0.0
        }
    }

    #[inline(always)]
    pub fn cum_dividend_return(self: &Stock) -> Price {
        100.0 * self.cum_dividend / self.base_notional()
    }
}

impl fmt::Display for Stock {
    fn fmt(self: &Stock, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stock({} {}@{:.2})", self.symbol, self.quantity, self.latest_price)
    }
}

impl GetSymbol for Stock {
    fn get_symbol(&self) -> &String {
        &self.symbol
    }
}

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stock_new() {
        let stock = Stock::new(String::from("AAPL"), StockType::Cash, datetime::today(), 100, 120.25);
        assert_eq!(stock.symbol, "AAPL");
        assert!(stock.stype == StockType::Cash);
        assert_eq!(stock.date, datetime::today());
        assert_eq!(stock.quantity, 100);
        assert_eq!(stock.base_price, 120.25);
        assert_eq!(stock.cum_dividend, 0.0);
        assert_eq!(stock.latest_price, 0.0);
        assert_eq!(stock.latest_date, datetime::earliest_date());
        assert_eq!(stock.days_held, 0);
        assert_eq!(stock.yearly_dividend(), 0.0);
        assert_eq!(stock.daily_unit_dividend(), 0.0);
        assert_eq!(stock.cum_dividend_return(), 0.0);
    }

    #[test]
    fn test_stock_set_latest_price() {
        let mut stock = Stock::new(String::from("AAPL"), StockType::Cash, datetime::today(), 100, 120.25);
        assert_eq!(stock.latest_price, 0.0);
        assert_eq!(stock.latest_date, datetime::earliest_date());
        assert_eq!(stock.days_held, 0);

        stock.set_latest_price(125.50, datetime::today());
        assert_eq!(stock.latest_price, 125.50);
        assert_eq!(stock.latest_date, datetime::today());
        assert_eq!(stock.days_held, 0);

        stock.set_latest_price(125.0, datetime::today_plus_days(10));
        assert_eq!(stock.latest_price, 125.0);
        assert_eq!(stock.latest_date, datetime::today_plus_days(10));
        assert_eq!(stock.days_held, 10);
        assert_eq!(stock.daily_unit_dividend(), 0.0);
        assert_eq!(stock.cum_dividend_return(), 0.0);
    }

    #[test]
    fn test_stock_latest_dividend() {
        let mut stock = Stock::new(String::from("AAPL"), StockType::Cash, datetime::today(), 100, 120.25);
        assert_eq!(stock.latest_div_date, datetime::earliest_date());
        assert_eq!(stock.latest_div_price, 0.0);
        assert_eq!(stock.latest_dividend(), 0.0);

        stock.set_latest_dividend(0.5, datetime::today());
        assert_eq!(stock.latest_div_date, datetime::today());
        assert_eq!(stock.latest_div_price, 0.5);
        assert_eq!(stock.latest_dividend(), 50.0);
    }

    #[test]
    fn test_stock_getters() {
        let mut stock = Stock::new(String::from("AAPL"), StockType::Cash, datetime::today(), 100, 120.25);
        stock.set_latest_price(125.50, datetime::today());

        assert_eq!(stock.net_price(), 5.25);
        assert_eq!(stock.base_notional(), 12025.0);
        assert_eq!(stock.latest_notional(), 12550.0);
        assert_eq!(stock.net_notional(), 525.0);
        assert!((stock.pct_change() - 4.36) <= 0.009);
    }

    #[test]
    fn test_stock_display() {
        let mut stock = Stock::new(String::from("AAPL"), StockType::Cash, datetime::today(), 100, 120.25);
        stock.set_latest_price(125.50, datetime::today());

        assert_eq!(format!("{}", stock), "Stock(AAPL 100@125.50)");
    }

    #[test]
    fn test_stock_dividend_functions() {
        let mut stock = Stock::new(String::from("AAPL"), StockType::Cash, datetime::today(), 200, 120.25);
        stock.set_latest_price(125.50, datetime::today_plus_days(40));
        stock.cum_dividend = 115.0;

        assert_eq!(stock.days_held, 40);
        assert!((stock.yearly_dividend() - 1049.375) <= 0.000000000001);
        assert!((stock.daily_unit_dividend() - 0.014374999999999999).abs() <= 0.000000000001);
        assert!((stock.cum_dividend_return() - 0.478170478170478170).abs() <= 0.000000000001);
    }
}
