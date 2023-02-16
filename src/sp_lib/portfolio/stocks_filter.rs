use std::collections::HashSet;
use std::error::Error;
use crate::portfolio::stock::{Stock, StockList};
use crate::portfolio::stock_type;

// --------------------------------------------------------------------------------
// StocksFilter

trait FilterFtn {
    fn filter_stocks(&self, stocks: &mut StockList, keep: bool);
}

type FilterFtnPtr = Box<dyn FilterFtn>;

pub struct StocksFilter {
    func: FilterFtnPtr
}

impl StocksFilter {
    pub fn from(filter_str: &str) -> Result<Self, Box<dyn Error>> {
        Ok(StocksFilter {
            func: Self::make_filter_func(filter_str)?
        })
    }

    pub fn filter_stocks(&self, stocks: &mut StockList, keep: bool) {
        self.func.filter_stocks(stocks, keep)
    }

    fn make_filter_func(filter_str: &str) -> Result<FilterFtnPtr, Box<dyn Error>> {
        let fstr = filter_str.trim();
        if let Ok(stype) = stock_type::str2stocktype(fstr) {
            Ok(Box::new(TypeFilter::make(stype)))
        }
        else if fstr.contains(',') || !fstr.contains(' ') {
            Ok(Box::new(SymbolsFilter::make(filter_str)))
        }
        else {
            Ok(Box::new(ExprFilter::make(filter_str)?))
        }
    }
}

// --------------------------------------------------------------------------------
// Type Filter

struct TypeFilter {
    stype: stock_type::StockType
}

impl TypeFilter {
    pub fn make(stype: stock_type::StockType) -> Self {
        TypeFilter {
            stype
        }
    }
}

impl FilterFtn for TypeFilter {
    fn filter_stocks(&self, stocks: &mut StockList, keep: bool) {
        stocks.retain(|stock| (stock.stype == self.stype) == keep);
    }
}

// --------------------------------------------------------------------------------
// Symbols Filter

struct SymbolsFilter {
    symbols: HashSet<String>
}

impl SymbolsFilter {
    pub fn make(filter_str: &str) -> Self {
        SymbolsFilter {
            symbols: filter_str.split(',').map(|name| String::from(name.trim())).collect()
        }
    }
}

impl FilterFtn for SymbolsFilter {
    fn filter_stocks(&self, stocks: &mut StockList, keep: bool) {
        stocks.retain(|stock| self.symbols.contains(stock.symbol.as_str()) == keep);
    }
}

// --------------------------------------------------------------------------------
// Expression Filter

struct ExprFilter {
    field_ftn: fn(&Stock) -> f64,
    op_ftn: fn(f64, f64) -> bool,
    value: f64
}

pub type ExprFieldFtn = fn(&Stock) -> f64;
pub type ExprOpFtn = fn(f64, f64) -> bool;

impl ExprFilter {
    pub fn make(filter_expr: &str) -> Result<Self, Box<dyn Error>> {
        let tokens: Vec<&str> = filter_expr.split_whitespace().collect();
        if tokens.len() != 3 {
            return Err(format!("Invalid by expression '{}'", filter_expr).into())
        }

        let field_ftn = Self::make_field_ftn(tokens[0])?;
        let op_ftn = Self::make_op_ftn(tokens[1])?;
        let value = tokens[2].parse::<f64>()?;

        Ok(ExprFilter {
            field_ftn,
            op_ftn,
            value
        })
    }

    fn make_field_ftn(field: &str) -> Result<ExprFieldFtn, Box<dyn Error>> {
        if field == "days" {
            Ok(|stock| stock.days_held as f64)
        } else if field == "price" {
            Ok(|stock| stock.latest_price)
        } else if field == "net" {
            Ok(|stock| stock.net_price())
        } else if field == "pct" {
            Ok(|stock| stock.pct_change())
        } else if field == "div" {
            Ok(|stock| stock.cum_dividend)
        } else if field == "size" {
            Ok(|stock| stock.quantity as f64)
        } else if field == "value" {
            Ok(|stock| stock.latest_notional())
        } else {
            Err(format!("Unsupported filter expression field '{}'", field).into())
        }
    }

    fn make_op_ftn(op: &str) -> Result<ExprOpFtn, Box<dyn Error>> {
        if op == "=" {
            Ok(|l, r| l == r)
        } else if op == "!=" {
            Ok(|l, r| l != r)
        } else if op == "<" {
            Ok(|l, r| l < r)
        } else if op == ">" {
            Ok(|l, r| l > r)
        } else if op == "<=" {
            Ok(|l, r| l <= r)
        } else if op == ">=" {
            Ok(|l, r| l >= r)
        } else {
            return Err(format!("Unsupported filter expression op '{}'", op).into())
        }
    }
}

impl FilterFtn for ExprFilter {
    fn filter_stocks(&self, stocks: &mut StockList, keep: bool) {
        stocks.retain(|stock| (self.op_ftn)((self.field_ftn)(stock), self.value) == keep);
    }
}
