use std::fs::File;
use std::io::prelude::*;

use crate::portfolio::stock::Price;
use crate::report::report_params::ReportParams;
use crate::report::report_trait::Report;
use crate::util::datetime;
use crate::util::error::Error;

pub struct ClosedReport {}

impl Report for ClosedReport {
    fn write(&self, params: &ReportParams) {
        let positions = params.closed_positions();

        let mut base_ntnl: Price = 0.0;
        let mut exit_ntnl: Price = 0.0;
        let mut net_ntnl: Price = 0.0;
        let mut tot_fees: Price = 0.0;
        let mut tot_div: Price = 0.0;

        for pos in positions.iter() {
            base_ntnl += pos.base_notional();
            exit_ntnl += pos.exit_notional();
            net_ntnl += pos.net_notional();
            tot_fees += pos.base_fee + pos.exit_fee;
            tot_div += pos.dividend;
        }

        println!("Closed Positions Report");
        println!("-----------------------");
        println!("            Date: {}", datetime::today().format("%Y-%m-%d"));
        println!("Total Base Value: {:.2}", base_ntnl);
        println!("Total Exit Value: {:.2}", exit_ntnl);
        println!(" Total Net Value: {:.2}", net_ntnl);
        println!("      Total Fees: {:.2}", tot_fees);
        println!("  Total Dividend: {:.2}", tot_div);
        println!("Net + Div - Fees: {:.2}", net_ntnl + tot_div - tot_fees);
        println!();

        println!("{:8} {:10} {:10} {:12} {:12} {:12} {:6} {:10}",
                 "Symbol",
                 "Base Date",
                 "Exit Date",
                 "Base Value",
                 "Exit Value",
                 "Net Value",
                 "Fees",
                 "Dividend");
        println!("{:8} {:10} {:10} {:12} {:12} {:12} {:6} {:10}",
                 "------",
                 "---------",
                 "---------",
                 "----------",
                 "----------",
                 "---------",
                 "----",
                 "--------");

        for pos in positions.iter() {
            println!("{:8} {:10} {:10} {:12.2} {:12.2} {:12.2} {:6.2} {:10.2}",
                     pos.symbol,
                     pos.base_date.format("%Y-%m-%d"),
                     pos.exit_date.format("%Y-%m-%d"),
                     pos.base_notional(),
                     pos.exit_notional(),
                     pos.net_notional(),
                     pos.base_fee + pos.exit_fee,
                     pos.dividend);
        }
    }

    fn export(&self, params: &ReportParams, filename: &str) -> Result<(), Error> {
        let positions = params.closed_positions();

        let mut file = File::create(filename)?;
        writeln!(file, "Symbol,Base Date,Exit Date,Base Value,Exit Value,Net Value,Fees,Dividend")?;

        for pos in positions.iter() {
            writeln!(file, "{},{},{},{:.2},{:.2},{:.2},{:.2},{:.2}",
                     pos.symbol,
                     pos.base_date.format("%Y-%m-%d"),
                     pos.exit_date.format("%Y-%m-%d"),
                     pos.base_notional(),
                     pos.exit_notional(),
                     pos.net_notional(),
                     pos.base_fee + pos.exit_fee,
                     pos.dividend)?;
        }
        Ok(())
    }
}
