use std::fmt;
use std::error::Error;
use nanomsg_device;

#[derive(Debug)]
pub enum ServerError {
    Invalid,
    NanomsgDevice(nanomsg_device::DeviceError),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerError::Invalid => write!(f, "Invalid"),
            ServerError::NanomsgDevice(ref err) => err.fmt(f),
        }
    }
}

impl Error for ServerError {
    fn description(&self) -> &str {
        match *self {
            ServerError::Invalid => "Invalid",
            ServerError::NanomsgDevice(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ServerError::Invalid => None,
            ServerError::NanomsgDevice(ref err) => Some(err),
        }
    }
}

impl From<nanomsg_device::DeviceError> for ServerError {
    fn from(err: nanomsg_device::DeviceError) -> ServerError {
        ServerError::NanomsgDevice(err)
    }
}
