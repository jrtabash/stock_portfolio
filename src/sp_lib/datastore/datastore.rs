use std::io::prelude::*;
use std::fmt;
use std::fs;
use std::str;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use crate::util::error::Error;

type FtnResult = Result<(), Error>;

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
            base_path: DataStore::make_base_path(root, name)
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

    pub fn create(&self) -> Result<(), Error> {
        if self.exists() {
            Err(format!("Datastore '{}' already exists", self.name).into())
        }
        else {
            fs::create_dir(self.base_path.as_path())?;
            Ok(())
        }
    }

    pub fn delete(&self) -> Result<(), Error> {
        if self.exists() {
            fs::remove_dir_all(self.base_path.as_path())?;
            Ok(())
        }
        else {
            Err(format!("Datastore '{}' does not exist", self.name).into())
        }
    }

    pub fn read_file(&self, sym_file: &Path) -> Result<String, Error> {
        let file = fs::File::open(sym_file)?;
        let mut reader = BufReader::new(&file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        Ok(content)
    }

    pub fn read_last_n(&self, sym_file: &Path, n: usize) -> Result<String, Error> {
        let mut file = fs::File::open(sym_file)?;

        let meta_data = file.metadata()?;
        let file_size = meta_data.len();
        let mut position = if file_size > 1 { file_size - 2 } else { 0 };

        file.seek(std::io::SeekFrom::Start(position))?;

        let mut buf = [0; 1];
        let mut count = 0;
        while position > 0 {
            if file.read(&mut buf)? > 0 && str::from_utf8(&buf)? == "\n" {
                count += 1;
                if count >= n {
                    break;
                }
            }
            position -= 1;
            file.seek(std::io::SeekFrom::Start(position))?;
        }

        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(String::from(content.trim()))
    }

    pub fn read_last(&self, sym_file: &Path) -> Result<String, Error> {
        self.read_last_n(sym_file, 1)
    }

    pub fn select_symbol(&self, tag: &str, symbol: &str) -> Result<String, Error> {
        let sym_file = DataStore::make_symbol_file(&self.base_path, tag, symbol);
        self.read_file(&sym_file)
    }

    pub fn select_last_n(&self, tag: &str, symbol: &str, n: usize) -> Result<String, Error> {
        let sym_file = DataStore::make_symbol_file(&self.base_path, tag, symbol);
        self.read_last_n(&sym_file, n)
    }

    pub fn select_last(&self, tag: &str, symbol: &str) -> Result<String, Error> {
        let sym_file = DataStore::make_symbol_file(&self.base_path, tag, symbol);
        self.read_last(&sym_file)
    }

    pub fn show_symbol(&self, tag: &str, symbol: &str) -> Result<(), Error> {
        let sym_file = DataStore::make_symbol_file(&self.base_path, tag, symbol);
        println!("{}", self.read_file(&sym_file)?.trim());
        Ok(())
    }

    pub fn insert_symbol(&self, tag: &str, symbol: &str, csv: &str) -> Result<usize, Error> {
        // Skip non-numeric header if one exists.
        let csv_ref =
            match csv.find(char::is_numeric) {
                Some(pos) => {
                    if pos > 0 { &csv[pos..] } else { csv }
                },
                None => ""
            };

        let mut count: usize = 0;
        if !csv_ref.is_empty() {
            let sym_file = DataStore::make_symbol_file(&self.base_path, tag, symbol);
            let exists = sym_file.exists();
            let file_last_line = if exists { self.read_last(&sym_file)? } else { String::new() };

            let mut file =
                if exists {
                    fs::OpenOptions::new().write(true).open(sym_file)?
                } else {
                    fs::File::create(sym_file)?
                };
            file.seek(std::io::SeekFrom::End(0))?;

            let mut last_line: Option<&str> = if !file_last_line.is_empty() { Some(&file_last_line) } else { None };
            for line in csv_ref.trim().split('\n') {
                // Expect and ignore consecutive lines that start with the same date
                if let Some(last) = last_line {
                    if let Some(comma) = last.find(',') {
                        if line.starts_with(&last[..comma]) {
                            continue;
                        }
                    }
                }
                writeln!(file, "{}", line)?;
                last_line = Some(line);
                count += 1;
            }
        }
        Ok(count)
    }

    pub fn drop_symbol(&self, tag: &str, symbol: &str) -> Result<(), Error> {
        let sym_file = DataStore::make_symbol_file(&self.base_path, tag, symbol);
        fs::remove_file(sym_file.as_path())?;
        Ok(())
    }

    pub fn foreach_entry<T>(&self,
                            init: T,
                            ftn: impl Fn(&fs::DirEntry, &mut T) -> FtnResult,
                            filter: impl Fn(&str) -> bool,
                            on_error: impl Fn(&fs::DirEntry, Error) -> FtnResult) -> Result<(T, usize, usize), Error> {
        let mut aggregate: T = init;
        let mut itm_count: usize = 0;
        let mut err_count: usize = 0;
        for entry in fs::read_dir(self.base_path())? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_file() {
                if let Some(entry_str) = entry_path.to_str() {
                    if !filter(entry_str) {
                        continue;
                    }
                }

                itm_count += 1;
                if let Err(err) = ftn(&entry, &mut aggregate) {
                    err_count += 1;
                    on_error(&entry, err)?;
                }
            }
        }
        Ok((aggregate, itm_count, err_count))
    }

    fn make_base_path(root: &str, name: &str) -> PathBuf {
        let mut pbuf = PathBuf::from(root);
        pbuf.push(name);
        pbuf
    }

    fn make_symbol_file(base: &Path, tag: &str, symbol: &str) -> PathBuf {
        let mut pbuf = base.to_path_buf();
        pbuf.push(&format!("{}_{}.csv", tag, symbol));
        pbuf
    }
}

impl fmt::Display for DataStore {
    fn fmt(self: &DataStore, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.base_path().display())
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
        assert_eq!(ds.insert_symbol(&tag, &symbol, &csv).unwrap(), 3);
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

        assert_eq!(ds.insert_symbol(&tag, &symbol, &csv).unwrap(), 3);
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

        assert_eq!(ds.insert_symbol(&tag, &symbol, &csv).unwrap(), 3);
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

        assert_eq!(ds.insert_symbol(&tag, &symbol, &csv).unwrap(), 3);
        assert!(ds.symbol_exists(&tag, &symbol));

        let data = ds.select_last(&tag, &symbol).unwrap();
        let dvec: Vec<&str> = data.split('\n').collect();
        assert_eq!(dvec.len(), 1);
        assert_eq!(dvec[0], "11,12,13,14,15");

        ds.delete().unwrap();
        assert!(!base_path.exists());
    }

    #[test]
    fn test_datastore_insert_select_last_n() {
        let root = env::temp_dir();
        let base_path = temp_file::make_path("test_insert_select_last_n");
        let ds = DataStore::new(root.to_str().unwrap(), "test_insert_select_last_n");

        let tag = "tst";
        let symbol = "TEST";
        let csv = "1,2,3,4,5\n\
                   6,7,8,9,10\n\
                   11,12,13,14,15\n\
                   16,17,18,19,20\n";

        ds.create().unwrap();
        assert!(!ds.symbol_exists(&tag, &symbol));

        assert_eq!(ds.insert_symbol(&tag, &symbol, &csv).unwrap(), 4);
        assert!(ds.symbol_exists(&tag, &symbol));

        let data = ds.select_last_n(&tag, &symbol, 1).unwrap();
        let dvec: Vec<&str> = data.split('\n').collect();
        assert_eq!(dvec.len(), 1);
        assert_eq!(dvec[0], "16,17,18,19,20");

        let data = ds.select_last_n(&tag, &symbol, 2).unwrap();
        let dvec: Vec<&str> = data.split('\n').collect();
        assert_eq!(dvec.len(), 2);
        assert_eq!(dvec[0], "11,12,13,14,15");
        assert_eq!(dvec[1], "16,17,18,19,20");

        let data = ds.select_last_n(&tag, &symbol, 3).unwrap();
        let dvec: Vec<&str> = data.split('\n').collect();
        assert_eq!(dvec.len(), 3);
        assert_eq!(dvec[0], "6,7,8,9,10");
        assert_eq!(dvec[1], "11,12,13,14,15");
        assert_eq!(dvec[2], "16,17,18,19,20");

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
        assert_eq!(ds.insert_symbol(&tag, &symbol, &csv).unwrap(), 3);
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
        assert_eq!(ds.insert_symbol(&tag, &symbol, &csv).unwrap(), 3);
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
        assert_eq!(ds.insert_symbol(&tag, &symbol, &csv).unwrap(), 3);
        assert_eq!(ds.insert_symbol(&tag, &symbol, &extra_csv).unwrap(), 2);

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
        let extra_csv ="6,7,8,9,10\n\
                        6,7,8,9,10\n\
                        11,12,13,14,15\n\
                        11,12,13,14,15\n\
                        16,17,18,19,20\n\
                        16,17,18,19,20\n\
                        16,17,18,19,20\n\
                        21,22,23,24,25\n";

        ds.create().unwrap();
        assert_eq!(ds.insert_symbol(&tag, &symbol, &csv).unwrap(), 2);
        assert_eq!(ds.insert_symbol(&tag, &symbol, &extra_csv).unwrap(), 3);

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
        assert_eq!(ds.insert_symbol(&tag, &symbol, &csv).unwrap(), 3);
        assert_eq!(ds.insert_symbol(&tag, &symbol, &extra_csv).unwrap(), 2);

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
        assert_eq!(ds.insert_symbol(&tag, &symbol, &csv).unwrap(), 3);
        assert!(test_file.exists());

        ds.drop_symbol(&tag, &symbol).unwrap();
        assert!(!test_file.exists());

        ds.delete().unwrap();
        assert!(!base_path.exists());
    }

    #[test]
    fn test_datastore_foreach_entry() {
        let root = env::temp_dir();
        let base_path = temp_file::make_path("test_foreach");
        let ds = DataStore::new(root.to_str().unwrap(), "test_foreach");

        let tag = "tst";
        let symbol1 = "TEST";
        let symbol2 = "FOOO";
        let symbol3 = "BARR";
        let csv = "1,2,3,4,5\n\
                   6,7,8,9,10\n\
                   11,12,13,14,15\n";

        ds.create().unwrap();
        assert_eq!(ds.insert_symbol(&tag, &symbol1, &csv).unwrap(), 3);
        assert_eq!(ds.insert_symbol(&tag, &symbol2, &csv).unwrap(), 3);
        assert_eq!(ds.insert_symbol(&tag, &symbol3, &csv).unwrap(), 3);

        let (sum, items, errors) = ds.foreach_entry(
            0,
            |_, tot| { *tot += 1; Ok(()) },
            |_|      { true },
            |_, err| { Err(err) }
        ).unwrap();
        assert_eq!(sum, 3);
        assert_eq!(items, 3);
        assert_eq!(errors, 0);

        let (sum, items, errors) = ds.foreach_entry(
            0,
            |_, tot|    { *tot += 1; Ok(()) },
            |entry_str| { !entry_str.contains(symbol2) },
            |_, err|    { Err(err) }
        ).unwrap();
        assert_eq!(sum, 2);
        assert_eq!(items, 2);
        assert_eq!(errors, 0);

        let (dummy, items, errors) = ds.foreach_entry(
            0,
            |_, _| { Err("error".into()) },
            |_|    { true },
            |_, _| { Ok(()) }
        ).unwrap();
        assert_eq!(dummy, 0);
        assert_eq!(items, 3);
        assert_eq!(errors, 3);

        let err = ds.foreach_entry(
            0,
            |_, _| { Err("error".into()) },
            |_|    { true },
            |_, e| { Err(e) }
        ).unwrap_err();
        assert_eq!(&format!("{}", err), "error");

        ds.delete().unwrap();
        assert!(!base_path.exists());
    }
}
