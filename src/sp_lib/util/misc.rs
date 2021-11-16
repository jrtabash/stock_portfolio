#[inline(always)]
pub fn maybe_letter_s(sz: usize) -> &'static str {
    if sz == 1 { "" } else { "s" }
}

#[inline(always)]
pub fn count_format(sz: usize, item: &str) -> String {
    format!("{} {}{}", sz, item, maybe_letter_s(sz))
}

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maybe_letter_s() {
        assert_eq!(maybe_letter_s(0), "s");
        assert_eq!(maybe_letter_s(1), "");
        assert_eq!(maybe_letter_s(2), "s");
    }

    #[test]
    fn test_count_format() {
        assert_eq!(count_format(0, "apple"), "0 apples");
        assert_eq!(count_format(1, "apple"), "1 apple");
        assert_eq!(count_format(2, "apple"), "2 apples");
    }
}
