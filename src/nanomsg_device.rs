use nanomsg::{Protocol, Socket};
use std::thread;

pub struct NanoMsgDevice {

}

impl NanoMsgDevice {
    /// Erzeugt und startet das NonoMsg Device
    pub fn create() {
        let mut front_socket = Socket::new_for_device(Protocol::Rep).unwrap();
        let mut front_endpoint = front_socket.bind("ipc:///tmp/xmz-client.ipc").unwrap();
        let mut back_socket = Socket::new_for_device(Protocol::Req).unwrap();
        let mut back_endpoint = back_socket.bind("ipc:///tmp/xmz-server.ipc").unwrap();

        let device_thread = thread::spawn(move || {
            println!("Device ist bereit.");
            Socket::device(&front_socket, &back_socket);
            println!("Device beendet!");

            front_endpoint.shutdown();
            back_endpoint.shutdown();
        });
    }
}
