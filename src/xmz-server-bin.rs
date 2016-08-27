// TODO: gernerische FUnktion LED/ RELAIS reset (alles auf Null)
//
extern crate nanomsg;
extern crate xmz_server;

use std::sync::{Arc, RwLock};
use std::thread;
use xmz_server::server::server::Server;


fn main() {
    let mut server = Server::new();
    let _ = server.init();

    let server = Arc::new(RwLock::new(server));
    // Verschiedene Server Instanzen erzeugen, diese werden später in den Threads erneut geklont.

    loop {
        let server_update_sensors = server.clone();
        let server_request_handler = server.clone();

        // 1. Thread zum Update der Sensoren via modbus_stop_bit
        //
        // Dieser Thread muss mindestens einmal durchlauden werden pro loop Zyklus, desshalb
        // hat dieser Thread einen Namen `thread_update_sensors` und desshalb wird der Thread
        // am Ende gejoined `thread_update_sensors.join()`
        let thread_update_sensors = thread::spawn(move || {
            server_update_sensors.write().map(|mut server| {
                tick("thread_update_sensors");
                match server.update_sensors() {
                    Ok(_) => println!("Update Server OK"),
                    Err(err) => println!("Update Server FEHLER: {}", err),
                }
            });
        });
        let _ = thread_update_sensors.join();

        // 2. Thread für die Client Server Kommunikation
        let _thread_request_handler = thread::spawn(move || {
            let _ = server_request_handler.write().map(|mut server| {
                tick("thread_request_handler");
                let _ = server.handle_nanomsg_requests();
            });
        });

    } // Ende loop
}


// Kleiner Helper für eine Statusmeldung aus einem Thread.
//
#[allow(dead_code)]
fn tick(name: &str) {
    println!("tick from: {}", name);
}
