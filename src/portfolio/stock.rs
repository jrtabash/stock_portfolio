use std::fmt;
use chrono::{Date, Local};

pub struct Stock {
    pub symbol: String,
    pub date: Date<Local>,
    pub quantity: u32,
    pub base_price: f64,
    pub current_price: f64
}

impl Stock {
    pub fn new(symbol: String,
               date: Date<Local>,
               quantity: u32,
               base_price: f64) -> Stock {
        let current_price: f64 = 0.0;
        Stock { symbol, date, quantity, base_price, current_price }
    }

    pub fn set_current_price(self: &mut Stock, price: f64) {
        self.current_price = price
    }

    pub fn net_price(self: &Stock) -> f64 {
        self.current_price - self.base_price
    }

    pub fn base_notional(self: &Stock) -> f64 {
        self.quantity as f64 * self.base_price
    }

    pub fn current_notional(self: &Stock) -> f64 {
        self.quantity as f64 * self.current_price
    }

    pub fn net_notional(self: &Stock) -> f64 {
        self.quantity as f64 * self.net_price()
    }
}

impl fmt::Display for Stock {
    fn fmt(self: &Stock, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}@{:.2}", self.symbol, self.quantity, self.current_price)
    }
}
