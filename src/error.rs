use serde_json;
use std::error::Error;
use std::fmt;
use std::io;
use std::result;
use sysfs_gpio::Error as SysfsGpioError;


pub type Result<T> = result::Result<T, XMZModTouchServerError>;

#[derive(Debug)]
pub enum XMZModTouchServerError {
    ConfigNotFound,
    IoError(io::Error),
    SerdeJson(serde_json::Error),
    SysfsGpio(SysfsGpioError),
}

impl fmt::Display for XMZModTouchServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            XMZModTouchServerError::ConfigNotFound => write!(f, "Config file not found."),
            XMZModTouchServerError::IoError(ref err) => err.fmt(f),
            XMZModTouchServerError::SerdeJson(ref err) => err.fmt(f),
            XMZModTouchServerError::SysfsGpio(ref err) => err.fmt(f),
        }
    }
}
impl Error for XMZModTouchServerError {
    fn description(&self) -> &str {
        match *self {
            XMZModTouchServerError::ConfigNotFound => "Config file not found.",
            XMZModTouchServerError::IoError(ref err) => err.description(),
            XMZModTouchServerError::SerdeJson(ref err) => err.description(),
            XMZModTouchServerError::SysfsGpio(ref err) => err.description(),
        }
    }
    fn cause(&self) -> Option<&Error> {
        match *self {
            XMZModTouchServerError::ConfigNotFound => None,
            XMZModTouchServerError::IoError(ref err) => Some(err),
            XMZModTouchServerError::SerdeJson(ref err) => Some(err),
            XMZModTouchServerError::SysfsGpio(ref err) => Some(err),
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

impl From<SysfsGpioError> for XMZModTouchServerError {
    fn from(err: SysfsGpioError) -> XMZModTouchServerError {
        XMZModTouchServerError::SysfsGpio(err)
    }
}
