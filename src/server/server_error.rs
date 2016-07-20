use nanomsg_device;
use nanomsg;
use std::error::Error;
use std::fmt;
use std::io;

/// Mögliche Fehler die auftreten können
#[derive(Debug)]
pub enum ServerError {
    Invalid,
    NanomsgDevice(nanomsg_device::DeviceError),
    Nanomsg(nanomsg::Error),
    Io(io::Error),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerError::Invalid => write!(f, "Invalid"),
            ServerError::NanomsgDevice(ref err) => err.fmt(f),
            ServerError::Nanomsg(ref err) => err.fmt(f),
            ServerError::Io(ref err) => err.fmt(f),
        }
    }
}

impl Error for ServerError {
    fn description(&self) -> &str {
        match *self {
            ServerError::Invalid => "Invalid",
            ServerError::NanomsgDevice(ref err) => err.description(),
            ServerError::Nanomsg(ref err) => err.description(),
            ServerError::Io(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ServerError::Invalid => None,
            ServerError::NanomsgDevice(ref err) => Some(err),
            ServerError::Nanomsg(ref err) => Some(err),
            ServerError::Io(ref err) => Some(err),
        }
    }
}

impl From<nanomsg_device::DeviceError> for ServerError {
    fn from(err: nanomsg_device::DeviceError) -> ServerError {
        ServerError::NanomsgDevice(err)
    }
}

impl From<nanomsg::Error> for ServerError {
    fn from(err: nanomsg::Error) -> ServerError {
        ServerError::Nanomsg(err)
    }
}

impl From<io::Error> for ServerError {
    fn from(err: io::Error) -> ServerError {
        ServerError::Io(err)
    }
}
