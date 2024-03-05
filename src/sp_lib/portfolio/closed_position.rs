use std::fmt;

use crate::util::datetime::SPDate;
use crate::util::fixed_price::FixedPrice;
use crate::portfolio::stock_type::StockType;
use crate::portfolio::symbol_trait::GetSymbol;

pub type Price = FixedPrice;

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
        Price::from_unsigned(self.quantity) * self.net_price()
    }

    #[inline(always)]
    pub fn base_notional(&self) -> Price {
        Price::from_unsigned(self.quantity) * self.base_price
    }

    #[inline(always)]
    pub fn exit_notional(&self) -> Price {
        Price::from_unsigned(self.quantity) * self.exit_price
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
    use crate::util::fixed_price::FP_0;

    #[test]
    fn test_closed_position_new() {
        let cp = default_closed_position();
        assert_eq!(cp.symbol, "AAPL");
        assert!(cp.stype == StockType::Cash);
        assert_eq!(cp.base_date, datetime::today_plus_days(-10));
        assert_eq!(cp.exit_date, datetime::today());
        assert_eq!(cp.quantity, 100);
        assert_eq!(cp.base_price, Price::from_string("115.00"));
        assert_eq!(cp.exit_price, Price::from_string("120.00"));
        assert_eq!(cp.base_fee, FP_0);
        assert_eq!(cp.exit_fee, Price::from_string("0.05"));
        assert_eq!(cp.dividend, Price::from_string("5.00"));
    }

    #[test]
    fn test_net_functions() {
        let cp = default_closed_position();
        assert_eq!(cp.net_price(), Price::from_string("5.00"));
        assert_eq!(cp.net_notional(), Price::from_string("500.00"));
    }

    fn default_closed_position() -> ClosedPosition {
        ClosedPosition::new(
            String::from("AAPL"),
            StockType::Cash,
            datetime::today_plus_days(-10),
            datetime::today(),
            100,
            Price::from_string("115.00"),
            Price::from_string("120.00"),
            FP_0,
            Price::from_string("0.05"),
            Price::from_string("5.00"))
    }
}
