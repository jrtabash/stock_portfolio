use std::fmt;
use crate::util::error::Error;

#[derive(Debug, Copy, Clone)]
#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub enum ReportType {
    Value,  // Value (Gains & Losses)
    Top,    // Top/Last Performing
    Volat,  // Volatility
    Daych,  // Day Change
    Closed, // Closed Positions Value
    Divid,  // Dividend
    Sum,    // Summary
}

pub fn reporttype2str(rt: ReportType) -> &'static str {
    match rt {
        ReportType::Value => "value",
        ReportType::Top => "top",
        ReportType::Volat => "volat",
        ReportType::Daych => "daych",
        ReportType::Closed => "closed",
        ReportType::Divid => "divid",
        ReportType::Sum => "sum"
    }
}

pub fn str2reporttype(rtstr: &str) -> Result<ReportType, Error> {
    match rtstr.to_lowercase().as_str() {
        "value" => Ok(ReportType::Value),
        "top" => Ok(ReportType::Top),
        "volat" => Ok(ReportType::Volat),
        "daych" => Ok(ReportType::Daych),
        "closed" => Ok(ReportType::Closed),
        "divid" => Ok(ReportType::Divid),
        "sum" => Ok(ReportType::Sum),
        _ => Err(format!("Unknown report type '{}'", rtstr).into())
    }
}

impl fmt::Display for ReportType {
    fn fmt(self: &ReportType, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", reporttype2str(*self))
    }
}

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_type() {
        let value = ReportType::Value;
        let top = ReportType::Top;
        let volat = ReportType::Volat;
        let daych = ReportType::Daych;
        let closed = ReportType::Closed;
        let divid = ReportType::Divid;
        let sum = ReportType::Sum;
        let value_str = "value";
        let top_str = "top";
        let volat_str = "volat";
        let daych_str = "daych";
        let closed_str = "closed";
        let divid_str = "divid";
        let sum_str = "sum";

        assert_eq!(reporttype2str(value), value_str);
        assert_eq!(reporttype2str(top), top_str);
        assert_eq!(reporttype2str(volat), volat_str);
        assert_eq!(reporttype2str(daych), daych_str);
        assert_eq!(reporttype2str(closed), closed_str);
        assert_eq!(reporttype2str(divid), divid_str);
        assert_eq!(reporttype2str(sum), sum_str);
        assert!(str2reporttype(&value_str).unwrap() == value);
        assert!(str2reporttype(&top_str).unwrap() == top);
        assert!(str2reporttype(&volat_str).unwrap() == volat);
        assert!(str2reporttype(&daych_str).unwrap() == daych);
        assert!(str2reporttype(&closed_str).unwrap() == closed);
        assert!(str2reporttype(&divid_str).unwrap() == divid);
        assert!(str2reporttype(&sum_str).unwrap() == sum);

        match str2reporttype("foobar") {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(format!("{}", err), "Unknown report type 'foobar'")
        };
    }
}
