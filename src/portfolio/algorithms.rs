use crate::portfolio::stock::*;

pub fn current_notional(stocks: &StockList) -> Price {
    stocks.iter().map(|stock| stock.current_notional()).sum()
}

pub fn net_notional(stocks: &StockList) -> Price {
    stocks.iter().map(|stock| stock.net_notional()).sum()
}
