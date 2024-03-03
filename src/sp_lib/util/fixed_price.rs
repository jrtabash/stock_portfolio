use crate::util::error::Error;
use crate::util::scaled_util::*;
use std::fmt;
use std::iter::zip;
use std::ops;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct FixedPrice {
    value: Scaled
}

pub const FP_0: FixedPrice = FixedPrice { value: 0 };
pub const FP_1: FixedPrice = FixedPrice { value: SCALE };
pub const FP_100: FixedPrice = FixedPrice { value: 100 * SCALE };
pub const FP_365: FixedPrice = FixedPrice { value: 365 * SCALE };
pub const FP_MIN: FixedPrice = FixedPrice { value: SCALE_MIN };
pub const FP_MAX: FixedPrice = FixedPrice { value: SCALE_MAX };

impl FixedPrice {
    #[inline(always)]
    pub fn new() -> Self {
        FixedPrice { value: 0 }
    }

    #[inline(always)]
    pub fn from_scaled(value: Scaled) -> Self {
        FixedPrice { value }
    }

    #[inline(always)]
    pub fn from_parts(whole: Scaled, partial: Scaled) -> Self {
        FixedPrice {
            value: parts_to_scaled(whole, partial)
        }
    }

    #[inline(always)]
    pub fn from_float(value: f64) -> Self {
        FixedPrice {
            value: float_to_scaled(value)
        }
    }

    #[inline(always)]
    pub fn from_signed(value: i32) -> Self {
        FixedPrice {
            value: value as Scaled * SCALE
        }
    }

    #[inline(always)]
    pub fn from_unsigned(value: u32) -> Self {
        FixedPrice {
            value: value as Scaled * SCALE
        }
    }

    #[inline(always)]
    pub fn from_string(value: &str) -> Self {
        FixedPrice {
            value: string_to_scaled(value)
        }
    }

    #[inline(always)]
    pub fn parse(value: &str) -> Result<Self, Error> {
        Ok(FixedPrice {
            value: parse_scaled(value)?
        })
    }

    #[inline(always)]
    pub fn to_scaled(&self) -> Scaled {
        self.value
    }

    #[inline(always)]
    pub fn to_parts(&self) -> (Scaled, Scaled) {
        scaled_to_parts(self.value)
    }

    #[inline(always)]
    pub fn to_whole(&self) -> Scaled {
        scaled_whole(self.value)
    }

    #[inline(always)]
    pub fn to_partial(&self) -> Scaled {
        scaled_partial(self.value)
    }

    #[inline(always)]
    pub fn to_float(&self) -> f64 {
        scaled_to_float(self.value)
    }

    pub fn to_formatted(&self, dp: u32) -> String {
        let (whole, partial) = scaled_to_parts(self.value);
        match dp {
            0 => format!("{}", whole),
            1 => format!("{}.{:01}", whole, (partial / 1000)),
            2 => format!("{}.{:02}", whole, (partial / 100)),
            3 => format!("{}.{:03}", whole, (partial / 10)),
            _ => format!("{}.{:04}", whole, partial)
        }
    }

    #[inline(always)]
    pub fn abs(&self) -> FixedPrice {
        FixedPrice::from_scaled(self.value.abs())
    }

    #[inline(always)]
    pub fn sign(&self) -> FixedPrice {
        if self.value >= 0 {
            FP_1
        } else {
            -FP_1
        }
    }

    #[inline(always)]
    pub fn slices_eql(lhs: &[FixedPrice], rhs: &[FixedPrice]) -> bool {
        lhs.len() == rhs.len() && zip(lhs, rhs).all(|(l, r)| l == r)
    }
}

impl Default for FixedPrice {
    fn default() -> Self {
        Self::new()
    }
}

impl From<f64> for FixedPrice {
    #[inline(always)]
    fn from(item: f64) -> Self {
        FixedPrice::from_float(item)
    }
}

impl From<i32> for FixedPrice {
    #[inline(always)]
    fn from(item: i32) -> Self {
        FixedPrice::from_signed(item)
    }
}

impl From<u32> for FixedPrice {
    #[inline(always)]
    fn from(item: u32) -> Self {
        FixedPrice::from_unsigned(item)
    }
}

impl fmt::Display for FixedPrice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (whole, partial) = scaled_to_parts(self.value);
        write!(f, "{}.{:04}", whole, partial)
    }
}

impl ops::Add for FixedPrice {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::from_scaled(self.value + other.value)
    }
}

impl ops::AddAssign for FixedPrice {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

impl ops::Sub for FixedPrice {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::from_scaled(self.value - other.value)
    }
}

impl ops::SubAssign for FixedPrice {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

impl ops::Mul for FixedPrice {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::from_scaled(self.value * other.value / SCALE)
    }
}

impl ops::MulAssign for FixedPrice {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl ops::Div for FixedPrice {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::from_scaled(self.value * SCALE / other.value)
    }
}

impl ops::DivAssign for FixedPrice {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl ops::Rem for FixedPrice {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Self::from_scaled(self.value % other.value)
    }
}

impl ops::RemAssign for FixedPrice {
    fn rem_assign(&mut self, other: Self) {
        *self = *self % other;
    }
}

impl ops::Neg for FixedPrice {
    type Output = Self;

    fn neg(self) -> Self {
        Self::from_scaled(-self.value)
    }
}

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp;

    #[test]
    fn test_price_construction() {
        assert_eq!(FixedPrice::new().to_scaled(), 0);
        assert_eq!(FixedPrice::from_scaled(10000).to_scaled(), 10000);
        assert_eq!(FixedPrice::from_parts(1, 5200).to_scaled(), 15200);
        assert_eq!(FixedPrice::from_float(1.52).to_scaled(), 15200);
        assert_eq!(FixedPrice::from_signed(100).to_scaled(), 1000000);
        assert_eq!(FixedPrice::from_unsigned(100).to_scaled(), 1000000);
        assert_eq!(FixedPrice::from_string("1.52").to_scaled(), 15200);
    }

    #[test]
    fn test_price_parse() {
        assert_eq!(FixedPrice::parse("1.52").unwrap().to_scaled(), 15200);
        assert_eq!(FixedPrice::parse("-1.52").unwrap().to_scaled(), -15200);
        assert_eq!(FixedPrice::parse("0.00").unwrap().to_scaled(), 0);

        assert!(FixedPrice::parse("foobar").is_err());
        assert!(FixedPrice::parse("-10foo").is_err());
    }

    #[test]
    fn test_price_conversion() {
        let price = FixedPrice::from_scaled(15200);
        assert_eq!(price.to_scaled(), 15200);
        assert_eq!(price.to_parts(), (1, 5200));
        assert_eq!(price.to_whole(), 1);
        assert_eq!(price.to_partial(), 5200);
        assert_eq!(price.to_float(), 1.52);
        assert_eq!(price.to_string(), "1.5200");
    }

    #[test]
    fn test_price_formatted() {
        let p = FixedPrice::from_string("12.3456");
        assert_eq!(p.to_formatted(0), "12");
        assert_eq!(p.to_formatted(1), "12.3");
        assert_eq!(p.to_formatted(2), "12.34");
        assert_eq!(p.to_formatted(3), "12.345");
        assert_eq!(p.to_formatted(4), "12.3456");
        assert_eq!(p.to_formatted(5), "12.3456");

        let p = FixedPrice::from_string("12.25");
        assert_eq!(p.to_formatted(0), "12");
        assert_eq!(p.to_formatted(1), "12.2");
        assert_eq!(p.to_formatted(2), "12.25");
        assert_eq!(p.to_formatted(3), "12.250");
        assert_eq!(p.to_formatted(4), "12.2500");
        assert_eq!(p.to_formatted(5), "12.2500");

        let p = FixedPrice::from_string("12.0025");
        assert_eq!(p.to_formatted(0), "12");
        assert_eq!(p.to_formatted(1), "12.0");
        assert_eq!(p.to_formatted(2), "12.00");
        assert_eq!(p.to_formatted(3), "12.002");
        assert_eq!(p.to_formatted(4), "12.0025");
        assert_eq!(p.to_formatted(5), "12.0025");
    }

    #[test]
    fn test_price_equality() {
        let zero1 = FixedPrice::new();
        let zero2 = FixedPrice::new();
        let nonzero1 = FixedPrice::from_string("1.52");
        let nonzero2 = FixedPrice::from_string("1.52");

        assert!(zero1 == zero2);
        assert!(zero1 != nonzero1);
        assert!(nonzero1 == nonzero2);

        assert!(zero1 == FP_0);
        assert!(nonzero1 != FP_0);
    }

    #[test]
    fn test_price_ordering() {
        let p1 = FixedPrice::from_string("1.0");
        let p2 = FixedPrice::from_string("2.0");
        let p3 = FixedPrice::from_string("1.0");

        assert!(p1 < p2);
        assert!(p1 <= p3);
        assert!(p2 > p1);
        assert!(p1 >= p3);

        assert_eq!(Ord::cmp(&p1, &p2), cmp::Ordering::Less);
        assert_eq!(Ord::cmp(&p2, &p1), cmp::Ordering::Greater);
        assert_eq!(Ord::cmp(&p1, &p3), cmp::Ordering::Equal);
        assert_eq!(p1.cmp(&p2), cmp::Ordering::Less);
        assert_eq!(p2.cmp(&p1), cmp::Ordering::Greater);
        assert_eq!(p1.cmp(&p3), cmp::Ordering::Equal);

        assert_eq!(Ord::min(p1, p2), p1);
        assert_eq!(Ord::max(p1, p2), p2);
        assert_eq!(p1.min(p2), p1);
        assert_eq!(p1.max(p2), p2);
    }

    #[test]
    fn test_price_consts() {
        assert_eq!(FixedPrice::new(), FP_0);
        assert_eq!(FixedPrice::from_scaled(SCALE), FP_1);
        assert_eq!(FixedPrice::from_scaled(100 * SCALE), FP_100);
        assert_eq!(FixedPrice::from_scaled(365 * SCALE), FP_365);
        assert_eq!(FixedPrice::from_scaled(SCALE_MIN), FP_MIN);
        assert_eq!(FixedPrice::from_scaled(SCALE_MAX), FP_MAX);
    }

    #[test]
    fn test_price_add() {
        let mut p1 = FixedPrice::from_string("1.52");
        let mut p2 = FixedPrice::from_string("2.12");
        let p3 = FixedPrice::from_string("-1.02");
        assert_eq!((p1 + p2).to_scaled(), 36400);
        assert_eq!((p1 + p3).to_scaled(), 5000);

        p1 += p2;
        assert_eq!(p1.to_scaled(), 36400);

        p2 += p3;
        assert_eq!(p2.to_scaled(), 11000);
    }

    #[test]
    fn test_price_sub() {
        let p1 = FixedPrice::from_string("1.52");
        let mut p2 = FixedPrice::from_string("2.12");
        let mut p3 = FixedPrice::from_string("-1.02");
        assert_eq!((p2 - p1).to_scaled(), 6000);
        assert_eq!((p1 - p3).to_scaled(), 25400);
        assert_eq!((p3 - p1).to_scaled(), -25400);

        p2 -= p1;
        assert_eq!(p2.to_scaled(), 6000);

        p3 -= p1;
        assert_eq!(p3.to_scaled(), -25400);
    }

    #[test]
    fn test_price_mul() {
        let p1 = FixedPrice::from_string("1.52");
        let mut p2 = FixedPrice::from_string("2.12");
        let mut p3 = FixedPrice::from_string("-1.02");
        assert_eq!((p1 * p2).to_scaled(), 32224);
        assert_eq!((p1 * p3).to_scaled(), -15504);

        p2 *= p1;
        assert_eq!(p2.to_scaled(), 32224);

        p3 *= p1;
        assert_eq!(p3.to_scaled(), -15504);
    }

    #[test]
    fn test_price_div() {
        let mut p1 = FixedPrice::from_string("1.52");
        let mut p2 = FixedPrice::from_string("2.12");
        let p3 = FixedPrice::from_string("-1.02");
        assert_eq!((p1 / p2).to_scaled(), 7169);
        assert_eq!((p2 / p1).to_scaled(), 13947);
        assert_eq!((p1 / p3).to_scaled(), -14901);

        p2 /= p1;
        assert_eq!(p2.to_scaled(), 13947);

        p1 /= p3;
        assert_eq!(p1.to_scaled(), -14901);

        let p1 = FixedPrice::from_string("1.0002");
        let p2 = FixedPrice::from_string("0.0001");
        let p3 = FixedPrice::from_string("2.0000");
        assert_eq!((p1 / p3).to_scaled(), 5001);
        assert_eq!((p2 / p3).to_scaled(), 0);

        let p1 = FixedPrice::from_string("10.00");
        let p2 = FixedPrice::from_string("100000.00");
        let p3 = FixedPrice::from_string("-100000.00");
        assert_eq!((p1 / p2).to_scaled(), 1);
        assert_eq!((p1 / p3).to_scaled(), -1);
    }

    #[test]
    fn test_price_rem() {
        let mut p1 = FixedPrice::from_string("9.00");
        let mut p2 = FixedPrice::from_string("10.00");
        let p3 = FixedPrice::from_string("2.00");
        assert_eq!((p1 % p3).to_scaled(), SCALE);
        assert_eq!((p2 % p3).to_scaled(), 0);

        p1 %= p3;
        assert_eq!(p1.to_scaled(), SCALE);

        p2 %= p3;
        assert_eq!(p2.to_scaled(), 0);
    }

    #[test]
    fn test_price_neg() {
        let p1 = FixedPrice::from_string("2.00");
        let p2 = FixedPrice::from_string("-3.00");
        let p3 = FP_0;
        assert_eq!((-p1).to_scaled(), -20000);
        assert_eq!((-p2).to_scaled(), 30000);
        assert_eq!((-p3).to_scaled(), 0);
    }

    #[test]
    fn test_price_abs() {
        let p1 = FixedPrice::from_string("2.00");
        let p2 = FixedPrice::from_string("-3.00");
        assert_eq!(p1.abs().to_scaled(), 20000);
        assert_eq!(p2.abs().to_scaled(), 30000);
    }

    #[test]
    fn test_price_sign() {
        let p1 = FixedPrice::from_string("2.00");
        let p2 = FixedPrice::from_string("-3.00");
        assert_eq!(p1.sign().to_scaled(), 10000);
        assert_eq!(p2.sign().to_scaled(), -10000);
    }

    #[test]
    fn test_price_slices_eql() {
        let p1 = FixedPrice::from_string("1.00");
        let p2 = FixedPrice::from_string("2.00");
        let p3 = FixedPrice::from_string("3.00");

        let q1 = FixedPrice::from_string("1.00");
        let q2 = FixedPrice::from_string("2.00");
        let q3 = FixedPrice::from_string("3.00");

        assert!(FixedPrice::slices_eql(&[p1], &[q1]));
        assert!(FixedPrice::slices_eql(&[p1, p2], &[q1, q2]));
        assert!(FixedPrice::slices_eql(&[p1, p2, p3], &[q1, q2, q3]));

        assert!(!FixedPrice::slices_eql(&[p1, p2], &[q1]));
        assert!(!FixedPrice::slices_eql(&[p1, p2], &[q1, q2, q3]));
        assert!(!FixedPrice::slices_eql(&[p1], &[q2]));
        assert!(!FixedPrice::slices_eql(&[p1, p2], &[q1, q3]));
        assert!(!FixedPrice::slices_eql(&[p1, p2], &[q2, q1]));
    }

    #[test]
    fn test_price_from() {
        let p: FixedPrice = 10.50.into();
        assert_eq!(p, FixedPrice::from_float(10.50));

        let p: FixedPrice = (10 as i32).into();
        assert_eq!(p, FixedPrice::from_signed(10));

        let p: FixedPrice = (10 as u32).into();
        assert_eq!(p, FixedPrice::from_unsigned(10));
    }
}
