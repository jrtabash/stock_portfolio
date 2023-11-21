use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::io::BufReader;
use crate::util::error::Error;
use crate::portfolio::stock::{Price, StockList};
use crate::portfolio::stocks_reader::StocksReader;
use crate::portfolio::closed_position::ClosedPositionList;
use crate::portfolio::closed_positions_reader::ClosedPositionsReader;

#[derive(Copy, Clone, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
enum SContentType {
    None,
    CSV,
    CSVFile,
    CSVCP,
    CSVFileCP,
}

pub struct StocksConfig {
    ds_root: String,
    ds_name: String,
    stocks: StockList,
    closed_positions: ClosedPositionList,
    cash: Price
}

impl StocksConfig {
    pub fn new() -> Self {
        StocksConfig {
            ds_root: String::new(),
            ds_name: String::new(),
            stocks: StockList::new(),
            closed_positions: ClosedPositionList::new(),
            cash: 0.0
        }
    }

    pub fn from_file(config_file: &str) -> Result<Self, Error> {
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

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(config_str: &str) -> Result<Self, Error> {
        Self::parse(config_str)
    }

    #[inline(always)] pub fn ds_root(&self) -> &str { &self.ds_root }
    #[inline(always)] pub fn ds_name(&self) -> &str { &self.ds_name }

    #[inline(always)] pub fn stocks(&self) -> &StockList { &self.stocks }
    #[inline(always)] pub fn stocks_mut(&mut self) -> &mut StockList { &mut self.stocks }

    #[inline(always)] pub fn closed_positions(&self) -> &ClosedPositionList { &self.closed_positions }
    #[inline(always)] pub fn closed_positions_mut(&mut self) -> &mut ClosedPositionList { &mut self.closed_positions }

    #[inline(always)] pub fn cash(&self) -> Price { self.cash }

    // --------------------------------------------------------------------------------
    // Private Helpers

    fn parse(content: &str) -> Result<Self, Error> {
        let mut root: String = env::var("HOME")?;
        let mut name: String = String::from("sp_datastore");
        let mut stocks: Option<StockList> = None;
        let mut closed_positions: Option<ClosedPositionList> = None;
        let mut cash: Price = 0.0;

        let mut collect_scontent = false;
        let mut scontent_type = SContentType::None;
        let mut scontent = String::new();

        for line in content.lines() {
            if line.is_empty() { continue; }

            if line.trim() == "}" {
                match scontent_type {
                    SContentType::CSV => stocks = Some(StocksReader::parse_content(&scontent)?),
                    SContentType::CSVFile => stocks = Some(StocksReader::new(scontent.trim().to_string()).read()?),
                    SContentType::CSVCP => closed_positions = Some(ClosedPositionsReader::parse_content(&scontent)?),
                    SContentType::CSVFileCP => closed_positions = Some(ClosedPositionsReader::new(scontent.trim().to_string()).read()?),
                    SContentType::None => return Err("StocksConfig::parse - Unexpected scontent type None".into())
                };
                collect_scontent = false;
                scontent.clear();
                scontent_type = SContentType::None;
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
                "cash" => {
                    match value.parse::<Price>() {
                        Ok(v) => cash = v,
                        Err(e) => return Err(format!("StocksConfig::parse - {}", e).into())
                    };
                },
                "stocks" => {
                    collect_scontent = true;
                    match value {
                        "csv{" => scontent_type = SContentType::CSV,
                        "csv_file{" => scontent_type = SContentType::CSVFile,
                        _ => return Err(format!("StocksConfig::parse - Unsupported block type '{}'", value).into())
                    };
                },
                "closed_positions" => {
                    collect_scontent = true;
                    match value {
                        "csv{" => scontent_type = SContentType::CSVCP,
                        "csv_file{" => scontent_type = SContentType::CSVFileCP,
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
            stocks: stocks.unwrap_or_default(),
            closed_positions: closed_positions.unwrap_or_default(),
            cash: cash
        })
    }
}

impl Default for StocksConfig {
    fn default() -> Self {
        Self::new()
    }
}
