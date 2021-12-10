use crate::datastore::history::History;
use crate::stats::description::Description;

pub struct HistDesc {
    open: Description,
    high: Description,
    low: Description,
    close: Description,
    adj_close: Description,
    volume: Description
}

impl HistDesc {
    pub fn from_hist(hist: &History) -> Self {
        let entries = hist.entries();
        HistDesc {
            open: Description::from_vec(entries, |entry| entry.open),
            high: Description::from_vec(entries, |entry| entry.high),
            low: Description::from_vec(entries, |entry| entry.low),
            close: Description::from_vec(entries, |entry| entry.close),
            adj_close: Description::from_vec(entries, |entry| entry.adj_close),
            volume: Description::from_vec(entries, |entry| entry.volume as f64)
        }
    }

    #[inline(always)]
    pub fn open(&self) -> &Description {
        &self.open
    }

    #[inline(always)]
    pub fn high(&self) -> &Description {
        &self.high
    }

    #[inline(always)]
    pub fn low(&self) -> &Description {
        &self.low
    }

    #[inline(always)]
    pub fn close(&self) -> &Description {
        &self.close
    }

    #[inline(always)]
    pub fn adj_close(&self) -> &Description {
        &self.adj_close
    }

    #[inline(always)]
    pub fn volume(&self) -> &Description {
        &self.volume
    }

    pub fn print(&self, symbol: &str) {
        fn print_field(name: &str, hd: &HistDesc, extract: impl Fn(&Description) -> f64) {
            println!("{}: {:12.4} {:12.4} {:12.4} {:12.4} {:12.4} {:16.4}",
                     name,
                     extract(hd.open()),
                     extract(hd.high()),
                     extract(hd.low()),
                     extract(hd.close()),
                     extract(hd.adj_close()),
                     extract(hd.volume()));
        }

        println!("symbol: {}", symbol);
        println!(" field: {:>12} {:>12} {:>12} {:>12} {:>12} {:>16}", "open", "high", "low", "close", "adj_close", "volume");
        println!(" count: {:>12} {:>12} {:>12} {:>12} {:>12} {:>16}",
                 self.open().count(),
                 self.high().count(),
                 self.low().count(),
                 self.close().count(),
                 self.adj_close().count(),
                 self.volume().count());
        print_field("   min", &self, |desc| desc.min());
        print_field("   max", &self, |desc| desc.max());
        print_field("  mean", &self, |desc| desc.mean());
        print_field("   std", &self, |desc| desc.stddev());
    }
}

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hist_desc() {
        let hist = hist_data();
        let hdesc = HistDesc::from_hist(&hist);

        assert_eq!(hdesc.open().count(), 21);
        assert!(value_eql(hdesc.open().min(), 139.470001));
        assert!(value_eql(hdesc.open().max(), 149.820007));
        assert!(value_eql(hdesc.open().mean(), 144.971428));
        assert!(value_eql(hdesc.open().stddev(), 3.580718));

        assert_eq!(hdesc.high().count(), 21);
        assert!(value_eql(hdesc.high().min(), 141.399994));
        assert!(value_eql(hdesc.high().max(), 153.169998));
        assert!(value_eql(hdesc.high().mean(), 146.418570));
        assert!(value_eql(hdesc.high().stddev(), 3.623685));

        assert_eq!(hdesc.low().count(), 21);
        assert!(value_eql(hdesc.low().min(), 138.270004));
        assert!(value_eql(hdesc.low().max(), 149.720001));
        assert!(value_eql(hdesc.low().mean(), 143.954761));
        assert!(value_eql(hdesc.low().stddev(), 3.963953));

        assert_eq!(hdesc.close().count(), 21);
        assert!(value_eql(hdesc.close().min(), 139.139999));
        assert!(value_eql(hdesc.close().max(), 152.570007));
        assert!(value_eql(hdesc.close().mean(), 145.563809));
        assert!(value_eql(hdesc.close().stddev(), 3.843008));

        assert_eq!(hdesc.adj_close().count(), 21);
        assert!(value_eql(hdesc.adj_close().min(), 138.937225));
        assert!(value_eql(hdesc.adj_close().max(), 152.347656));
        assert!(value_eql(hdesc.adj_close().mean(), 145.351675));
        assert!(value_eql(hdesc.adj_close().stddev(), 3.837406));

        assert_eq!(hdesc.volume().count(), 21);
        assert!(value_eql(hdesc.volume().min(), 50720600.0));
        assert!(value_eql(hdesc.volume().max(), 124850400.0));
        assert!(value_eql(hdesc.volume().mean(), 74517466.666666));
        assert!(value_eql(hdesc.volume().stddev(), 18393701.437620));
    }

    fn hist_data() -> History {
        History::parse_csv(
            "AAPL",
            "2021-10-01,141.899994,142.919998,139.110001,142.649994,142.442108,94639600\n\
             2021-10-04,141.759995,142.210007,138.270004,139.139999,138.937225,98322000\n\
             2021-10-05,139.490005,142.240005,139.360001,141.110001,140.904358,80861100\n\
             2021-10-06,139.470001,142.149994,138.369995,142.000000,141.793060,83221100\n\
             2021-10-07,143.059998,144.220001,142.720001,143.289993,143.081177,61732700\n\
             2021-10-08,144.029999,144.179993,142.559998,142.899994,142.691742,58718700\n\
             2021-10-11,142.270004,144.809998,141.809998,142.809998,142.601883,64452200\n\
             2021-10-12,143.229996,143.250000,141.039993,141.509995,141.303772,73035900\n\
             2021-10-13,141.240005,141.399994,139.199997,140.910004,140.704651,78762700\n\
             2021-10-14,142.110001,143.880005,141.509995,143.759995,143.550491,69907100\n\
             2021-10-15,143.770004,144.899994,143.509995,144.839996,144.628922,67885200\n\
             2021-10-18,143.449997,146.839996,143.160004,146.550003,146.336426,85589200\n\
             2021-10-19,147.009995,149.169998,146.550003,148.759995,148.543198,76378900\n\
             2021-10-20,148.699997,149.750000,148.119995,149.259995,149.042480,58418800\n\
             2021-10-21,148.809998,149.639999,147.869995,149.479996,149.262146,61421000\n\
             2021-10-22,149.690002,150.179993,148.639999,148.690002,148.473312,58883400\n\
             2021-10-25,148.679993,149.369995,147.619995,148.639999,148.423386,50720600\n\
             2021-10-26,149.330002,150.839996,149.009995,149.320007,149.102402,60893400\n\
             2021-10-27,149.360001,149.729996,148.490005,148.850006,148.633087,56094900\n\
             2021-10-28,149.820007,153.169998,149.720001,152.570007,152.347656,100077900\n\
             2021-10-29,147.220001,149.940002,146.410004,149.800003,149.581696,124850400").unwrap()
    }

    fn value_eql(lhs: f64, rhs: f64) -> bool {
        (lhs - rhs).abs() < 0.000001
    }
}
