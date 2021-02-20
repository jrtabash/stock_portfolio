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
// Stock Portfolio

pub struct StockPortfolio {
    pub stocks: Vec<Stock>
}

impl StockPortfolio {
    pub fn new() -> StockPortfolio {
        let stocks: Vec<Stock> = Vec::new();
        StockPortfolio { stocks }
    }

    pub fn count(self: &StockPortfolio) -> usize {
        self.stocks.len()
    }

    pub fn add_stock(self: &mut StockPortfolio, stock: Stock) {
        self.stocks.push(stock);
    }

    pub fn current_notional(self: &StockPortfolio) -> Price {
        self.stocks.iter().map(|stock| stock.current_notional()).sum()
    }

    pub fn net_notional(self: &StockPortfolio) -> Price {
        self.stocks.iter().map(|stock| stock.net_notional()).sum()
    }

    pub fn report(self: &StockPortfolio) {
        let header = vec!["Ticker", "Date\t", "Qty", "Base", "Current", " Net", "NetVal", "CurrentVal"];
        let seprts = vec!["------", "----\t", "---", "----", "-------", " ---", "------", "----------"];

        println!("Stock Portfolio");
        println!("---------------");
        println!("            Date: {}", Local::today().format("%Y-%m-%d"));
        println!("           Count: {}", self.count());
        println!("    Net Notional: {:.2}", self.net_notional());
        println!("Current Notional: {:.2}", self.current_notional());
        println!("");
        println!("{}", header.join("\t"));
        println!("{}", seprts.join("\t"));

        for stock in self.stocks.iter() {
            println!("{}\t{}\t{}\t{:.2}\t{:.2}\t {:.2}\t{:.2}\t{:.2}",
                     stock.symbol,
                     stock.date.format("%Y-%m-%d"),
                     stock.quantity,
                     stock.base_price,
                     stock.current_price,
                     stock.net_price(),
                     stock.net_notional(),
                     stock.current_notional());
        }
    }
}

impl fmt::Display for StockPortfolio {
    fn fmt(self: &StockPortfolio, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StockPortfolio({})", self.stocks.len())
    }
}
