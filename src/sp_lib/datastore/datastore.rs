use std::io::prelude::*;
use std::fs;
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

    pub fn create(&self) -> Result<(), Box<dyn Error>> {
        if self.exists() {
            Err(format!("Datastore '{}' already exists", self.name).into())
        }
        else {
            fs::create_dir(self.base_path.as_path())?;
            Ok(())
        }
    }

    pub fn load(&self, symbol: &str) -> Result<String, Box<dyn Error>> {
        let sym_file = DataStore::make_symbol_file(&self.base_path, symbol);
        let file = fs::File::open(sym_file.as_path())?;
        let mut reader = BufReader::new(&file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        Ok(content)
    }

    pub fn store(&self, symbol: &str, csv: &str) -> Result<(), Box<dyn Error>> {
        if !csv.is_empty() {
            let sym_file = DataStore::make_symbol_file(&self.base_path, symbol);
            let exists = sym_file.exists();
            let mut file =
                if exists {
                    fs::OpenOptions::new().write(true).open(sym_file)?
                } else {
                    fs::File::create(sym_file)?
                };
            file.seek(std::io::SeekFrom::End(0))?;
            write!(file, "{}", csv)?;
            if !csv.ends_with("\n") {
                write!(file, "\n")?;
            }
        }
        Ok(())
    }

    fn make_base_path(root: &str, name: &str) -> PathBuf {
        let mut pbuf = PathBuf::from(root);
        pbuf.push(name);
        pbuf
    }

    fn make_symbol_file(base: &PathBuf, symbol: &str) -> PathBuf {
        let mut pbuf = base.clone();
        pbuf.push(&format!("{}.csv", symbol));
        pbuf
    }
}

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::temp_file;
    use std::env;
    use std::fs;

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
    fn test_datastore_create() {
        let root = env::temp_dir();
        let base_path = temp_file::make_path("test_create");
        let ds = DataStore::new(root.to_str().unwrap(), "test_create");
        assert!(!ds.exists());
        assert!(ds.create().is_ok());
        assert!(ds.create().is_err());
        assert!(ds.exists());
        assert!(base_path.exists());
        assert!(fs::remove_dir(base_path.as_path()).is_ok());
    }

    #[test]
    fn test_datastore_store_load() {
        let root = env::temp_dir();
        let base_path = temp_file::make_path("test_store_load");
        let ds = DataStore::new(root.to_str().unwrap(), "test_store_load");

        let symbol = "TEST";
        let csv = "1,2,3,4,5\n\
                   6,7,8,9,10\n\
                   11,12,13,14,15\n";
        let test_file = temp_file::make_path("test_store_load/TEST.csv");

        ds.create().unwrap();
        ds.store(&symbol, &csv).unwrap();
        assert!(test_file.exists());

        let data = ds.load(&symbol).unwrap();
        let dvec: Vec<&str> = data.split('\n').collect();
        assert_eq!(dvec.len(), 4);
        assert_eq!(dvec[0], "1,2,3,4,5");
        assert_eq!(dvec[1], "6,7,8,9,10");
        assert_eq!(dvec[2], "11,12,13,14,15");
        assert_eq!(dvec[3], "");

        assert!(temp_file::remove_file(test_file.to_str().unwrap()));
        assert!(fs::remove_dir(base_path.as_path()).is_ok());
    }

    #[test]
    fn test_datastore_append() {
        let root = env::temp_dir();
        let base_path = temp_file::make_path("test_store_append");
        let ds = DataStore::new(root.to_str().unwrap(), "test_store_append");

        let symbol = "TEST";
        let csv = "1,2,3,4,5\n\
                   6,7,8,9,10\n\
                   11,12,13,14,15";
        let extra_csv = "16,17,18,19,20\n\
                         21,22,23,24,25\n";
        let test_file = temp_file::make_path("test_store_append/TEST.csv");

        ds.create().unwrap();
        ds.store(&symbol, &csv).unwrap();
        ds.store(&symbol, &extra_csv).unwrap();

        let data = ds.load(&symbol).unwrap();
        let dvec: Vec<&str> = data.split('\n').collect();
        assert_eq!(dvec.len(), 6);
        assert_eq!(dvec[0], "1,2,3,4,5");
        assert_eq!(dvec[1], "6,7,8,9,10");
        assert_eq!(dvec[2], "11,12,13,14,15");
        assert_eq!(dvec[3], "16,17,18,19,20");
        assert_eq!(dvec[4], "21,22,23,24,25");
        assert_eq!(dvec[5], "");

        assert!(temp_file::remove_file(test_file.to_str().unwrap()));
        assert!(fs::remove_dir(base_path.as_path()).is_ok());
    }
}
