use std::error::Error;
use std::fmt;

#[derive(Debug, Copy, Clone)]
#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub enum ReportType {
    Value, // Value (Gains & Losses)
    Top    // Top/Last Performing
}

pub fn reporttype2str(rt: ReportType) -> &'static str {
    match rt {
        ReportType::Value => "value",
        ReportType::Top => "top"
    }
}

pub fn str2reporttype(rtstr: &str) -> Result<ReportType, Box<dyn Error>> {
    match rtstr.to_lowercase().as_str() {
        "value" => Ok(ReportType::Value),
        "top" => Ok(ReportType::Top),
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
        let value_str = "value";
        let top_str = "top";

        assert_eq!(reporttype2str(value), value_str);
        assert_eq!(reporttype2str(top), top_str);
        assert!(str2reporttype(&value_str).unwrap() == value);
        assert!(str2reporttype(&top_str).unwrap() == top);

        match str2reporttype("foobar") {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(format!("{}", err), "Unknown report type 'foobar'")
        };
    }
}
