// --------------------------------------------------------------------------------
// Events

#[derive(Copy, Clone)]
#[allow(dead_code)]
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
#[allow(dead_code)]
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
