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

        let server1 = server.clone();
        let server2 = server.clone();


        // Im main loop werden verschiedene Threads gespannt die jeweils Teilaufgaben des Servers
        // ab arbeiten.
        let guard = thread::spawn(move || {
            // 1. Task, Update Sensoren, LED und Relais
            let update_task = thread::spawn(move || {
                let _ = match server1.write() {
                    Ok(mut server) => { server.update_sensors(); }
                    Err(err) => {
                        //println!("Thread Update Task: Fehler beim write lock des Servers: {}", err);
                    }
                };
                tick("Thread1");
            });
            let _ = update_task.join();


            // 2. Task
            let _nanomsg_server = thread::spawn(move || {
                let _ = match server2.write() {
                    Ok(mut server) => { server.handle_nanomsg_requests(); },
                    Err(err) => {
                        //println!("Thread Nanomsg Server: Fehler beim write lock des Servers: {}", err);
                    },
                };
                tick("Thread2");
            });
            let _ = _nanomsg_server.join();

        });
        let _ = guard.join();



    } // Ende loop
}
