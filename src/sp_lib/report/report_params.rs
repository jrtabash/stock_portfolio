use crate::datastore::datastore::DataStore;
use crate::portfolio::closed_position::ClosedPositionList;
use crate::portfolio::stock::StockList;
use crate::portfolio::stocks_config::StocksConfig;
use crate::report::report_type::ReportType;

pub struct ReportParams<'a, 'b> {
    rtype: ReportType,
    config: &'a StocksConfig,
    ds: Option<&'b DataStore>,
    groupby: bool
}

impl<'a, 'b> ReportParams<'a, 'b> {
    pub fn new(rtype: ReportType, config: &'a StocksConfig) -> Self {
        ReportParams {
            rtype,
            config,
            ds: None,
            groupby: false
        }
    }

    pub fn show_groupby(mut self, grpby: bool) -> Self {
        self.groupby = grpby;
        self
    }

    pub fn with_datastore(mut self, ds: &'b DataStore) -> Self {
        self.ds = Some(ds);
        self
    }

    #[inline(always)]
    pub fn rtype(&self) -> ReportType { self.rtype }

    #[inline(always)]
    pub fn config(&self) -> &'a StocksConfig { &self.config }

    #[inline(always)]
    pub fn stocks(&self) -> &'a StockList { self.config.stocks() }

    #[inline(always)]
    pub fn closed_positions(&self) -> &'a ClosedPositionList { self.config.closed_positions() }

    #[inline(always)]
    pub fn datastore(&self) -> Option<&'b DataStore> { self.ds }

    #[inline(always)]
    pub fn groupby(&self) -> bool { self.groupby }
}
