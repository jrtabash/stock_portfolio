use std::{env, fs};
use sp_lib::util::{datetime, temp_file};
use sp_lib::datastore::{datastore, history, dividends, export};

type Price = history::Price;

#[test]
fn test_datastore() {
    sp_ds_create();

    sp_ds_insert(history::tag(), 1);
    sp_ds_insert(history::tag(), 2);
    sp_ds_insert(dividends::tag(), 1);
    sp_ds_insert(dividends::tag(), 2);

    sp_ds_select_history();
    sp_ds_select_dividends();

    sp_ds_export_symbol();

    sp_ds_drop(dividends::tag());
    sp_ds_drop(history::tag());

    sp_ds_delete();
}

fn sp_ds_root() -> String {
    format!("{}", env::temp_dir().display())
}

fn sp_ds_name() -> &'static str {
    &"test_datastore"
}

fn sp_ds_symbol() -> &'static str {
    &"TEST"
}

fn sp_ds_data(which: &str, idx: i32) -> &'static str {
    if which == history::tag() {
        if idx == 1 {
            return &"Date,Open,High,Low,Close,Adj Close,Volume\n\
                     2021-02-22,10.0,12.0,8.0,11.0,11.0,10000\n\
                     2021-02-23,11.0,12.5,8.5,11.5,11.5,9000";
        }
        else if idx == 2 {
            return &"Date,Open,High,Low,Close,Adj Close,Volume\n\
                     2021-02-24,11.5,14.0,11.0,12.5,12.5,11000\n\
                     2021-02-25,12.5,13.5,10.5,12.0,12.0,10000\n\
                     2021-02-26,12.0,14.0,11.0,14.0,14.0,12000";
        }

    }
    else if which == dividends::tag() {
        if idx == 1 {
            return &"Date,Dividends\n\
                     2021-02-23,1.2";
        }
    }
    &""
}

fn sp_ds_create() {
    let ds = datastore::DataStore::new(&sp_ds_root(), sp_ds_name());
    assert!(!ds.exists());
    assert!(ds.create().is_ok());
}

fn sp_ds_delete() {
    let ds = datastore::DataStore::new(&sp_ds_root(), sp_ds_name());
    assert!(ds.exists());
    assert!(ds.delete().is_ok());
}

fn sp_ds_insert(which: &str, idx: i32) {
    let ds = datastore::DataStore::new(&sp_ds_root(), sp_ds_name());
    assert!(ds.exists());
    assert!(ds.insert_symbol(which, sp_ds_symbol(), sp_ds_data(which, idx)).is_ok());
}

fn sp_ds_select_history() {
    let ds = datastore::DataStore::new(&sp_ds_root(), sp_ds_name());
    assert!(ds.exists());

    fn check_history(entry: &history::HistoryEntry, csv: &str) {
        let values: Vec<&str> = csv.split(',').collect();
        assert_eq!(values.len(), 7);
        assert_eq!(entry.date, datetime::parse_date(&values[0]).unwrap());
        assert_eq!(entry.open, values[1].parse::<Price>().unwrap());
        assert_eq!(entry.high, values[2].parse::<Price>().unwrap());
        assert_eq!(entry.low, values[3].parse::<Price>().unwrap());
        assert_eq!(entry.close, values[4].parse::<Price>().unwrap());
        assert_eq!(entry.adj_close, values[5].parse::<Price>().unwrap());
        assert_eq!(entry.volume, values[6].parse::<u64>().unwrap());
    }

    // No Filter
    match history::History::ds_select_all(&ds, sp_ds_symbol()) {
        Ok(hist) => {
            assert_eq!(hist.symbol(), sp_ds_symbol());
            assert_eq!(hist.count(), 5);

            let entries = hist.entries();
            check_history(&entries[0], "2021-02-22,10.0,12.0,8.0,11.0,11.0,10000");
            check_history(&entries[1], "2021-02-23,11.0,12.5,8.5,11.5,11.5,9000");
            check_history(&entries[2], "2021-02-24,11.5,14.0,11.0,12.5,12.5,11000");
            check_history(&entries[3], "2021-02-25,12.5,13.5,10.5,12.0,12.0,10000");
            check_history(&entries[4], "2021-02-26,12.0,14.0,11.0,14.0,14.0,12000");
        },
        Err(_) => assert!(false)
    };

    // Filter
    match history::History::ds_select_if(&ds, sp_ds_symbol(), |entry| entry.open > 11.0 && entry.close > entry.open) {
        Ok(hist) => {
            assert_eq!(hist.symbol(), sp_ds_symbol());
            assert_eq!(hist.count(), 2);

            let entries = hist.entries();
            check_history(&entries[0], "2021-02-24,11.5,14.0,11.0,12.5,12.5,11000");
            check_history(&entries[1], "2021-02-26,12.0,14.0,11.0,14.0,14.0,12000");
        },
        Err(_) => assert!(false)
    };

    // Last
    match history::History::ds_select_last(&ds, sp_ds_symbol()) {
        Ok(hist) => {
            assert_eq!(hist.symbol(), sp_ds_symbol());
            assert_eq!(hist.count(), 1);

            let entries = hist.entries();
            check_history(&entries[0], "2021-02-26,12.0,14.0,11.0,14.0,14.0,12000");
        },
        Err(_) => assert!(false)
    };
}

fn sp_ds_select_dividends() {
    let ds = datastore::DataStore::new(&sp_ds_root(), sp_ds_name());
    assert!(ds.exists());

    fn check_dividend(entry: &dividends::DividendEntry, csv: &str) {
        let values: Vec<&str> = csv.split(',').collect();
        assert_eq!(values.len(), 2);
        assert_eq!(entry.date, datetime::parse_date(&values[0]).unwrap());
        assert_eq!(entry.price, values[1].parse::<Price>().unwrap());
    }

    // No Filter
    match dividends::Dividends::ds_select_all(&ds, sp_ds_symbol()) {
        Ok(div) => {
            assert_eq!(div.symbol(), sp_ds_symbol());
            assert_eq!(div.count(), 1);

            let entries = div.entries();
            check_dividend(&entries[0], "2021-02-23,1.2");
        },
        Err(_) => assert!(false)
    };

    // Filter
    match dividends::Dividends::ds_select_if(&ds, sp_ds_symbol(), |entry| entry.price > 1.5) {
        Ok(div) => {
            assert_eq!(div.symbol(), sp_ds_symbol());
            assert_eq!(div.count(), 0);
        },
        Err(_) => assert!(false)
    };

    // Last
    match dividends::Dividends::ds_select_last(&ds, sp_ds_symbol()) {
        Ok(div) => {
            assert_eq!(div.symbol(), sp_ds_symbol());
            assert_eq!(div.count(), 1);

            let entries = div.entries();
            check_dividend(&entries[0], "2021-02-23,1.2");
        },
        Err(_) => assert!(false)
    };
}

fn sp_ds_export_symbol() {
    let ds = datastore::DataStore::new(&sp_ds_root(), sp_ds_name());
    assert!(ds.exists());

    let csv_name = "sp_ds_export_symbol.csv";
    let csv_filename = temp_file::make_path(&csv_name);

    temp_file::remove_file(&csv_name);

    assert!(export::export_symbol(&ds, sp_ds_symbol(), &csv_filename.to_str().unwrap()).is_ok());

    let expect = "date,open,high,low,close,adj_close,volume,dividend\n\
                  2021-02-22,10.00,12.00,8.00,11.00,11.00,10000,0.00\n\
                  2021-02-23,11.00,12.50,8.50,11.50,11.50,9000,1.20\n\
                  2021-02-24,11.50,14.00,11.00,12.50,12.50,11000,0.00\n\
                  2021-02-25,12.50,13.50,10.50,12.00,12.00,10000,0.00\n\
                  2021-02-26,12.00,14.00,11.00,14.00,14.00,12000,0.00\n";
    let actual = fs::read_to_string(&csv_filename).unwrap();
    assert_eq!(actual, expect);

    assert!(temp_file::remove_file(&csv_name));
}

fn sp_ds_drop(which: &str) {
    let ds = datastore::DataStore::new(&sp_ds_root(), sp_ds_name());
    assert!(ds.exists());
    assert!(ds.drop_symbol(which, sp_ds_symbol()).is_ok());
}
