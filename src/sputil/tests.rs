#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Local, Duration, Date, Datelike};
    use crate::sputil::datetime::*;

    #[test]
    fn test_date2timestamp() {
        let date = chrono::Local.ymd(2021, 2, 17);
        assert_eq!(date2timestamp(&date), 1613541600);
    }

    #[test]
    fn test_today_plus_delta() {
        let today = Local::today();
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
        fn test(date: Date<Local>) {
            let date_str = format!("{:04}-{:02}-{:02}", date.year(), date.month(), date.day());
            match parse_date(&date_str) {
                Ok(dt) => assert_eq!(dt, date),
                Err(_) => assert!(false)
            }
        }

        test(Local::today());
        test(today_plus_days(-1));
        test(today_plus_days(1));
        test(today_plus_days(-30));
        test(today_plus_days(30));
    }
}
