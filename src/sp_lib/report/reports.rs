use crate::util::error::Error;
use crate::report::report_params::ReportParams;
use crate::report::report_trait::Report;
use crate::report::report_type::ReportType;

use crate::report::rpt_closed_report::ClosedReport;
use crate::report::rpt_daych_report::DaychReport;
use crate::report::rpt_divid_report::DividReport;
use crate::report::rpt_top_report::TopReport;
use crate::report::rpt_value_report::ValueReport;
use crate::report::rpt_volat_report::VolatReport;

pub fn print_report(params: ReportParams) {
    make_report(params.rtype()).write(&params)
}

pub fn export_report(params: ReportParams, filename: &str) -> Result<(), Error> {
    make_report(params.rtype()).export(&params, filename)
}

fn make_report(rtype: ReportType) -> Box<dyn Report> {
    match rtype {
        ReportType::Value => Box::new(ValueReport{}),
        ReportType::Top => Box::new(TopReport{}),
        ReportType::Volat => Box::new(VolatReport{}),
        ReportType::Daych => Box::new(DaychReport{}),
        ReportType::Closed => Box::new(ClosedReport{}),
        ReportType::Divid => Box::new(DividReport{})
    }
}
