use std::io::prelude::*;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use crate::portfolio::stock::StockList;
use crate::portfolio::stocks_reader::StocksReader;

pub struct StocksConfig {
    root: String,
    name: String,
    stocks: StockList
}

impl StocksConfig {
    pub fn new() -> Self {
        StocksConfig {
            root: String::new(),
            name: String::new(),
            stocks: StockList::new()
        }
    }

    pub fn from_file(config_file: &str) -> Result<Self, Box<dyn Error>> {
        match File::open(config_file) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut content = String::new();
                match reader.read_to_string(&mut content) {
                    Ok(_) => Self::parse(&content),
                    Err(e) => Err(format!("config::read - {}", e).into())
                }
            },
            Err(e) => Err(format!("StocksConfig::read - {}", e).into())
        }
    }

    pub fn from_str(config_str: &str) -> Result<Self, Box<dyn Error>> {
        Self::parse(config_str)
    }

    #[inline(always)] pub fn root(&self) -> &str { &self.root }
    #[inline(always)] pub fn name(&self) -> &str { &self.name }

    #[inline(always)] pub fn stocks(&self) -> &StockList { &self.stocks }
    #[inline(always)] pub fn stocks_mut(&mut self) -> &mut StockList { &mut self.stocks }

    // --------------------------------------------------------------------------------
    // Private Helpers

    fn parse(content: &str) -> Result<Self, Box<dyn Error>> {
        let mut root: String = env::var("HOME")?;
        let mut name: String = String::from("sp_datastore");
        let mut stocks: Option<StockList> = None;

        let mut collect_scontent = false;
        let mut scontent = String::new();

        for line in content.lines() {
            if line == "" { continue; }

            if line.trim() == "}" {
                stocks = Some(StocksReader::parse_content(&scontent)?);
                collect_scontent = false;
                continue;
            }

            if collect_scontent {
                scontent.push_str(line.trim());
                scontent.push('\n');
                continue;
            }

            let tokens: Vec<&str> = line
                .split(':')
                .map(|t| t.trim())
                .collect();
            if tokens.len() != 2 {
                return Err(format!("StocksConfig::parse - Invalid line '{}'", line).into());
            }

            let value: &str = tokens[1];
            match tokens[0] {
                "root" => if value != "$default" { root = String::from(value) },
                "name" => if value != "$default" { name = String::from(value) },
                "stocks" => {
                    if value != "csv{" {
                        return Err(format!("StocksConfig::parse - unsupported block type '{}'", value).into());
                    }
                    collect_scontent = true;
                },
                _ => (),
            };
        }

        Ok(StocksConfig {
            root: root,
            name: name,
            stocks: stocks.unwrap_or(StockList::new())
        })
    }
}
