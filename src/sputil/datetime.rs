use std::error::Error;
use chrono::{Date, Local, Duration, NaiveDate, Datelike, TimeZone, Weekday};

pub type LocalDate = Date<Local>;

#[inline(always)]
pub fn date2timestamp(date: &LocalDate) -> i64 {
    date.and_hms(0, 0, 0).timestamp()
}

#[inline(always)]
pub fn make_date(year: i32, month: u32, day: u32) -> LocalDate {
    chrono::Local.ymd(year, month, day)
}

#[inline(always)]
pub fn today() -> LocalDate {
    Local::today()
}

#[inline(always)]
pub fn today_plus_days(days: i64) -> LocalDate {
    Local::today() + Duration::days(days)
}

#[inline(always)]
pub fn earliest_date() -> LocalDate {
    chrono::Local.ymd(1970, 1, 1)
}

pub fn parse_date(date_str: &str) -> Result<LocalDate, Box<dyn Error>> {
    match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
        Ok(dt) => Ok(chrono::Local.ymd(dt.year(), dt.month(), dt.day())),
        Err(e) => Result::Err(format!("parse_date: {}", e).into())
    }
}

#[inline(always)]
pub fn is_friday(date: &LocalDate) -> bool {
    date.weekday() == Weekday::Fri
}

#[inline(always)]
pub fn is_weekend(date: &LocalDate) -> bool {
    date.weekday() == Weekday::Sat || date.weekday() == Weekday::Sun
}
