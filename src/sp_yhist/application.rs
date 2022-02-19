use std::error::Error;
use sp_lib::util::datetime;
use sp_lib::yfinance::{types, query};
use crate::arguments::Arguments;

const EVT_HISTORY: &str = "history";
const EVT_DIVIDEND: &str = "dividend";
const EVT_SPLIT: &str = "split";

const INT_DAY: &str = "day";
const INT_WEEK: &str = "week";
const INT_MONTH: &str = "month";

pub struct Application {
    args: Arguments
}

impl Application {
    pub fn new() -> Self {
        Application {
            args: Arguments::new()
        }
    }

    pub fn run(self: &mut Self) -> Result<(), Box<dyn Error>> {
        let from_date = self.args.from().unwrap_or_else(
            || datetime::today_plus_days(-7));
        let to_date = datetime::date_plus_days(
            &self.args.to().unwrap_or_else(datetime::today),
            1);

        if from_date <= to_date {
            let mut query = query::HistoryQuery::new(
                self.args.symbol().to_string(),
                from_date,
                to_date,
                Self::str2int(self.args.interval())?,
                Self::str2evts(self.args.events())?);
            query.execute()?;
            println!("{}", query.result);
            Ok(())
        }
        else {
            Err(format!("To date is greater than from date").into())
        }
    }

    pub fn str2evts(estr: &str) -> Result<types::Events, Box<dyn Error>> {
        match estr {
            EVT_HISTORY => Ok(types::Events::History),
            EVT_DIVIDEND => Ok(types::Events::Dividend),
            EVT_SPLIT => Ok(types::Events::Split),
            _ => {
                Err(format!("Invalid events").into())
            }
        }
    }

    pub fn str2int(istr: &str) -> Result<types::Interval, Box<dyn Error>> {
        match istr {
            INT_DAY => Ok(types::Interval::Daily),
            INT_WEEK => Ok(types::Interval::Weekly),
            INT_MONTH => Ok(types::Interval::Monthly),
            _ => {
                Err(format!("Invalid interval").into())
            }
        }
    }
}
