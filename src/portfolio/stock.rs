use std::fmt;
use chrono::{Date, Local};

pub type Price = f64;

// --------------------------------------------------------------------------------
// Stock

pub struct Stock {
    pub symbol: String,
    pub date: Date<Local>,
    pub quantity: u32,
    pub base_price: Price,
    pub current_price: Price
}

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

// --------------------------------------------------------------------------------
// Stock List

pub struct StockList {
    pub stocks: Vec<Stock>
}

pub struct StockIterator<'a> {
    stocks_ref: &'a Vec<Stock>,
    index: usize,
}

impl StockList {
    pub fn new() -> StockList {
        let stocks: Vec<Stock> = Vec::new();
        StockList { stocks }
    }

    pub fn count(self: &StockList) -> usize {
        self.stocks.len()
    }

    pub fn add_stock(self: &mut StockList, stock: Stock) {
        self.stocks.push(stock);
    }

    pub fn current_notional(self: &StockList) -> Price {
        self.stocks.iter().map(|stock| stock.current_notional()).sum()
    }

    pub fn net_notional(self: &StockList) -> Price {
        self.stocks.iter().map(|stock| stock.net_notional()).sum()
    }

    pub fn iter(self: &StockList) -> StockIterator {
        let stocks_ref = &self.stocks;
        let index = 0;
        StockIterator { stocks_ref, index }
    }
}

impl fmt::Display for StockList {
    fn fmt(self: &StockList, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StockList({})", self.stocks.len())
    }
}

impl<'a> Iterator for StockIterator<'a> {
    type Item = &'a Stock;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.stocks_ref.len() {
            let idx = self.index;
            self.index += 1;
            Some(&self.stocks_ref[idx])
        }
        else {
            None
        }
    }
}
