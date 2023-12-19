use crate::report::report_params::ReportParams;
use crate::util::error::Error;

pub trait Report {
    fn write(&self, params: &ReportParams);
    fn export(&self, params: &ReportParams, filename: &str) -> Result<(), Error>;
}
