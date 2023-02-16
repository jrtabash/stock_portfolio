use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::env;
use std::fs;

pub fn make_path(name: &str) -> PathBuf {
    let mut pbuf = env::temp_dir();
    pbuf.push(name);
    pbuf
}

pub fn create_file(name: &str, data: &str) -> bool {
    let path_buf = make_path(name);
    let path = path_buf.as_path();

    if remove_path(path) {
        if let Ok(mut file) = fs::File::create(path) {
            if !data.is_empty() && write!(file, "{}", data).is_err() {
                return false;
            }
            return true;
        }
    }

    false
}

pub fn remove_file(name: &str) -> bool {
    let path_buf = make_path(name);
    let path = path_buf.as_path();
    remove_path(path)
}

fn remove_path(path: &Path) -> bool {
    if path.exists() && fs::remove_file(path).is_err() {
        return false;
    }
    true
}

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn test_temp_file_empty() {
        let name = "test_temp_file_empty.txt";
        let data = "";
        test_temp_file(&name, &data);
    }

    #[test]
    fn test_temp_file_data() {
        let name = "test_temp_file_data.txt";
        let data = "This is a test";
        test_temp_file(&name, &data);
    }

    fn test_temp_file(name: &str, data: &str) {
        assert!(create_file(&name, &data));
        assert!(check_data(&name, &data));
        assert!(remove_file(&name));
    }

    fn check_data(name: &str, data: &str) -> bool {
        let file_path = make_path(&name);
        if let Ok(file) = File::open(file_path.as_path()) {
            let mut reader = BufReader::new(file);
            let mut content = String::new();
            if let Ok(_) = reader.read_to_string(&mut content) {
                return data == content;
            }
        }
        return false;
    }
}
