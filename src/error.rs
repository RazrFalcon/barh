use std::io;
use std::num;

use serde_json;
use config;

pub type BarhResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    NumParseError(num::ParseIntError),
    JsonError(serde_json::Error),
    ConfigError(config::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Error {
        Error::IoError(value)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(value: num::ParseIntError) -> Error {
        Error::NumParseError(value)
    }
}

impl From<config::Error> for Error {
    fn from(value: config::Error) -> Error {
        Error::ConfigError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Error {
        Error::JsonError(value)
    }
}
