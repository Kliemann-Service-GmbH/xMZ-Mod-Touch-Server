extern crate xmz_server;
extern crate nanomsg;

use nanomsg::{Socket, Protocol};
use std::cell::RefCell;
use std::io::{Read, Write};
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use xmz_server::module::{Module, ModuleType};
use xmz_server::nanomsg_device::NanoMsgDevice;
use xmz_server::server::server::Server;
use xmz_server::server::server_command::{ServerCommand};
use std::str::FromStr;

fn main() {
    let mut server = Server::new();
    let device = NanoMsgDevice::create();
    server.default_configuration();

    let server = Arc::new(RwLock::new(server));
    // Verschiedene Server Instanzen erzeugen, diese werden später in den Threads erneut geklont.
    let server1 = server.clone();
    let server2 = server.clone();
    let server3 = server.clone();

    let guard = thread::spawn(move || {
        loop {

            // 1. Task, Update Sensoren, LED und Relais
            let server1 = server1.clone();
            let update_task = thread::spawn(move || {
                let mut server = server1.write().expect("Fehler beim write lock des Servers");
                server.update_sensors();
                // server.leds.shift_out();
                // server.relais.shift_out();
            });
            // update_task.join();

            // // 2. Thread zur Zeit Ausgabe der Sensorwerte
            // let server2 = server2.clone();
            // let worker_task = thread::spawn(move || {
            //     match server2.read() {
            //         Ok(server) => {
            //             for module in &server.modules[..] {
            //                 for sensor in module.sensors.iter() {
            //                     println!("{}: ({}) {:.2} {} [{}]", module.modbus_slave_id, sensor.sensor_type, sensor.concentration().unwrap_or(0.0), sensor.si, sensor.adc_value.unwrap_or(0));
            //                 }
            //             }
            //             print!("{}[2J", 27 as char);
            //         }
            //         Err(err) => { println!("Error while lock: {}", err) }
            //     };
            //     thread::sleep(Duration::from_millis(1000));
            // });
            // worker_task.join();

            // 3. Task
            let server3 = server3.clone();
            let nanomsg_server = thread::spawn(move || {
                let mut server = server3.write().expect("Fehler beim write lock des Servers");

                match Socket::new(Protocol::Rep) {
                    Ok(mut socket) => {
                        match socket.connect("ipc:///tmp/xmz-server.ipc") {
                            Ok(mut endpoint) => {
                                let mut request = String::new();

                                println!("Nanomsg Server ist bereit");

                                loop {
                                    match socket.read_to_string(&mut request) {
                                        Ok(_) => {
                                            println!("Server Empfang: {}", request);
                                            let server_command = ServerCommand::from_str(&request).unwrap();
                                            server.execute(server_command);

                                            match socket.write_all("OK".as_bytes()) {
                                                Ok(..) => { println!("Server sendet OK"); }
                                                Err(err) => {
                                                    println!("Server konnte nicht OK senden");
                                                    break
                                                }
                                            }
                                            request.clear();
                                        }
                                        Err(err) => {
                                            println!("Server konnte Anfrage nicht verarbeiten: {}", err);
                                        }
                                    }
                                    request.clear();
                                }
                                let _ = endpoint.shutdown();
                            }
                            Err(err) => { println!("Fehler beim Erstellen des Nanomsg Endpoints: {}", err); }
                        }
                    }
                    Err(err) => { println!("Fehler beim Erstellen des Nanomsg Sockets: {}", err); }
                }
            });
            nanomsg_server.join();

        } // Ende loop
    });
    guard.join();
}
