use std::io::prelude::*;
use std::fmt;
use std::fs;
use std::str;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::error::Error;

pub struct DataStore {
    root: PathBuf,
    name: String,
    base_path: PathBuf
}

impl DataStore {
    pub fn new(root: &str, name: &str) -> Self {
        DataStore {
            root: PathBuf::from(root),
            name: String::from(name),
            base_path: DataStore::make_base_path(&root, &name)
        }
    }

    #[inline(always)]
    pub fn root(&self) -> &Path {
        &self.root
    }

    #[inline(always)]
    pub fn name(&self) -> &String {
        &self.name
    }

    #[inline(always)]
    pub fn base_path(&self) -> &Path {
        self.base_path.as_path()
    }

    #[inline(always)]
    pub fn exists(&self) -> bool {
        self.root.exists() && self.base_path.exists()
    }

    #[inline(always)]
    pub fn symbol_exists(&self, tag: &str, symbol: &str) -> bool {
        DataStore::make_symbol_file(&self.base_path, tag, symbol).exists()
    }

    pub fn create(&self) -> Result<(), Box<dyn Error>> {
        if self.exists() {
            Err(format!("Datastore '{}' already exists", self.name).into())
        }
        else {
            fs::create_dir(self.base_path.as_path())?;
            Ok(())
        }
    }

    pub fn delete(&self) -> Result<(), Box<dyn Error>> {
        if self.exists() {
            fs::remove_dir_all(self.base_path.as_path())?;
            Ok(())
        }
        else {
            Err(format!("Datastore '{}' does not exist", self.name).into())
        }
    }

    pub fn read_file(&self, sym_file: &Path) -> Result<String, Box<dyn Error>> {
        let file = fs::File::open(sym_file)?;
        let mut reader = BufReader::new(&file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        Ok(content)
    }

    pub fn read_last(&self, sym_file: &Path) -> Result<String, Box<dyn Error>> {
        let mut file = fs::File::open(sym_file)?;

        let meta_data = file.metadata()?;
        let file_size = meta_data.len();
        let mut position = if file_size > 1 { file_size - 2 } else { 0 };

        file.seek(std::io::SeekFrom::Start(position))?;

        let mut buf = [0; 1];
        while position > 0 {
            file.read(&mut buf)?;
            if str::from_utf8(&buf)? == "\n" {
                break;
            }
            position = position - 1;
            file.seek(std::io::SeekFrom::Start(position))?;
        }

        let mut content = String::new();
        file.read_to_string(&mut content)?;
        match content.trim().split('\n').last() {
            Some(line) => Ok(String::from(line)),
            None => Ok(String::new())
        }
    }

    pub fn select_symbol(&self, tag: &str, symbol: &str) -> Result<String, Box<dyn Error>> {
        let sym_file = DataStore::make_symbol_file(&self.base_path, tag, symbol);
        self.read_file(&sym_file)
    }

    pub fn select_last(&self, tag: &str, symbol: &str) -> Result<String, Box<dyn Error>> {
        let sym_file = DataStore::make_symbol_file(&self.base_path, tag, symbol);
        self.read_last(&sym_file)
    }

    pub fn insert_symbol(&self, tag: &str, symbol: &str, csv: &str) -> Result<(), Box<dyn Error>> {
        // Skip non-numeric header if one exists.
        let csv_ref =
            match csv.find(char::is_numeric) {
                Some(pos) => {
                    if pos > 0 { &csv[pos..] } else { csv }
                },
                None => &""
            };

        if !csv_ref.is_empty() {
            let sym_file = DataStore::make_symbol_file(&self.base_path, tag, symbol);
            let exists = sym_file.exists();
            let mut file =
                if exists {
                    fs::OpenOptions::new().write(true).open(sym_file)?
                } else {
                    fs::File::create(sym_file)?
                };
            file.seek(std::io::SeekFrom::End(0))?;

            let mut last_line: Option<&str> = None;
            for line in csv_ref.trim().split('\n') {
                // Expect and ignore consecutive lines that start with the same date
                if let Some(last) = last_line {
                    if let Some(comma) = last.find(',') {
                        if line.starts_with(&last[..comma]) {
                            continue;
                        }
                    }
                }
                write!(file, "{}\n", line)?;
                last_line = Some(line);
            }
        }
        Ok(())
    }

    pub fn drop_symbol(&self, tag: &str, symbol: &str) -> Result<(), Box<dyn Error>> {
        let sym_file = DataStore::make_symbol_file(&self.base_path, tag, symbol);
        fs::remove_file(sym_file.as_path())?;
        Ok(())
    }

    fn make_base_path(root: &str, name: &str) -> PathBuf {
        let mut pbuf = PathBuf::from(root);
        pbuf.push(name);
        pbuf
    }

    fn make_symbol_file(base: &PathBuf, tag: &str, symbol: &str) -> PathBuf {
        let mut pbuf = base.clone();
        pbuf.push(&format!("{}_{}.csv", tag, symbol));
        pbuf
    }
}

impl fmt::Display for DataStore {
    fn fmt(self: &DataStore, f: &mut fmt::Formatter) -> fmt::Result {
        match self.base_path().to_str() {
            Some(ds_path) => write!(f, "{}", ds_path),
            None => write!(f, "{}", self.name())
        }
    }
}

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::temp_file;
    use std::env;

    #[test]
    fn test_datastore_new() {
        let root = env::temp_dir();
        let base_path = temp_file::make_path("test_ops");
        let ds = DataStore::new(root.to_str().unwrap(), "test_ops");
        assert!(!ds.exists());
        assert_eq!(ds.root(), root.as_path());
        assert_eq!(ds.name(), "test_ops");
        assert_eq!(ds.base_path(), base_path.as_path());
        assert!(!base_path.exists());
    }

    #[test]
    fn test_datastore_create_delete() {
        let root = env::temp_dir();
        let base_path = temp_file::make_path("test_create_delete");
        let ds = DataStore::new(root.to_str().unwrap(), "test_create_delete");
        assert!(!ds.exists());
        assert!(ds.create().is_ok());
        assert!(ds.create().is_err());
        assert!(ds.exists());
        assert!(base_path.exists());
        assert!(ds.delete().is_ok());
        assert!(ds.delete().is_err());
        assert!(!base_path.exists());
    }

    #[test]
    fn test_datastore_create_insert_delete() {
        let root = env::temp_dir();
        let base_path = temp_file::make_path("test_create_delete2");
        let ds = DataStore::new(root.to_str().unwrap(), "test_create_delete2");

        let tag = "tst";
        let symbol = "TEST";
        let csv = "1,2,3,4,5\n\
                   6,7,8,9,10\n\
                   11,12,13,14,15\n";
        let test_file = temp_file::make_path("test_create_delete2/tst_TEST.csv");

        assert!(!ds.exists());
        assert!(ds.create().is_ok());
        assert!(ds.create().is_err());
        assert!(ds.exists());
        assert!(base_path.exists());

        assert!(!test_file.exists());
        assert!(ds.insert_symbol(&tag, &symbol, &csv).is_ok());
        assert!(test_file.exists());

        assert!(ds.delete().is_ok());
        assert!(ds.delete().is_err());
        assert!(!base_path.exists());
        assert!(!test_file.exists());
    }

    #[test]
    fn test_datastore_insert_select() {
        let root = env::temp_dir();
        let base_path = temp_file::make_path("test_insert_select");
        let ds = DataStore::new(root.to_str().unwrap(), "test_insert_select");

        let tag = "tst";
        let symbol = "TEST";
        let csv = "1,2,3,4,5\n\
                   6,7,8,9,10\n\
                   11,12,13,14,15\n";
        let test_file = temp_file::make_path("test_insert_select/tst_TEST.csv");

        ds.create().unwrap();
        assert!(!ds.symbol_exists(&tag, &symbol));

        ds.insert_symbol(&tag, &symbol, &csv).unwrap();
        assert!(ds.symbol_exists(&tag, &symbol));
        assert!(test_file.exists());

        let data = ds.select_symbol(&tag, &symbol).unwrap();
        let dvec: Vec<&str> = data.split('\n').collect();
        assert_eq!(dvec.len(), 4);
        assert_eq!(dvec[0], "1,2,3,4,5");
        assert_eq!(dvec[1], "6,7,8,9,10");
        assert_eq!(dvec[2], "11,12,13,14,15");
        assert_eq!(dvec[3], "");

        ds.delete().unwrap();
        assert!(!base_path.exists());
    }

    #[test]
    fn test_datastore_insert_dup_first_column_select() {
        let root = env::temp_dir();
        let base_path = temp_file::make_path("test_insert_dup_select");
        let ds = DataStore::new(root.to_str().unwrap(), "test_insert_dup_select");

        let tag = "tst";
        let symbol = "TEST";
        let csv = "1,2,3,4,5\n\
                   1,2,3,4,5\n\
                   6,7,8,9,10\n\
                   6,7,8,9,10\n\
                   6,7,8,9,10\n\
                   11,12,13,14,15\n";
        let test_file = temp_file::make_path("test_insert_dup_select/tst_TEST.csv");

        ds.create().unwrap();
        assert!(!ds.symbol_exists(&tag, &symbol));

        ds.insert_symbol(&tag, &symbol, &csv).unwrap();
        assert!(ds.symbol_exists(&tag, &symbol));
        assert!(test_file.exists());

        let data = ds.select_symbol(&tag, &symbol).unwrap();
        let dvec: Vec<&str> = data.split('\n').collect();
        assert_eq!(dvec.len(), 4);
        assert_eq!(dvec[0], "1,2,3,4,5");
        assert_eq!(dvec[1], "6,7,8,9,10");
        assert_eq!(dvec[2], "11,12,13,14,15");
        assert_eq!(dvec[3], "");

        ds.delete().unwrap();
        assert!(!base_path.exists());
    }

    #[test]
    fn test_datastore_insert_select_last() {
        let root = env::temp_dir();
        let base_path = temp_file::make_path("test_insert_select_last");
        let ds = DataStore::new(root.to_str().unwrap(), "test_insert_select_last");

        let tag = "tst";
        let symbol = "TEST";
        let csv = "1,2,3,4,5\n\
                   6,7,8,9,10\n\
                   11,12,13,14,15\n";

        ds.create().unwrap();
        assert!(!ds.symbol_exists(&tag, &symbol));

        ds.insert_symbol(&tag, &symbol, &csv).unwrap();
        assert!(ds.symbol_exists(&tag, &symbol));

        let data = ds.select_last(&tag, &symbol).unwrap();
        let dvec: Vec<&str> = data.split('\n').collect();
        assert_eq!(dvec.len(), 1);
        assert_eq!(dvec[0], "11,12,13,14,15");

        ds.delete().unwrap();
        assert!(!base_path.exists());
    }

    #[test]
    fn test_datastore_insert_with_header() {
        let root = env::temp_dir();
        let base_path = temp_file::make_path("test_insert_with_header");
        let ds = DataStore::new(root.to_str().unwrap(), "test_insert_with_header");

        let tag = "tst";
        let symbol = "TEST";
        let csv = "A,B,C,D,E\n\
                   1,2,3,4,5\n\
                   6,7,8,9,10\n\
                   11,12,13,14,15\n";
        let test_file = temp_file::make_path("test_insert_with_header/tst_TEST.csv");

        ds.create().unwrap();
        ds.insert_symbol(&tag, &symbol, &csv).unwrap();
        assert!(test_file.exists());

        let data = ds.select_symbol(&tag, &symbol).unwrap();
        let dvec: Vec<&str> = data.split('\n').collect();
        assert_eq!(dvec.len(), 4);
        assert_eq!(dvec[0], "1,2,3,4,5");
        assert_eq!(dvec[1], "6,7,8,9,10");
        assert_eq!(dvec[2], "11,12,13,14,15");
        assert_eq!(dvec[3], "");

        ds.delete().unwrap();
        assert!(!base_path.exists());
    }

    #[test]
    fn test_datastore_insert_dup_first_column_with_header() {
        let root = env::temp_dir();
        let base_path = temp_file::make_path("test_insert_dup_with_header");
        let ds = DataStore::new(root.to_str().unwrap(), "test_insert_dup_with_header");

        let tag = "tst";
        let symbol = "TEST";
        let csv = "A,B,C,D,E\n\
                   1,2,3,4,5\n\
                   1,2,3,4,5\n\
                   6,7,8,9,10\n\
                   6,7,8,9,10\n\
                   6,7,8,9,10\n\
                   11,12,13,14,15\n";
        let test_file = temp_file::make_path("test_insert_dup_with_header/tst_TEST.csv");

        ds.create().unwrap();
        ds.insert_symbol(&tag, &symbol, &csv).unwrap();
        assert!(test_file.exists());

        let data = ds.select_symbol(&tag, &symbol).unwrap();
        let dvec: Vec<&str> = data.split('\n').collect();
        assert_eq!(dvec.len(), 4);
        assert_eq!(dvec[0], "1,2,3,4,5");
        assert_eq!(dvec[1], "6,7,8,9,10");
        assert_eq!(dvec[2], "11,12,13,14,15");
        assert_eq!(dvec[3], "");

        ds.delete().unwrap();
        assert!(!base_path.exists());
    }

    #[test]
    fn test_datastore_append() {
        let root = env::temp_dir();
        let base_path = temp_file::make_path("test_append");
        let ds = DataStore::new(root.to_str().unwrap(), "test_append");

        let tag = "tst";
        let symbol = "TEST";
        let csv = "1,2,3,4,5\n\
                   6,7,8,9,10\n\
                   11,12,13,14,15";
        let extra_csv = "16,17,18,19,20\n\
                         21,22,23,24,25\n";

        ds.create().unwrap();
        ds.insert_symbol(&tag, &symbol, &csv).unwrap();
        ds.insert_symbol(&tag, &symbol, &extra_csv).unwrap();

        let data = ds.select_symbol(&tag, &symbol).unwrap();
        let dvec: Vec<&str> = data.split('\n').collect();
        assert_eq!(dvec.len(), 6);
        assert_eq!(dvec[0], "1,2,3,4,5");
        assert_eq!(dvec[1], "6,7,8,9,10");
        assert_eq!(dvec[2], "11,12,13,14,15");
        assert_eq!(dvec[3], "16,17,18,19,20");
        assert_eq!(dvec[4], "21,22,23,24,25");
        assert_eq!(dvec[5], "");

        ds.delete().unwrap();
        assert!(!base_path.exists());
    }

    #[test]
    fn test_datastore_append_dup_first_column() {
        let root = env::temp_dir();
        let base_path = temp_file::make_path("test_append_dup");
        let ds = DataStore::new(root.to_str().unwrap(), "test_append_dup");

        let tag = "tst";
        let symbol = "TEST";
        let csv = "1,2,3,4,5\n\
                   6,7,8,9,10\n\
                   6,7,8,9,10";
        let extra_csv = "11,12,13,14,15\n\
                         11,12,13,14,15\n\
                         16,17,18,19,20\n\
                         16,17,18,19,20\n\
                         16,17,18,19,20\n\
                         21,22,23,24,25\n";

        ds.create().unwrap();
        ds.insert_symbol(&tag, &symbol, &csv).unwrap();
        ds.insert_symbol(&tag, &symbol, &extra_csv).unwrap();

        let data = ds.select_symbol(&tag, &symbol).unwrap();
        let dvec: Vec<&str> = data.split('\n').collect();
        assert_eq!(dvec.len(), 6);
        assert_eq!(dvec[0], "1,2,3,4,5");
        assert_eq!(dvec[1], "6,7,8,9,10");
        assert_eq!(dvec[2], "11,12,13,14,15");
        assert_eq!(dvec[3], "16,17,18,19,20");
        assert_eq!(dvec[4], "21,22,23,24,25");
        assert_eq!(dvec[5], "");

        ds.delete().unwrap();
        assert!(!base_path.exists());
    }

    #[test]
    fn test_datastore_append_with_header() {
        let root = env::temp_dir();
        let base_path = temp_file::make_path("test_append_with_header");
        let ds = DataStore::new(root.to_str().unwrap(), "test_append_with_header");

        let tag = "tst";
        let symbol = "TEST";
        let csv = "AA,BB,CC,DD,EE\n\
                   1,2,3,4,5\n\
                   6,7,8,9,10\n\
                   11,12,13,14,15";
        let extra_csv = "AA,BB,CC,DD,EE\n\
                         16,17,18,19,20\n\
                         21,22,23,24,25\n";

        ds.create().unwrap();
        ds.insert_symbol(&tag, &symbol, &csv).unwrap();
        ds.insert_symbol(&tag, &symbol, &extra_csv).unwrap();

        let data = ds.select_symbol(&tag, &symbol).unwrap();
        let dvec: Vec<&str> = data.split('\n').collect();
        assert_eq!(dvec.len(), 6);
        assert_eq!(dvec[0], "1,2,3,4,5");
        assert_eq!(dvec[1], "6,7,8,9,10");
        assert_eq!(dvec[2], "11,12,13,14,15");
        assert_eq!(dvec[3], "16,17,18,19,20");
        assert_eq!(dvec[4], "21,22,23,24,25");
        assert_eq!(dvec[5], "");

        ds.delete().unwrap();
        assert!(!base_path.exists());
    }

    #[test]
    fn test_datastore_drop() {
        let root = env::temp_dir();
        let base_path = temp_file::make_path("test_drop");
        let ds = DataStore::new(root.to_str().unwrap(), "test_drop");

        let tag = "tst";
        let symbol = "TEST";
        let csv = "1,2,3,4,5\n\
                   6,7,8,9,10\n\
                   11,12,13,14,15\n";
        let test_file = temp_file::make_path("test_drop/tst_TEST.csv");

        ds.create().unwrap();
        ds.insert_symbol(&tag, &symbol, &csv).unwrap();
        assert!(test_file.exists());

        ds.drop_symbol(&tag, &symbol).unwrap();
        assert!(!test_file.exists());

        ds.delete().unwrap();
        assert!(!base_path.exists());
    }
}
