use serde_json;
use std::error::Error;
use std::fmt;
use std::io;
use std::result;


pub type Result<T> = result::Result<T, XMZModTouchServerError>;

#[derive(Debug)]
pub enum XMZModTouchServerError {
    IoError(io::Error),
    SerdeJson(serde_json::Error),
    ConfigNotFound,
}
impl fmt::Display for XMZModTouchServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            XMZModTouchServerError::IoError(ref err) => err.fmt(f),
            XMZModTouchServerError::SerdeJson(ref err) => err.fmt(f),
            XMZModTouchServerError::ConfigNotFound => write!(f, "Config file not found."),
        }
    }
}
impl Error for XMZModTouchServerError {
    fn description(&self) -> &str {
        match *self {
            XMZModTouchServerError::IoError(ref err) => err.description(),
            XMZModTouchServerError::SerdeJson(ref err) => err.description(),
            XMZModTouchServerError::ConfigNotFound => "Config file not found.",
        }
    }
    fn cause(&self) -> Option<&Error> {
        match *self {
            XMZModTouchServerError::IoError(ref err) => Some(err),
            XMZModTouchServerError::SerdeJson(ref err) => Some(err),
            XMZModTouchServerError::ConfigNotFound => None,
        }
    }
}

impl From<io::Error> for XMZModTouchServerError {
    fn from(err: io::Error) -> XMZModTouchServerError {
        XMZModTouchServerError::IoError(err)
    }
}

impl From<serde_json::Error> for XMZModTouchServerError {
    fn from(err: serde_json::Error) -> XMZModTouchServerError {
        XMZModTouchServerError::SerdeJson(err)
    }
}
