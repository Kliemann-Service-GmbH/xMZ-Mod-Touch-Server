// TODO: gernerische FUnktion LED/ RELAIS reset (alles auf Null)
//
#[macro_use] extern crate clap;
extern crate nanomsg;
extern crate xmz_server;

use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use xmz_server::server::server_command::{ServerCommand};
use xmz_server::server::server_error::ServerError;
use xmz_server::server::server::Server;


fn tick(name: &str) {
    println!("tick from: {}", name);
}

fn main() {
    // Pull version information out of Cargo.toml
    let version = format!("{}.{}.{}{}",
                env!("CARGO_PKG_VERSION_MAJOR"),
                env!("CARGO_PKG_VERSION_MINOR"),
                env!("CARGO_PKG_VERSION_PATCH"),
                option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""));

    // Comand Line Interface
    let matches = clap_app!(xmz_server =>
        (version: version.as_str())
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: env!("CARGO_PKG_DESCRIPTION"))
    ).get_matches();

    let mut counter = 0;
    let mut server = Server::new();
    let _ = server.init();

    let server = Arc::new(RwLock::new(server));
    // Verschiedene Server Instanzen erzeugen, diese werden später in den Threads erneut geklont.

    loop {

        let server1 = server.clone();
        let server2 = server.clone();
        let server3 = server.clone();


        // Im main loop werden verschiedene Threads gespannt die jeweils Teilaufgaben des Servers
        // ab arbeiten.
        let guard = thread::spawn(move || {
            // 1. Task, Update Sensoren, LED und Relais
            let server1 = server1.clone();
            let update_task = thread::spawn(move || {
                let _ = match server1.write() {
                    Ok(mut server) => { server.update_sensors(); }
                    Err(err) => { println!("Thread Update Task: Fehler beim write lock des Servers: {}", err); }
                };
                tick("Thread1");
            });
            let _ = update_task.join();


            // // 2. Thread zur Zeit Ausgabe der Sensorwerte
            // let server2 = server2.clone();
            // let _worker_task = thread::spawn(move || {
            //     match server2.read() {
            //         Ok(server) => {
            //             for module in &server.modules[..] {
            //                 for sensor in module.sensors.iter() {
            //                     println!("{}: ({}) {:.2} {} [{}]", module.get_modbus_slave_id(), sensor.sensor_type, sensor.concentration().unwrap_or(0.0), sensor.si, sensor.adc_value.unwrap_or(0));
            //                 }
            //             }
            //             thread::sleep(Duration::from_millis(1000));
            //             print!("{}[2J", 27 as char);
            //         }
            //         Err(err) => { println!("Error while lock: {}", err) }
            //     };
            //     tick("Thread2");
            // });
            // let _ = _worker_task.join();

            // 3. Task
            let server3 = server3.clone();
            let _nanomsg_server = thread::spawn(move || {
                let _ = match server3.write() {
                    Ok(mut server) => { server.handle_nanomsg_requests(); },
                    Err(err) => { println!("Thread Nanomsg Server: Fehler beim write lock des Servers: {}", err); },
                };
            });
            // let _ = _nanomsg_server.join();

        });
        let _ = guard.join();



    } // Ende loop
}
