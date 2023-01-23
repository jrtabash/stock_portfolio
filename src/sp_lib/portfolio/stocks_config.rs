use std::io::prelude::*;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use crate::portfolio::stock::StockList;
use crate::portfolio::stocks_reader::StocksReader;

#[derive(Copy, Clone, PartialEq)]
enum SContentType {
    None,
    CSV,
    CSVFile,
}

pub struct StocksConfig {
    ds_root: String,
    ds_name: String,
    stocks: StockList
}

impl StocksConfig {
    pub fn new() -> Self {
        StocksConfig {
            ds_root: String::new(),
            ds_name: String::new(),
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

    #[inline(always)] pub fn ds_root(&self) -> &str { &self.ds_root }
    #[inline(always)] pub fn ds_name(&self) -> &str { &self.ds_name }

    #[inline(always)] pub fn stocks(&self) -> &StockList { &self.stocks }
    #[inline(always)] pub fn stocks_mut(&mut self) -> &mut StockList { &mut self.stocks }

    // --------------------------------------------------------------------------------
    // Private Helpers

    fn parse(content: &str) -> Result<Self, Box<dyn Error>> {
        let mut root: String = env::var("HOME")?;
        let mut name: String = String::from("sp_datastore");
        let mut stocks: Option<StockList> = None;

        let mut collect_scontent = false;
        let mut scontent_type = SContentType::None;
        let mut scontent = String::new();

        for line in content.lines() {
            if line == "" { continue; }

            if line.trim() == "}" {
                stocks = Some(
                    match scontent_type {
                        SContentType::CSV => StocksReader::parse_content(&scontent)?,
                        SContentType::CSVFile => StocksReader::new(scontent.trim().to_string()).read()?,
                        SContentType::None => return Err("StocksConfig::parse - Unexpected scontent type None".into())
                    });
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
                "ds_root" => if value != "$default" { root = String::from(value) },
                "ds_name" => if value != "$default" { name = String::from(value) },
                "stocks" => {
                    if scontent_type != SContentType::None {
                        // We have already parsed a stocks entry, there shouldn't be another one.
                        return Err("StocksConfig::parse - Unsupported multiple stocks entries".into());
                    }

                    collect_scontent = true;
                    match value {
                        "csv{" => scontent_type = SContentType::CSV,
                        "csv_file{" => scontent_type = SContentType::CSVFile,
                        _ => return Err(format!("StocksConfig::parse - Unsupported block type '{}'", value).into())
                    };
                },
                _ => {
                    return Err(format!("StocksConfig::parse - Unknown config name '{}'", tokens[0]).into());
                }
            };
        }

        Ok(StocksConfig {
            ds_root: root,
            ds_name: name,
            stocks: stocks.unwrap_or(StockList::new())
        })
    }
}
