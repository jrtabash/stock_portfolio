pub type Scaled = i64;

pub const SCALE: Scaled = 10000;
pub const PARTIAL_MIN: Scaled = 0;
pub const PARTIAL_MAX: Scaled = 9999;
pub const SCALE_MIN: Scaled = Scaled::MIN;
pub const SCALE_MAX: Scaled = Scaled::MAX;

#[inline(always)]
pub fn scaled_whole(value: Scaled) -> Scaled {
    value / SCALE
}

#[inline(always)]
pub fn scaled_partial(value: Scaled) -> Scaled {
    (value % SCALE).abs()
}

#[inline(always)]
pub fn parts_to_scaled(whole: Scaled, partial: Scaled) -> Scaled {
    whole * SCALE + partial.min(PARTIAL_MAX).max(PARTIAL_MIN)
}

#[inline(always)]
pub fn scaled_to_parts(value: Scaled) -> (Scaled, Scaled) {
    (scaled_whole(value), scaled_partial(value))
}

#[inline(always)]
pub fn float_to_scaled(value: f64) -> Scaled {
    (value * (SCALE as f64)) as Scaled
}

#[inline(always)]
pub fn scaled_to_float(value: Scaled) -> f64 {
    value as f64 / SCALE as f64
}

pub fn string_to_scaled(value: &str) -> Scaled {
    let negative = value.starts_with('-');
    let value_str = if negative { &value[1..] } else { value };

    let mut seen_dot: bool = false;
    let mut part_scale: Scaled = 1;
    let mut ret: Scaled = 0;

    for c in value_str.chars() {
        if c == '.' {
            seen_dot = true;
            continue;
        }

        if seen_dot {
            if part_scale >= SCALE {
                break;
            }
            part_scale *= 10;
        }
        ret = (ret * 10) + (c.to_digit(10).unwrap_or(0) as Scaled);
    }

    while part_scale < SCALE {
        ret *= 10;
        part_scale *= 10;
    }

    if negative {
        ret *= -1;
    }

    ret
}

#[inline(always)]
pub fn scaled_to_string(value: Scaled) -> String {
    format!("{}.{:04}", scaled_whole(value), scaled_partial(value))
}

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whole_partial() {
        let s = 12500;
        assert_eq!(scaled_whole(s), 1);
        assert_eq!(scaled_partial(s), 2500);

        let s = -12500;
        assert_eq!(scaled_whole(s), -1);
        assert_eq!(scaled_partial(s), 2500);
    }

    #[test]
    fn test_parts_scaled() {
        fn test_case(w: Scaled, p: Scaled, e: Scaled, ep: Scaled) {
            let s = parts_to_scaled(w, p);
            assert_eq!(s, e);
            assert_eq!((w, ep), scaled_to_parts(s));
        }

        test_case(1, 5200, 15200, 5200);
        test_case(1, 0, 10000, 0);
        test_case(1, 9999, 19999, 9999);
        test_case(1, -1, 10000, 0);
        test_case(1, 10000, 19999, 9999);
    }

    #[test]
    fn test_float_scaled() {
        fn test_case(f: f64, e: Scaled) {
            let s = float_to_scaled(f);
            assert_eq!(s, e);
            assert!((f - scaled_to_float(s)).abs() < 0.0001);
        }

        test_case(1.25, 12500);
        test_case(12.3455, 123455);
        test_case(0.0001, 1);
        test_case(0.9999, 9999);
        test_case(-1.25, -12500);
        test_case(154.123456, 1541234);
    }

    #[test]
    fn test_string_scaled() {
        fn test_case(s: &str, e: Scaled, es: &str) {
            let v = string_to_scaled(s);
            assert_eq!(v, e);
            assert_eq!(es, scaled_to_string(v));
        }

        test_case("1.25", 12500, "1.2500");
        test_case("12.3455", 123455, "12.3455");
        test_case("0.0001", 1, "0.0001");
        test_case("0.9999", 9999, "0.9999");
        test_case("-1.25", -12500, "-1.2500");
        test_case("154.123456", 1541234, "154.1234");
        test_case("154.123", 1541230, "154.1230");
        test_case("154", 1540000, "154.0000");
        test_case("154.", 1540000, "154.0000");
        test_case("-154", -1540000, "-154.0000");
    }
}
