use std::error::Error;
use std::fmt;

#[derive(Copy, Clone)]
#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub enum StockType {
    Stock,
    ETF
}

pub fn stocktype2str(st: StockType) -> &'static str {
    match st {
        StockType::Stock => "stock",
        StockType::ETF => "etf"
    }
}

pub fn str2stocktype(ststr: &str) -> Result<StockType, Box<dyn Error>> {
    match ststr.to_lowercase().as_str() {
        "stock" => Ok(StockType::Stock),
        "etf" => Ok(StockType::ETF),
        _ => Err(format!("Unknown stock type '{}'", ststr).into())
    }
}

impl fmt::Display for StockType {
    fn fmt(self: &StockType, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", stocktype2str(*self))
    }
}
