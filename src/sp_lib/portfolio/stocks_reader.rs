use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

use crate::util::datetime;
use crate::portfolio::stock_type;
use crate::portfolio::stock::{Price, Stock, StockList};

pub struct StocksReader {
    stocks_file: String
}

impl StocksReader {
    pub fn new(stocks_file: String) -> StocksReader {
        StocksReader {
            stocks_file
        }
    }

    pub fn read(self: &StocksReader) -> Result<StockList, Box<dyn Error>> {
        match File::open(&self.stocks_file) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut content = String::new();
                match reader.read_to_string(&mut content) {
                    Ok(_) => Self::parse_content(&content),
                    Err(e) => Err(format!("StocksReader::read - {}", e).into())
                }
            },
            Err(e) => Err(format!("StocksReader::read - {}", e).into())
        }
    }

    pub fn parse_content(content: &String) -> Result<StockList, Box<dyn Error>> {
        let mut stocks = StockList::new();

        let mut skip_header: bool = true;
        for stock_line in content.lines() {
            // Assume first line is a header and skip it.
            if skip_header {
                skip_header = false;
                continue;
            }

            if stock_line == "" {
                continue;
            }

            let stock_tokens: Vec<&str> = stock_line.split(",").collect();
            if stock_tokens.len() != 5 {
                return Err(format!("StocksReader::parse_content - Invalid stock line '{}'", stock_line).into())
            }

            let symbol = String::from(stock_tokens[0]);
            let stype = stock_type::str2stocktype(stock_tokens[1])?;
            let date = datetime::parse_date(&stock_tokens[2])?;

            let quantity = match stock_tokens[3].parse::<u32>() {
                Ok(qty) => qty,
                Err(e) => return Err(format!("StocksReader::parse_content - Invalid quantity '{}'", e).into())
            };

            let base_price = match stock_tokens[4].parse::<Price>() {
                Ok(px) => px,
                Err(e) => return Err(format!("StocksReader::parse_content - Invalid base_price '{}'", e).into())
            };

            stocks.push(Stock::new(symbol, stype, date, quantity, base_price));
        }

        Ok(stocks)
    }
}
