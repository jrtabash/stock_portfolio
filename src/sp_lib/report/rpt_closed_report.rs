use std::fs::File;
use std::io::prelude::*;

use crate::portfolio::closed_position::Price;
use crate::report::report_params::ReportParams;
use crate::report::report_trait::Report;
use crate::util::datetime;
use crate::util::error::Error;
use crate::util::fixed_price::FP_0;

pub struct ClosedReport {}

impl Report for ClosedReport {
    fn write(&self, params: &ReportParams) {
        let positions = params.closed_positions();

        let mut base_ntnl: Price = FP_0;
        let mut exit_ntnl: Price = FP_0;
        let mut net_ntnl: Price = FP_0;
        let mut tot_fees: Price = FP_0;
        let mut tot_div: Price = FP_0;

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
        println!("Total Base Value: {}", base_ntnl.to_formatted(2));
        println!("Total Exit Value: {}", exit_ntnl.to_formatted(2));
        println!(" Total Net Value: {}", net_ntnl.to_formatted(2));
        println!("      Total Fees: {}", tot_fees.to_formatted(2));
        println!("  Total Dividend: {}", tot_div.to_formatted(2));
        println!("Net + Div - Fees: {}", (net_ntnl + tot_div - tot_fees).to_formatted(2));
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
            println!("{:8} {:10} {:10} {:>12} {:>12} {:>12} {:>6} {:>10}",
                     pos.symbol,
                     pos.base_date.format("%Y-%m-%d"),
                     pos.exit_date.format("%Y-%m-%d"),
                     pos.base_notional().to_formatted(2),
                     pos.exit_notional().to_formatted(2),
                     pos.net_notional().to_formatted(2),
                     (pos.base_fee + pos.exit_fee).to_formatted(2),
                     pos.dividend.to_formatted(2));
        }
    }

    fn export(&self, params: &ReportParams, filename: &str) -> Result<(), Error> {
        let positions = params.closed_positions();

        let mut file = File::create(filename)?;
        writeln!(file, "Symbol,Base Date,Exit Date,Base Value,Exit Value,Net Value,Fees,Dividend")?;

        for pos in positions.iter() {
            writeln!(file, "{},{},{},{},{},{},{},{}",
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
