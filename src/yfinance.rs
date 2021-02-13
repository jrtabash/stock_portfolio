use std::str;
use chrono::{Date, Local};
use curl::easy::Easy;

// --------------------------------------------------------------------------------
// Events

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Events {
    History,
    Dividend,
    Split
}

fn events2str(evt: Events) -> &'static str {
    match evt {
        Events::History => "history",
        Events::Dividend => "div",
        Events::Split => "split"
    }
}

// --------------------------------------------------------------------------------
// Interval

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Interval {
    Daily,
    Weekly,
    Monthly
}

fn interval2str(int: Interval) -> &'static str {
    match int {
        Interval::Daily => "1d",
        Interval::Weekly => "1wk",
        Interval::Monthly => "1mo"
    }
}

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

    pub fn url(self: & HistoryQuery) -> String {
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

    pub fn execute(self: & mut HistoryQuery) -> bool {
        self.result.clear();
        url_request(&self.url(), &mut self.result)
    }
}

// --------------------------------------------------------------------------------
// Private Helpers

fn date2timestamp(date: &Date<Local>) -> i64 {
    date.and_hms(0, 0, 0).timestamp()
}

fn url_request(url: &String, result: &mut String) -> bool {
    let mut handle = Easy::new();

    match handle.url(url) {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {}", e);
            return false
        }
    }

    let mut transfer = handle.transfer();
    let res = transfer.write_function(|new_data| {
        let txt = str::from_utf8(new_data).unwrap_or("");
        result.push_str(txt);
        Ok(txt.len())
    });
    match res {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {}", e);
            return false
        }
    }

    match transfer.perform() {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {}", e);
            return false
        }
    }

    return true
}

// --------------------------------------------------------------------------------
// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_events() {
        assert_eq!(events2str(Events::History), "history");
        assert_eq!(events2str(Events::Dividend), "div");
        assert_eq!(events2str(Events::Split), "split");
    }

    #[test]
    fn test_interval() {
        assert_eq!(interval2str(Interval::Daily), "1d");
        assert_eq!(interval2str(Interval::Weekly), "1wk");
        assert_eq!(interval2str(Interval::Monthly), "1mo");
    }

    #[test]
    fn test_history_query() {
        let start = chrono::Local.ymd(2021, 2, 11);
        let end = chrono::Local.ymd(2021, 2, 13);
        let mut query = HistoryQuery::new(String::from("AAPL"), start, end, Interval::Daily, Events::History);

        assert!(query.execute());
        assert!(query.result.len() > 0);

        let result_vec: Vec<&str> = query.result.split("\n").collect();
        assert_eq!(result_vec.len(), 3);
        assert_eq!(result_vec[0], "Date,Open,High,Low,Close,Adj Close,Volume");

        let prices_vec: Vec<&str> = result_vec[1].split(",").collect();
        assert_eq!(prices_vec.len(), 7);
        assert_eq!(prices_vec[0], "2021-02-11");
        assert_eq!(prices_vec[6], "64154400");
        check_prices(&prices_vec, &vec!["135.90", "136.39", "133.77", "135.13", "135.13"]);

        let prices_vec: Vec<&str> = result_vec[2].split(",").collect();
        assert_eq!(prices_vec.len(), 7);
        assert_eq!(prices_vec[0], "2021-02-12");
        assert_eq!(prices_vec[6], "60029300");
        check_prices(&prices_vec, &vec!["134.35", "135.53", "133.69", "135.37", "135.37"]);
    }

    fn check_prices(actual: &Vec<&str>, expect: &Vec<&str>) {
        assert_eq!(actual.len(), 7);
        assert_eq!(expect.len(), 5);

        for i in 1..6 {
            let px = format!("{:.2}", actual[i].parse().unwrap_or(0.0));
            assert_eq!(px, expect[i-1]);
        }
    }
}
