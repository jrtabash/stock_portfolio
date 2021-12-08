use std::cmp::Ordering;

// TODO: Use fixed point number to represent price.
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

#[inline(always)]
pub fn price_zero(px: PriceType) -> bool {
    px.abs() < 0.000001
}

#[inline(always)]
pub fn price_eql(lhs: PriceType, rhs: PriceType) -> bool {
    price_zero(lhs - rhs)
}

pub fn prices_eql(lhs: &[PriceType], rhs: &[PriceType]) -> bool {
    if lhs.len() != rhs.len() {
        return false;
    }

    for i in 0..lhs.len() {
        if !price_eql(lhs[i], rhs[i]) {
            return false;
        }
    }

    return true;
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

    #[test]
    fn test_price_zero() {
        assert!(price_zero(0.0));
        assert!(price_zero(0.0000009));
        assert!(price_zero(-0.0000009));

        assert!(!price_zero(1.0));
        assert!(!price_zero(0.000001));
        assert!(!price_zero(-0.000001));
    }

    #[test]
    fn test_price_eql() {
        assert!(price_eql(0.0, 0.0));
        assert!(price_eql(0.000001, 0.000001));
        assert!(price_eql(-0.000001, -0.000001));

        assert!(!price_eql(1.0, 1.1));
        assert!(!price_eql(0.000001, 0.000002));
        assert!(!price_eql(-0.000001, -0.000002));

        assert!(price_eql(0.000001, 0.0000009));
        assert!(price_eql(-0.000001, -0.0000009));
    }

    #[test]
    fn test_prices_eql() {
        let lhs: Vec<PriceType> = vec![0.1, 2.1, 3.7, 0.05];
        let rhs1: Vec<PriceType> = vec![0.1, 2.1, 3.7, 0.05];
        let rhs2: Vec<PriceType> = vec![0.1, 2.1, 3.7, 0.25];
        let rhs3: Vec<PriceType> = vec![0.1, 2.1, 3.7];

        assert!(prices_eql(&lhs, &rhs1));
        assert!(!prices_eql(&lhs, &rhs2));
        assert!(!prices_eql(&lhs, &rhs3));

        assert!(prices_eql(&lhs[1..3], &rhs1[1..3]));
        assert!(prices_eql(&lhs[1..3], &rhs2[1..3]));
        assert!(prices_eql(&lhs[1..3], &rhs3[1..3]));
    }
}
