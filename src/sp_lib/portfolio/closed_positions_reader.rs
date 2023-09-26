use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

use crate::util::datetime;
use crate::portfolio::stock_type;
use crate::portfolio::closed_position::{Price, ClosedPosition, ClosedPositionList};

pub struct ClosedPositionsReader {
    closed_positions_file: String
}

impl ClosedPositionsReader {
    pub fn new(closed_positions_file: String) -> Self {
        ClosedPositionsReader {
            closed_positions_file
        }
    }

    pub fn read(&self) -> Result<ClosedPositionList, Box<dyn Error>> {
        match File::open(&self.closed_positions_file) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut content = String::new();
                match reader.read_to_string(&mut content) {
                    Ok(_) => Self::parse_content(&content),
                    Err(e) => Err(format!("ClosedPositionsReader::read - {}", e).into())
                }
            },
            Err(e) => Err(format!("ClosedPositionsReader::read - {}", e).into())
        }
    }

    pub fn parse_content(content: &str) -> Result<ClosedPositionList, Box<dyn Error>> {
        let mut positions = ClosedPositionList::new();

        let mut skip_header: bool = true;
        for position_line in content.lines() {
            // Assume first line is a header and skip it.
            if skip_header {
                skip_header = false;
                continue;
            }

            if position_line.is_empty() {
                continue;
            }

            let position_tokens: Vec<&str> = position_line.split(',').collect();
            if position_tokens.len() != 10 {
                return Err(format!("ClosedPositionsReader::parse_content - Invalid position line '{}'", position_line).into())
            }

            let symbol = String::from(position_tokens[0]);
            let stype = stock_type::str2stocktype(position_tokens[1])?;
            let base_date = datetime::parse_date(position_tokens[2])?;
            let exit_date = datetime::parse_date(position_tokens[3])?;

            let quantity = match position_tokens[4].parse::<u32>() {
                Ok(qty) => qty,
                Err(e) => return Err(format!("ClosedPositionsReader::parse_content - Invalid quantity '{}'", e).into())
            };

            let base_price = Self::parse_price(position_tokens[5], "base_price")?;
            let exit_price = Self::parse_price(position_tokens[6], "exit_price")?;
            let base_fee = Self::parse_price(position_tokens[7], "base_fee")?;
            let exit_fee = Self::parse_price(position_tokens[8], "exit_fee")?;
            let dividend = Self::parse_price(position_tokens[9], "dividend")?;

            positions.push(ClosedPosition::new(
                symbol,
                stype,
                base_date,
                exit_date,
                quantity,
                base_price,
                exit_price,
                base_fee,
                exit_fee,
                dividend));
        }

        Ok(positions)
    }

    fn parse_price(token: &str, which: &str) -> Result <Price, Box<dyn Error>> {
        match token.parse::<Price>() {
            Ok(px) => Ok(px),
            Err(e) => Err(format!("ClosedPositionsReader::parse_price - Invalid {} '{}'", which, e).into())
        }
    }
}

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::portfolio::stock_type::StockType;

    #[test]
    fn test_parse_content() {
        let content = make_content();
        match ClosedPositionsReader::parse_content(&content) {
            Ok(positions) => {
                assert_eq!(positions.len(), 3);
                check_equal(&positions[0], &position_0());
                check_equal(&positions[1], &position_1());
                check_equal(&positions[2], &position_2());
            },
            Err(e) => {
                eprintln!("{}", e);
                assert!(false);
            }
        };
    }

    fn make_content() -> &'static str {
        "symbol,type,base_date,exit_date,quantity,base_price,exit_price,base_fee,exit_fee,dividend\n\
         MYSYM,stock,2016-04-15,2023-03-28,100,44.10,131.56,0.00,0.12,1009.00\n\
         MYSYM,stock,2016-04-15,2023-03-28,100,44.10,131.55,0.00,0.12,1009.00\n\
         MYOTH,stock,2021-10-18,2023-09-06,44,85.60,165.45,0.00,0.07,1205.60\n"
    }

    fn position_0() -> ClosedPosition {
        ClosedPosition::new("MYSYM".to_string(),
                            StockType::Stock,
                            datetime::make_date(2016, 4, 15),
                            datetime::make_date(2023, 3, 28),
                            100,
                            44.10,
                            131.56,
                            0.00,
                            0.12,
                            1009.00)
    }

    fn position_1() -> ClosedPosition {
        ClosedPosition::new("MYSYM".to_string(),
                            StockType::Stock,
                            datetime::make_date(2016, 4, 15),
                            datetime::make_date(2023, 3, 28),
                            100,
                            44.10,
                            131.55,
                            0.00,
                            0.12,
                            1009.00)
    }

    fn position_2() -> ClosedPosition {
        ClosedPosition::new("MYOTH".to_string(),
                            StockType::Stock,
                            datetime::make_date(2021, 10, 18),
                            datetime::make_date(2023, 9, 6),
                            44,
                            85.60,
                            165.45,
                            0.00,
                            0.07,
                            1205.60)
    }

    fn check_equal(lhs: &ClosedPosition, rhs: &ClosedPosition) {
        assert_eq!(lhs.symbol, rhs.symbol);
        assert_eq!(lhs.stype, rhs.stype);
        assert_eq!(lhs.base_date, rhs.base_date);
        assert_eq!(lhs.exit_date, rhs.exit_date);
        assert_eq!(lhs.quantity, rhs.quantity);
        assert_eq!(lhs.base_price, rhs.base_price);
        assert_eq!(lhs.exit_price, rhs.exit_price);
        assert_eq!(lhs.base_fee, rhs.base_fee);
        assert_eq!(lhs.exit_fee, rhs.exit_fee);
        assert_eq!(lhs.dividend, rhs.dividend);
    }
}
