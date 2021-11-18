use std::fs;

#[inline(always)]
pub fn maybe_letter_s(sz: usize) -> &'static str {
    if sz == 1 { "" } else { "s" }
}

#[inline(always)]
pub fn count_format(sz: usize, item: &str) -> String {
    format!("{} {}{}", sz, item, maybe_letter_s(sz))
}

#[inline(always)]
pub fn direntry_filename(entry: &fs::DirEntry) -> String {
    match entry.file_name().to_str() {
        Some(fname) => String::from(fname),
        None => String::from("?")
    }
}

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use crate::util::temp_file;

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

    #[test]
    fn test_direntry_filename() {
        let name = "direntry.txt";
        let path = String::from(temp_file::make_path(&name).to_str().unwrap());
        let data = "";
        let mut found = false;

        temp_file::remove_file(&name);
        assert!(temp_file::create_file(&name, &data));

        for entry in fs::read_dir(env::temp_dir().as_path()).unwrap() {
            let entry = entry.unwrap();
            if let Some(entry_str) = entry.path().to_str() {
                if entry_str != &path {
                    continue;
                }
            }

            found = true;
            assert_eq!(direntry_filename(&entry), String::from(name));
            break;
        }

        assert!(found);
        assert!(temp_file::remove_file(&name));
    }
}
