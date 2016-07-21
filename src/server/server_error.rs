use nanomsg_device;
use nanomsg;
use std::fmt;
use std::io;
use server::server_command;

/// Mögliche Fehler die auftreten können
#[derive(Debug)]
pub enum Error {
    Invalid,
    NanomsgDevice(nanomsg_device::DeviceError),
    Nanomsg(nanomsg::Error),
    Io(io::Error),
    ServerCommand(server_command::ServerCommandError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Invalid => write!(f, "Invalid"),
            Error::NanomsgDevice(ref err) => err.fmt(f),
            Error::Nanomsg(ref err) => err.fmt(f),
            Error::Io(ref err) => err.fmt(f),
            Error::ServerCommand(ref err) => err.fmt(f),
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Invalid => "Invalid",
            Error::NanomsgDevice(ref err) => err.description(),
            Error::Nanomsg(ref err) => err.description(),
            Error::Io(ref err) => err.description(),
            Error::ServerCommand(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::Invalid => None,
            Error::NanomsgDevice(ref err) => Some(err),
            Error::Nanomsg(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::ServerCommand(ref err) => Some(err),
        }
    }
}

impl From<nanomsg_device::DeviceError> for Error {
    fn from(err: nanomsg_device::DeviceError) -> Error {
        Error::NanomsgDevice(err)
    }
}

impl From<nanomsg::Error> for Error {
    fn from(err: nanomsg::Error) -> Error {
        Error::Nanomsg(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<server_command::ServerCommandError> for Error {
    fn from(err: server_command::ServerCommandError) -> Error {
        Error::ServerCommand(err)
    }
}
