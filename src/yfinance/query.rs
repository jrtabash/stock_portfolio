use std::str;
use chrono::{Date, Local};
use curl;
use curl::easy;

use crate::yfinance::types::*;

// --------------------------------------------------------------------------------
// HistoryQuery

pub struct HistoryQuery {
    symbol: String,
    from_date: Date<Local>,
    to_date: Date<Local>,
    interval: Interval,
    events: Events,
    pub result: String
}

impl HistoryQuery {
    pub fn new(symbol: String,
               from_date: Date<Local>,
               to_date: Date<Local>,
               interval: Interval,
               events: Events) -> HistoryQuery {
        let result = String::new();
        HistoryQuery { symbol, from_date, to_date, interval, events, result }
    }

    pub fn url(self: &HistoryQuery) -> String {
        let base_url = "https://query1.finance.yahoo.com/v7/finance/download";
        let period1 = date2timestamp(&self.from_date);
        let period2 = date2timestamp(&self.to_date);
        let int_str = interval2str(self.interval);
        let evt_str = events2str(self.events);
        format!("{}/{}?period1={}&period2={}&interval={}&events={}&includeAdjustedClose=true",
                base_url,
                self.symbol,
                period1,
                period2,
                int_str,
                evt_str)
    }

    pub fn execute(self: &mut HistoryQuery) -> bool {
        self.result.clear();
        match url_request(&self.url(), &mut self.result) {
            Ok(_) => true,
            Err(e) => {
                println!("Error: {}", e);
                false
            }
        }
    }
}

// --------------------------------------------------------------------------------
// Private Helpers

fn date2timestamp(date: &Date<Local>) -> i64 {
    date.and_hms(0, 0, 0).timestamp()
}

fn url_request(url: &String, result: &mut String) -> Result<(), curl::Error> {
    let mut handle = easy::Easy::new();

    handle.url(url)?;

    let mut transfer = handle.transfer();
    transfer.write_function(|new_data| {
        let txt = str::from_utf8(new_data).unwrap_or("");
        result.push_str(txt);
        Ok(txt.len())
    })?;

    transfer.perform()?;

    Ok(())
}
