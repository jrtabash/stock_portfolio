use std::{env, fmt, io, num, str};
use curl;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    err_msg: String
}

impl Error {
    pub fn new(err_msg: String) -> Self {
        Self { err_msg }
    }

    pub fn from_str(err_msg: &str) -> Self {
        Self { err_msg: err_msg.to_string() }
    }

    #[inline(always)]
    pub fn to_str(&self) -> &str {
        self.err_msg.as_str()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.err_msg)
    }
}

impl From<String> for Error {
    fn from(item: String) -> Self {
        Error::new(item)
    }
}

impl From<&str> for Error {
    fn from(item: &str) -> Self {
        Error::from_str(item)
    }
}

impl From<env::VarError> for Error {
    fn from(item: env::VarError) -> Self {
        Error::new(format!("{item}"))
    }
}

impl From<io::Error> for Error {
    fn from(item: io::Error) -> Self {
        Error::new(format!("{item}"))
    }
}

impl From<str::Utf8Error> for Error {
    fn from(item: str::Utf8Error) -> Self {
        Error::new(format!("{item}"))
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(item: num::ParseFloatError) -> Self {
        Error::new(format!("{item}"))
    }
}

impl From<num::ParseIntError> for Error {
    fn from(item: num::ParseIntError) -> Self {
        Error::new(format!("{item}"))
    }
}

impl From<curl::Error> for Error {
    fn from(item: curl::Error) -> Self {
        Error::new(format!("{item}"))
    }
}
