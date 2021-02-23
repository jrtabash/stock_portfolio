use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

use crate::sputil::datetime::*;
use crate::portfolio::stock::*;

pub struct StocksReader {
    stocks_file: String
}

impl StocksReader {
    pub fn new(stocks_file: String) -> StocksReader {
        StocksReader { stocks_file }
    }

    pub fn read(self: &StocksReader) -> Result<StockList, String> {
        match File::open(&self.stocks_file) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut content = String::new();
                match reader.read_to_string(&mut content) {
                    Ok(_) => self.parse_content(&content),
                    Err(e) => Result::Err(format!("StocksReader::read - {}", e))
                }
            },
            Err(e) => Result::Err(format!("StocksReader::read - {}", e))
        }
    }

    // --------------------------------------------------------------------------------
    // Private

    fn parse_content(self: &StocksReader, content: &String) -> Result<StockList, String> {
        let mut stocks = StockList::new();

        let mut skip: bool = true;
        for stock_line in content.split("\n") {
            // Assume first line is a header and skip it.
            if skip {
                skip = false;
                continue;
            }

            if stock_line == "" {
                continue;
            }

            let stock_tokens: Vec<&str> = stock_line.split(",").collect();
            if stock_tokens.len() != 4 {
                return Result::Err(format!("StocksReadr::parse_content - Invalid stock line '{}'", stock_line))
            }

            let symbol = String::from(stock_tokens[0]);
            let date = parse_date(&stock_tokens[1])?;

            let quantity = match stock_tokens[2].parse::<u32>() {
                Ok(qty) => qty,
                Err(e) => return Result::Err(format!("StocksReader::parse_content - Invalid quantity '{}'", e))
            };

            let base_price = match stock_tokens[3].parse::<Price>() {
                Ok(px) => px,
                Err(e) => return Result::Err(format!("StocksReader::parse_content - Invalid base_price '{}'", e))
            };

            stocks.push(Stock::new(symbol, date, quantity, base_price));
        }

        Ok(stocks)
    }
}
