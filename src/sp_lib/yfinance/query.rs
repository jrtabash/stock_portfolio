use std::str;
use curl;
use curl::easy;

use crate::yfinance::types;
use crate::yfinance::types::{Interval, Events};
use crate::util::datetime;
use crate::util::error::Error;
use crate::util::datetime::SPDate;

// --------------------------------------------------------------------------------
// HistoryQuery

pub struct HistoryQuery {
    symbol: String,
    from_date: SPDate,
    to_date: SPDate,
    interval: Interval,
    events: Events,
    pub result: String
}

impl HistoryQuery {
    pub fn new(symbol: String,
               from_date: SPDate,
               to_date: SPDate,
               interval: Interval,
               events: Events) -> HistoryQuery {
        HistoryQuery {
            symbol,
            from_date,
            to_date,
            interval,
            events,
            result: String::new()
        }
    }

    pub fn url(self: &HistoryQuery) -> String {
        let base_url = "https://query1.finance.yahoo.com/v7/finance/download";
        let period1 = datetime::date2timestamp(&self.from_date);
        let period2 = datetime::date2timestamp(&self.to_date);
        let int_str = types::interval2str(self.interval);
        let evt_str = types::events2str(self.events);
        format!("{}/{}?period1={}&period2={}&interval={}&events={}&includeAdjustedClose=true",
                base_url,
                self.symbol,
                period1,
                period2,
                int_str,
                evt_str)
    }

    pub fn execute(self: &mut HistoryQuery) -> Result<(), Error> {
        self.result.clear();
        url_request(&self.url(), &mut self.result)?;
        Ok(())
    }
}

// --------------------------------------------------------------------------------
// Private Helpers

fn url_request(url: &str, result: &mut String) -> Result<(), Error> {
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
