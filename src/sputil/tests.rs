#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Local, Duration};
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
}
