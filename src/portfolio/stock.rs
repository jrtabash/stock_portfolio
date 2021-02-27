use std::fmt;
use chrono::{Date, Local};

use crate::sputil::datetime::*;

pub type Price = f64;

pub struct Stock {
    pub symbol: String,            // Ticker
    pub date: Date<Local>,         // Buy Date
    pub quantity: u32,             // Buy Quantity
    pub base_price: Price,         // Buy Price
    pub latest_price: Price,       // Latest Price
    pub latest_date: Date<Local>   // Latest Date
}

pub type StockList = Vec<Stock>;

impl Stock {
    pub fn new(symbol: String,
               date: Date<Local>,
               quantity: u32,
               base_price: Price) -> Stock {
        let latest_price: Price = 0.0;
        let latest_date = earliest_date();
        Stock { symbol, date, quantity, base_price, latest_price, latest_date }
    }

    pub fn set_latest_price(self: &mut Stock, price: Price, date: Date<Local>) {
        self.latest_price = price;
        self.latest_date = date;
    }

    pub fn net_price(self: &Stock) -> Price {
        self.latest_price - self.base_price
    }

    pub fn base_notional(self: &Stock) -> Price {
        self.quantity as Price * self.base_price
    }

    pub fn latest_notional(self: &Stock) -> Price {
        self.quantity as Price * self.latest_price
    }

    pub fn net_notional(self: &Stock) -> Price {
        self.quantity as Price * self.net_price()
    }
}

impl fmt::Display for Stock {
    fn fmt(self: &Stock, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stock({} {}@{:.2})", self.symbol, self.quantity, self.latest_price)
    }
}
