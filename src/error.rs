extern crate serde_json;

use std::error::Error;
use std::fmt;
use std::io;


#[derive(Debug)]
pub enum XMZServerError {
    IoError(io::Error),
    SerdeJson(serde_json::Error),
    ConfigNotFound,
}
impl fmt::Display for XMZServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            XMZServerError::IoError(ref err) => err.fmt(f),
            XMZServerError::SerdeJson(ref err) => err.fmt(f),
            XMZServerError::ConfigNotFound => write!(f, "Config file not found."),
        }
    }
}
impl Error for XMZServerError {
    fn description(&self) -> &str {
        match *self {
            XMZServerError::IoError(ref err) => err.description(),
            XMZServerError::SerdeJson(ref err) => err.description(),
            XMZServerError::ConfigNotFound => "Config file not found.",
        }
    }
    fn cause(&self) -> Option<&Error> {
        match *self {
            XMZServerError::IoError(ref err) => Some(err),
            XMZServerError::SerdeJson(ref err) => Some(err),
            XMZServerError::ConfigNotFound => None,
        }
    }
}

impl From<io::Error> for XMZServerError {
    fn from(err: io::Error) -> XMZServerError {
        XMZServerError::IoError(err)
    }
}

impl From<serde_json::Error> for XMZServerError {
    fn from(err: serde_json::Error) -> XMZServerError {
        XMZServerError::SerdeJson(err)
    }
}
