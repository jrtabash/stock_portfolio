use crate::datastore::datastore::DataStore;
use crate::datastore::history::History;
use crate::stats::hist_ftns;
use crate::portfolio::algorithms;
use crate::portfolio::stock::StockList;

pub fn extra_sort_ftn(order_by: &str) -> Option<fn(&DataStore, &mut StockList, bool)> {
    match order_by {
        "volat"   => Some(extra_sort_by_volat),    // orderall volatility
        "volat22" => Some(extra_sort_by_volat22),  // 22 day volatility
        "change"  => Some(extra_sort_by_change),   // day change
        "pctchg"  => Some(extra_sort_by_pctchg),   // day percent change
        "prevpr"  => Some(extra_sort_by_prevpr),   // previous day price
        "low"     => Some(extra_sort_by_low),      // day low price
        "high"    => Some(extra_sort_by_high),     // day high price
        "volume"  => Some(extra_sort_by_volume),   // day volume
        _ => None
    }
}

pub fn extra_sort_by_volat(ds: &DataStore, stocks: &mut StockList, desc: bool) {
    algorithms::sort_stocks_by_extra_ftn(
        stocks,
        |stock| -> f64 {
            if let Ok(hist) = History::ds_select_if(ds, &stock.symbol, |e| e.date >= stock.date) {
                if let Ok(volat) = hist_ftns::hist_volatility(&hist) {
                    return volat
                }
            }
            0.0
        },
        desc);
}

pub fn extra_sort_by_volat22(ds: &DataStore, stocks: &mut StockList, desc: bool) {
    algorithms::sort_stocks_by_extra_ftn(
        stocks,
        |stock| -> f64 {
            if let Ok(hist) = History::ds_select_if(ds, &stock.symbol, |e| e.date >= stock.date) {
                if hist.count() >= 22 {
                    let start_idx = hist.count() - 22;
                    if let Ok(volat) = hist_ftns::entries_volatility(&hist.entries()[start_idx..]) {
                        return volat
                    }
                }
            }
            0.0
        },
        desc);
}

pub fn extra_sort_by_change(ds: &DataStore, stocks: &mut StockList, desc: bool) {
    algorithms::sort_stocks_by_extra_ftn(
        stocks,
        |stock| -> f64 {
            if let Ok(hist) = History::ds_select_last_n(ds, &stock.symbol, 2) {
                let entries = hist.entries();
                if entries.len() == 2 {
                    let delta = entries[1].adj_close - entries[0].adj_close;
                    return delta
                }
            }
            0.0
        },
        desc);
}

pub fn extra_sort_by_pctchg(ds: &DataStore, stocks: &mut StockList, desc: bool) {
    algorithms::sort_stocks_by_extra_ftn(
        stocks,
        |stock| -> f64 {
            if let Ok(hist) = History::ds_select_last_n(ds, &stock.symbol, 2) {
                let entries = hist.entries();
                if entries.len() == 2 {
                    let prev_price = entries[0].adj_close;
                    let delta = entries[1].adj_close - prev_price;
                    let pct = 100.0 * if prev_price > 0.0 { delta / prev_price } else { 0.00 };
                    return pct
                }
            }
            0.0
        },
        desc);
}

pub fn extra_sort_by_prevpr(ds: &DataStore, stocks: &mut StockList, desc: bool) {
    algorithms::sort_stocks_by_extra_ftn(
        stocks,
        |stock| -> f64 {
            if let Ok(hist) = History::ds_select_last_n(ds, &stock.symbol, 2) {
                let entries = hist.entries();
                if entries.len() == 2 {
                    return entries[0].adj_close
                }
            }
            0.0
        },
        desc);
}

pub fn extra_sort_by_low(ds: &DataStore, stocks: &mut StockList, desc: bool) {
    algorithms::sort_stocks_by_extra_ftn(
        stocks,
        |stock| -> f64 {
            if let Ok(hist) = History::ds_select_last(ds, &stock.symbol) {
                let entries = hist.entries();
                if entries.len() == 1 {
                    return entries[0].low
                }
            }
            0.0
        },
        desc);
}

pub fn extra_sort_by_high(ds: &DataStore, stocks: &mut StockList, desc: bool) {
    algorithms::sort_stocks_by_extra_ftn(
        stocks,
        |stock| -> f64 {
            if let Ok(hist) = History::ds_select_last(ds, &stock.symbol) {
                let entries = hist.entries();
                if entries.len() == 1 {
                    return entries[0].high
                }
            }
            0.0
        },
        desc);
}

pub fn extra_sort_by_volume(ds: &DataStore, stocks: &mut StockList, desc: bool) {
    algorithms::sort_stocks_by_extra_ftn(
        stocks,
        |stock| -> f64 {
            if let Ok(hist) = History::ds_select_last(ds, &stock.symbol) {
                let entries = hist.entries();
                if entries.len() == 1 {
                    return entries[0].volume as f64
                }
            }
            0.0
        },
        desc);
}
