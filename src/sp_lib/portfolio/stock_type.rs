use std::fmt;
use crate::util::error::Error;

#[derive(Debug, Copy, Clone)]
#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub enum StockType {
    Cash,
    ETF,
    Index
}

pub fn stocktype2str(st: StockType) -> &'static str {
    match st {
        StockType::Cash => "cash",
        StockType::ETF => "etf",
        StockType::Index => "index"
    }
}

pub fn str2stocktype(ststr: &str) -> Result<StockType, Error> {
    match ststr.to_lowercase().as_str() {
        "cash" | "stock" => Ok(StockType::Cash),
        "etf" => Ok(StockType::ETF),
        "index" => Ok(StockType::Index),
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
        let cash = StockType::Cash;
        let etf = StockType::ETF;
        let index = StockType::Index;
        let cash_str = "cash";
        let etf_str = "etf";
        let index_str = "index";

        assert_eq!(stocktype2str(cash), cash_str);
        assert_eq!(stocktype2str(etf), etf_str);
        assert_eq!(stocktype2str(index), index_str);
        assert!(str2stocktype(&cash_str).unwrap() == cash);
        assert!(str2stocktype(&etf_str).unwrap() == etf);
        assert!(str2stocktype(&index_str).unwrap() == index);

        // Make sure str2stocktype is backward compatible.
        assert!(str2stocktype("stock").unwrap() == cash);

        match str2stocktype("foobar") {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(format!("{}", err), "Unknown stock type 'foobar'")
        };
    }
}
