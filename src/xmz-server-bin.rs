// TODO: gernerische FUnktion LED/ RELAIS reset (alles auf Null)
//
extern crate nanomsg;
extern crate xmz_server;

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use xmz_server::server::server::Server;


fn tick(name: &str) {
    println!("tick from: {}", name);
}

fn main() {
    let mut server = Server::new();
    let _ = server.init();

    // let server = Arc::new(RwLock::new(server));
    // Verschiedene Server Instanzen erzeugen, diese werden später in den Threads erneut geklont.

    loop {
        tick("Update_Sensors");
        server.update_sensors();
        tick("Handle Request");
        server.handle_nanomsg_requests();
    } // Ende loop
}
