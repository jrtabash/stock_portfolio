// --------------------------------------------------------------------------------
// Events

#[derive(Copy, Clone)]
pub enum Events {
    History,
    Dividend,
    Split
}

pub fn events2str(evt: Events) -> &'static str {
    match evt {
        Events::History => "history",
        Events::Dividend => "div",
        Events::Split => "split"
    }
}

// --------------------------------------------------------------------------------
// Interval

#[derive(Copy, Clone)]
pub enum Interval {
    Daily,
    Weekly,
    Monthly
}

pub fn interval2str(int: Interval) -> &'static str {
    match int {
        Interval::Daily => "1d",
        Interval::Weekly => "1wk",
        Interval::Monthly => "1mo"
    }
}

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_events() {
        assert_eq!(events2str(Events::History), "history");
        assert_eq!(events2str(Events::Dividend), "div");
        assert_eq!(events2str(Events::Split), "split");
    }

    #[test]
    fn test_interval() {
        assert_eq!(interval2str(Interval::Daily), "1d");
        assert_eq!(interval2str(Interval::Weekly), "1wk");
        assert_eq!(interval2str(Interval::Monthly), "1mo");
    }
}
