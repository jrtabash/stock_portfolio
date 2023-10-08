use std::fmt;

use crate::util::datetime::SPDate;
use crate::util::price_type::PriceType;
use crate::portfolio::stock_type::StockType;
use crate::portfolio::symbol_trait::GetSymbol;

pub type Price = PriceType;

pub struct ClosedPosition {
    pub symbol: String,
    pub stype: StockType,
    pub base_date: SPDate,
    pub exit_date: SPDate,
    pub quantity: u32,
    pub base_price: Price,
    pub exit_price: Price,
    pub base_fee: Price,
    pub exit_fee: Price,
    pub dividend: Price
}

pub type ClosedPositionList = Vec<ClosedPosition>;

impl ClosedPosition {
    pub fn new(symbol: String,
               stype: StockType,
               base_date: SPDate,
               exit_date: SPDate,
               quantity: u32,
               base_price: Price,
               exit_price: Price,
               base_fee: Price,
               exit_fee: Price,
               dividend: Price) -> Self {
        ClosedPosition {
            symbol,
            stype,
            base_date,
            exit_date,
            quantity,
            base_price,
            exit_price,
            base_fee,
            exit_fee,
            dividend
        }
    }

    #[inline(always)]
    pub fn net_price(&self) -> Price {
        self.exit_price - self.base_price
    }

    #[inline(always)]
    pub fn net_notional(&self) -> Price {
        self.quantity as Price * self.net_price()
    }
}

impl fmt::Display for ClosedPosition {
    fn fmt(self: &ClosedPosition, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClosedPosition({} NetNotional={})", self.symbol, self.net_notional())
    }
}

impl GetSymbol for ClosedPosition {
    fn get_symbol(&self) -> &String {
        &self.symbol
    }
}

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::datetime;

    #[test]
    fn test_closed_position_new() {
        let cp = default_closed_position();
        assert_eq!(cp.symbol, "AAPL");
        assert!(cp.stype == StockType::Stock);
        assert_eq!(cp.base_date, datetime::today_plus_days(-10));
        assert_eq!(cp.exit_date, datetime::today());
        assert_eq!(cp.quantity, 100);
        assert_eq!(cp.base_price, 115.00);
        assert_eq!(cp.exit_price, 120.00);
        assert_eq!(cp.base_fee, 0.00);
        assert_eq!(cp.exit_fee, 0.05);
        assert_eq!(cp.dividend, 5.00);
    }

    #[test]
    fn test_net_functions() {
        let cp = default_closed_position();
        assert_eq!(cp.net_price(), 5.00);
        assert_eq!(cp.net_notional(), 500.00);
    }

    fn default_closed_position() -> ClosedPosition {
        ClosedPosition::new(
            String::from("AAPL"),
            StockType::Stock,
            datetime::today_plus_days(-10),
            datetime::today(),
            100,
            115.00,
            120.00,
            0.00,
            0.05,
            5.00)
    }
}
