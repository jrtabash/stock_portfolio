use std::fmt;
use chrono::{Date, Local};

pub type Price = f64;

pub struct Stock {
    pub symbol: String,
    pub date: Date<Local>,
    pub quantity: u32,
    pub base_price: Price,
    pub current_price: Price
}

pub type StockList = Vec<Stock>;

impl Stock {
    pub fn new(symbol: String,
               date: Date<Local>,
               quantity: u32,
               base_price: Price) -> Stock {
        let current_price: Price = 0.0;
        Stock { symbol, date, quantity, base_price, current_price }
    }

    pub fn set_current_price(self: &mut Stock, price: Price) {
        self.current_price = price
    }

    pub fn net_price(self: &Stock) -> Price {
        self.current_price - self.base_price
    }

    pub fn base_notional(self: &Stock) -> Price {
        self.quantity as Price * self.base_price
    }

    pub fn current_notional(self: &Stock) -> Price {
        self.quantity as Price * self.current_price
    }

    pub fn net_notional(self: &Stock) -> Price {
        self.quantity as Price * self.net_price()
    }
}

impl fmt::Display for Stock {
    fn fmt(self: &Stock, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stock({} {}@{:.2})", self.symbol, self.quantity, self.current_price)
    }
}
