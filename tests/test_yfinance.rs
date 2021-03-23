#[cfg(test)]
mod tests {
    use stock_portfolio::sputil::datetime;
    use stock_portfolio::yfinance::types::*;
    use stock_portfolio::yfinance::query::*;

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
        let start = datetime::make_date(2021, 2, 11);
        let end = datetime::make_date(2021, 2, 13);
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
