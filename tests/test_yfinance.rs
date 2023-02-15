use sp_lib::util::datetime;
use sp_lib::yfinance::types::*;
use sp_lib::yfinance::query::*;

#[test]
fn test_history_query() {
    let start = datetime::make_date(2021, 2, 11);
    let end = datetime::make_date(2021, 2, 13);
    let mut query = HistoryQuery::new(String::from("AAPL"), start, end, Interval::Daily, Events::History);

    assert_eq!(query.execute(), Ok(()));
    assert!(query.result.len() > 0);

    let result_vec: Vec<&str> = query.result.lines().collect();
    assert_eq!(result_vec.len(), 3);
    assert_eq!(result_vec[0], "Date,Open,High,Low,Close,Adj Close,Volume");

    let prices_vec: Vec<&str> = result_vec[1].split(",").collect();
    assert_eq!(prices_vec.len(), 7);
    assert_eq!(prices_vec[0], "2021-02-11");
    assert!(prices_vec[6].parse::<u32>().unwrap() >= 64000000);
    check_prices(&prices_vec, &vec![135.90, 136.39, 133.77, 135.13, 133.75]);

    let prices_vec: Vec<&str> = result_vec[2].split(",").collect();
    assert_eq!(prices_vec.len(), 7);
    assert_eq!(prices_vec[0], "2021-02-12");
    assert!(prices_vec[6].parse::<u32>().unwrap() >= 60000000);
    check_prices(&prices_vec, &vec![134.35, 135.53, 133.69, 135.37, 134.00]);
}

fn check_prices(actual: &Vec<&str>, expect: &Vec<f64>) {
    assert_eq!(actual.len(), 7);
    assert_eq!(expect.len(), 5);

    for i in 1..6 {
        let act = actual[i].parse().unwrap_or(0.0);
        assert!((act - expect[i-1]).abs() < 0.5);
    }
}
