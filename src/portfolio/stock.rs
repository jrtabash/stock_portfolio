use std::fmt;

use crate::sputil::datetime::*;
use crate::sputil::price_type::*;
use crate::portfolio::stock_type::*;

pub type Price = PriceType;

pub struct Stock {
    pub symbol: String,          // Ticker
    pub stype: StockType,        // Stock Type
    pub date: LocalDate,         // Buy Date
    pub quantity: u32,           // Buy Quantity
    pub base_price: Price,       // Buy Price
    pub latest_price: Price,     // Latest Price
    pub latest_date: LocalDate   // Latest Date
}

pub type StockList = Vec<Stock>;

impl Stock {
    pub fn new(symbol: String,
               stype: StockType,
               date: LocalDate,
               quantity: u32,
               base_price: Price) -> Stock {
        let latest_price: Price = 0.0;
        let latest_date = earliest_date();
        Stock { symbol, stype, date, quantity, base_price, latest_price, latest_date }
    }

    #[inline(always)]
    pub fn set_latest_price(self: &mut Stock, price: Price, date: LocalDate) {
        self.latest_price = price;
        self.latest_date = date;
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
