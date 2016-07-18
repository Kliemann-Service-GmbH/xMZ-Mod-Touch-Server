use std::fmt;
use std::error::Error;


#[derive(Debug)]
pub enum ServerError {
    Invalid,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerError::Invalid => write!(f, "Invalid"),
        }
    }
}

impl Error for ServerError {
    fn description(&self) -> &str {
        match *self {
            ServerError::Invalid => "Invalid",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ServerError::Invalid => None,
        }
    }
}
