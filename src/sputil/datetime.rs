use chrono::{Date, Local, Duration};

pub fn date2timestamp(date: &Date<Local>) -> i64 {
    date.and_hms(0, 0, 0).timestamp()
}

pub fn today_plus_days(days: i64) -> Date<Local> {
    Local::today() + Duration::days(days)
}
