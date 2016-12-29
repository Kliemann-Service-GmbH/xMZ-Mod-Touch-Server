use std::error::Error;
use std::{io, result, fmt};


#[derive(Debug)]
pub enum XMZError {
    Io(io::Error),
    NotAllowed,
    SystemCommandFailed,
}

pub type Result<T> = result::Result<T, XMZError>;

impl fmt::Display for XMZError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            XMZError::Io(ref err) => write!(f, "IO error: {}", err),
            XMZError::NotAllowed => write!(f, "Operation nicht erlaubt."),
            XMZError::SystemCommandFailed => {
                write!(f, "System Command konnte nicht ausgeführt werden.")
            }
        }
    }
}

impl Error for XMZError {
    fn description(&self) -> &str {
        match *self {
            XMZError::Io(ref err) => err.description(),
            XMZError::NotAllowed => "Nicht erlaubt",
            XMZError::SystemCommandFailed => "System Command fehlerhaft",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            XMZError::Io(ref err) => Some(err),
            XMZError::NotAllowed => None,
            XMZError::SystemCommandFailed => None,
        }
    }
}

impl From<io::Error> for XMZError {
    fn from(err: io::Error) -> XMZError {
        XMZError::Io(err)
    }
}
