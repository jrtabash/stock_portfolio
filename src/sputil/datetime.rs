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
        Err(e) => Err(format!("parse_date: {}", e).into())
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

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_today() {
        let date = today();
        assert_eq!(date, Local::today());
    }

    #[test]
    fn test_make_date() {
        let date = make_date(2021, 2, 17);
        assert_eq!(date.year(), 2021);
        assert_eq!(date.month(), 2);
        assert_eq!(date.day(), 17);
    }

    #[test]
    fn test_date2timestamp() {
        let date = make_date(2021, 2, 17);
        assert_eq!(date2timestamp(&date), 1613541600);
    }

    #[test]
    fn test_today_plus_delta() {
        let today = today();
        assert_eq!(today_plus_days(0), today);
        assert_eq!(today_plus_days(1), today + Duration::days(1));
        assert_eq!(today_plus_days(-1), today + Duration::days(-1));
    }

    #[test]
    fn test_earliest_date() {
        let earliest = earliest_date();
        assert_eq!(earliest.year(), 1970);
        assert_eq!(earliest.month(), 1);
        assert_eq!(earliest.day(), 1);
    }

    #[test]
    fn test_parse_date() {
        fn test(date: LocalDate) {
            let date_str = format!("{:04}-{:02}-{:02}", date.year(), date.month(), date.day());
            match parse_date(&date_str) {
                Ok(dt) => assert_eq!(dt, date),
                Err(_) => assert!(false)
            }
        }

        test(today());
        test(today_plus_days(-1));
        test(today_plus_days(1));
        test(today_plus_days(-30));
        test(today_plus_days(30));
    }

    #[test]
    fn test_parse_date_error() {
        match parse_date("20211701") {
            Ok(_) => assert!(false),
            Err(error) => assert!(format!("{}", error).starts_with("parse_date: "))
        }
    }

    #[test]
    fn test_is_friday() {
        let thu = make_date(2021, 3, 18);
        let fri = make_date(2021, 3, 19);
        let sat = make_date(2021, 3, 20);

        assert!(!is_friday(&thu));
        assert!(is_friday(&fri));
        assert!(!is_friday(&sat));
    }

    #[test]
    fn test_is_weekend() {
        let fri = make_date(2021, 3, 19);
        let sat = make_date(2021, 3, 20);
        let sun = make_date(2021, 3, 21);
        let mon = make_date(2021, 3, 22);

        assert!(!is_weekend(&fri));
        assert!(is_weekend(&sat));
        assert!(is_weekend(&sun));
        assert!(!is_weekend(&mon));
    }
}
