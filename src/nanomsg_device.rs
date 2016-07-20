use nanomsg::{Protocol, Socket};
use std::thread;
use nanomsg;
use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub enum DeviceError {
    Nanomsg(nanomsg::Error),
}

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeviceError::Nanomsg(ref err) => err.fmt(f),
        }
    }
}

impl Error for DeviceError {
    fn description(&self) -> &str {
        match *self {
            DeviceError::Nanomsg(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            DeviceError::Nanomsg(ref err) => Some(err),
        }
    }
}

impl From<nanomsg::Error> for DeviceError {
    fn from(err: nanomsg::Error) -> DeviceError {
        DeviceError::Nanomsg(err)
    }
}

pub struct NanomsgDevice {}

impl NanomsgDevice {
    /// Erzeugt und startet das NonoMsg Device
    pub fn create() -> Result<(), DeviceError> {
        let mut front_socket   = try!(Socket::new_for_device(Protocol::Rep));
        let mut front_endpoint = try!(front_socket.bind("ipc:///tmp/xmz-client.ipc"));
        let mut back_socket    = try!(Socket::new_for_device(Protocol::Req));
        let mut back_endpoint  = try!(back_socket.bind("ipc:///tmp/xmz-server.ipc"));

        let _device_thread = thread::spawn(move || {
            println!("Device ist bereit.");
            let _ = Socket::device(&front_socket, &back_socket);
            println!("Device beendet!");

            let _ = front_endpoint.shutdown();
            let _ = back_endpoint.shutdown();
        });
        Ok(())
    }
}
