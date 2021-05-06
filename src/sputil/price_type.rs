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

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_cmp() {
        assert_eq!(price_cmp(10.50, 1.0), Ordering::Greater);
        assert_eq!(price_cmp(1.0, 10.50), Ordering::Less);
        assert_eq!(price_cmp(1.0, 1.0), Ordering::Equal);
    }
}
