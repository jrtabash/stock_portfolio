use std::error::Error;
use chrono::{Date, Local, Duration, NaiveDate, Datelike, TimeZone};

pub fn date2timestamp(date: &Date<Local>) -> i64 {
    date.and_hms(0, 0, 0).timestamp()
}

pub fn today_plus_days(days: i64) -> Date<Local> {
    Local::today() + Duration::days(days)
}

pub fn earliest_date() -> Date<Local> {
    chrono::Local.ymd(1970, 1, 1)
}

pub fn parse_date(date_str: &str) -> Result<Date<Local>, Box<dyn Error>> {
    match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
        Ok(dt) => Ok(chrono::Local.ymd(dt.year(), dt.month(), dt.day())),
        Err(e) => Result::Err(format!("parse_date: {}", e).into())
    }
}
