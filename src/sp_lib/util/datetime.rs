use std::error::Error;
use chrono::{Local, Duration, NaiveDate, NaiveDateTime, Datelike, Weekday};

pub type SPDate = NaiveDate;

#[inline(always)]
pub fn date2timestamp(date: &SPDate) -> i64 {
    date.and_hms_opt(0, 0, 0).unwrap_or(NaiveDateTime::MIN).timestamp()
}

#[inline(always)]
pub fn make_date(year: i32, month: u32, day: u32) -> SPDate {
    NaiveDate::from_ymd_opt(year, month, day).unwrap_or(NaiveDate::MIN)
}

#[inline(always)]
pub fn today() -> SPDate {
    Local::now().date_naive()
}

#[inline(always)]
pub fn today_plus_days(days: i64) -> SPDate {
    today() + Duration::days(days)
}

#[inline(always)]
pub fn date_plus_days(date: &SPDate, days: i64) -> SPDate {
    *date + Duration::days(days)
}

#[inline(always)]
pub fn earliest_date() -> SPDate {
    make_date(1970, 1, 1)
}

pub fn parse_date(date_str: &str) -> Result<SPDate, Box<dyn Error>> {
    match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
        Ok(dt) => Ok(make_date(dt.year(), dt.month(), dt.day())),
        Err(e) => Err(format!("parse_date: {}", e).into())
    }
}

#[inline(always)]
pub fn is_monday(date: &SPDate) -> bool {
    date.weekday() == Weekday::Mon
}

#[inline(always)]
pub fn is_friday(date: &SPDate) -> bool {
    date.weekday() == Weekday::Fri
}

#[inline(always)]
pub fn is_weekend(date: &SPDate) -> bool {
    date.weekday() == Weekday::Sat || date.weekday() == Weekday::Sun
}

#[inline(always)]
pub fn count_days(from_date: &SPDate, to_date: &SPDate) -> i64 {
    to_date.signed_duration_since(from_date.clone()).num_days()
}

pub fn check_dup_or_back_gap(old_date: &SPDate, new_date: &SPDate) -> Result<(), Box<dyn Error>> {
    if old_date == new_date {
        return Err(format!("Duplicate date {}", new_date.format("%Y-%m-%d")).into())
    }
    else if new_date < old_date {
        return Err(format!("Earlier date {}", new_date.format("%Y-%m-%d")).into())
    }
    Ok(())
}

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_today() {
        let date = today();
        assert_eq!(date, Local::now().date_naive());
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
        assert_eq!(date2timestamp(&date), 1613520000);
    }

    #[test]
    fn test_today_plus_delta() {
        let today = today();
        assert_eq!(today_plus_days(0), today);
        assert_eq!(today_plus_days(1), today + Duration::days(1));
        assert_eq!(today_plus_days(-1), today + Duration::days(-1));
    }

    #[test]
    fn test_date_plus_delta() {
        let dt = today();
        assert_eq!(date_plus_days(&dt, 0), dt);
        assert_eq!(date_plus_days(&dt, 1), dt + Duration::days(1));
        assert_eq!(date_plus_days(&dt, -1), dt + Duration::days(-1));
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
        fn test(date: SPDate) {
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
    fn test_is_monday() {
        let sun = make_date(2021, 3, 21);
        let mon = make_date(2021, 3, 22);
        let tue = make_date(2021, 3, 23);

        assert!(!is_monday(&sun));
        assert!(is_monday(&mon));
        assert!(!is_monday(&tue));
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

    #[test]
    fn test_count_days() {
        let dt1 = make_date(2021, 3, 19);
        let dt2 = make_date(2021, 3, 20);
        let dt3 = make_date(2021, 3, 21);

        assert_eq!(count_days(&dt1, &dt1), 0);
        assert_eq!(count_days(&dt1, &dt2), 1);
        assert_eq!(count_days(&dt1, &dt3), 2);
    }

    #[test]
    fn test_check_dup_or_back_gap() {
        let dt1 = make_date(2021, 3, 19);
        let dt2 = make_date(2021, 3, 19);
        let dt3 = make_date(2021, 3, 20);

        assert!(check_dup_or_back_gap(&dt1, &dt3).is_ok());

        match check_dup_or_back_gap(&dt1, &dt2) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(&format!("{}", err), "Duplicate date 2021-03-19")
        };

        match check_dup_or_back_gap(&dt3, &dt2) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(&format!("{}", err), "Earlier date 2021-03-19")
        };
    }
}
