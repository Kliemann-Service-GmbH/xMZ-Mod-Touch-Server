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

    let server = Arc::new(RwLock::new(server));
    // Verschiedene Server Instanzen erzeugen, diese werden später in den Threads erneut geklont.

    loop {
        let server_update_sensors = server.clone();
        let server_request_handler = server.clone();

        let thread_update_sensors = thread::spawn(move || {
            server_update_sensors.write().map(|mut server| {
                tick("thread_update_sensors");
                server.update_sensors();
            });
        });
        thread_update_sensors.join();

        let thread_request_handler = thread::spawn(move || {
            server_request_handler.write().map(|mut server| {
                tick("thread_request_handler");
                server.handle_nanomsg_requests();
            });
        });
        // thread_request_handler.join();

        // let _ = thread::spawn(move || {
        //     server_request_handler.write().map(|mut server| {
        //         tick("Handle Request");
        //         server.handle_nanomsg_requests();
        //     })
        // });
    } // Ende loop
}
