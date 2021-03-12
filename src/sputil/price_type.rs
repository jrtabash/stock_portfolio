use std::cmp::Ordering;

pub type PriceType = f64;

pub fn price_cmp(lhs: PriceType, rhs: PriceType) -> Ordering {
    if lhs < rhs {
        return Ordering::Less;
    }
    else if lhs > rhs {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}
