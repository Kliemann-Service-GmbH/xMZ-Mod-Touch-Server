use nanomsg;
use std::fmt;
use std::io;
use server::server_command;
use libmodbus_rs::modbus;

/// Mögliche Fehler die auftreten können
#[derive(Debug)]
pub enum Error {
    Invalid,
    Nanomsg(nanomsg::Error),
    Io(io::Error),
    ServerCommand(server_command::ServerCommandError),
    Libmodbus(modbus::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Invalid => write!(f, "Invalid"),
            Error::Nanomsg(ref err) => err.fmt(f),
            Error::Io(ref err) => err.fmt(f),
            Error::ServerCommand(ref err) => err.fmt(f),
            Error::Libmodbus(ref err) => err.fmt(f),
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Invalid => "Invalid",
            Error::Nanomsg(ref err) => err.description(),
            Error::Io(ref err) => err.description(),
            Error::ServerCommand(ref err) => err.description(),
            Error::Libmodbus(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::Invalid => None,
            Error::Nanomsg(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::ServerCommand(ref err) => Some(err),
            Error::Libmodbus(ref err) => Some(err),
        }
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

impl From<modbus::Error> for Error {
    fn from(err: modbus::Error) -> Error {
        Error::Libmodbus(err)
    }
}
