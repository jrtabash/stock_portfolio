use std::error::Error;
use std::fmt;

#[derive(Debug, Copy, Clone)]
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

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stock_type() {
        let stock = StockType::Stock;
        let etf = StockType::ETF;
        let stock_str = "stock";
        let etf_str = "etf";

        assert_eq!(stocktype2str(stock), stock_str);
        assert_eq!(stocktype2str(etf), etf_str);
        assert!(str2stocktype(&stock_str).unwrap() == stock);
        assert!(str2stocktype(&etf_str).unwrap() == etf);

        match str2stocktype("foobar") {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(format!("{}", err), "Unknown stock type 'foobar'")
        };
    }
}
