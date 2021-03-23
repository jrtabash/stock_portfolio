#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use chrono::{Duration, Datelike};
    use stock_portfolio::sputil::datetime::*;
    use stock_portfolio::sputil::price_type::*;

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
    fn test_price_type_price_cmp() {
        assert_eq!(price_cmp(10.50, 1.0), Ordering::Greater);
        assert_eq!(price_cmp(1.0, 10.50), Ordering::Less);
        assert_eq!(price_cmp(1.0, 1.0), Ordering::Equal);
    }
}
