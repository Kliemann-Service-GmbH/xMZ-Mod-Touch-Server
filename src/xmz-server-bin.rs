extern crate xmz_server;
extern crate nanomsg;

use nanomsg::{Socket, Protocol};
use std::io::{Read};
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use xmz_server::nanomsg_device::NanoMsgDevice;
use xmz_server::server::server_command::{ServerCommand};
use xmz_server::server::server::Server;

fn main() {
    let mut server = Server::new();
    let _device = NanoMsgDevice::create();
    server.default_configuration();

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
            });
            let _ = update_task.join();

            // 2. Thread zur Zeit Ausgabe der Sensorwerte
            let server2 = server2.clone();
            let worker_task = thread::spawn(move || {
                match server2.read() {
                    Ok(server) => {
                        for module in &server.modules[..] {
                            for sensor in module.sensors.iter() {
                                println!("{}: ({}) {:.2} {} [{}]", module.get_modbus_slave_id(), sensor.sensor_type, sensor.concentration().unwrap_or(0.0), sensor.si, sensor.adc_value.unwrap_or(0));
                            }
                        }
                        print!("{}[2J", 27 as char);
                    }
                    Err(err) => { println!("Error while lock: {}", err) }
                };
                thread::sleep(Duration::from_millis(1000));
            });
            let _ = worker_task.join();

            // 3. Task
            let server3 = server3.clone();
            let nanomsg_server = thread::spawn(move || {
                let _ = match server3.write() {
                    Ok(mut server) => {
                        // Erstelle Nanomsg Socket
                        match Socket::new(Protocol::Rep) {
                            Ok(mut socket) => {
                                let _ = socket.set_send_timeout(1000);
                                // socket.set_receive_timeout(1000);

                                // Connect Nanomsg socket (Verbindung zum Nanomsg Device siehe nanomsg_device)
                                match socket.connect("ipc:///tmp/xmz-server.ipc") {
                                    // Wenn ein endpoint bereit ist nutze den
                                    Ok(mut endpoint) => {
                                        let mut request = String::new();

                                        match socket.read_to_string(&mut request) {
                                            Ok(_) => {
                                                println!("Server Empfang: {}", request);
                                                // ServerCommand aus dem Request formen
                                                match ServerCommand::from_str(&request) {
                                                    Ok(server_command) => { server.execute(server_command, &mut socket); }
                                                    Err(err) => { println!("Fehler beim Auswerten des Server Commands: {}", err); }
                                                }
                                                request.clear();
                                            }
                                            Err(err) => {
                                                println!("Server konnte Anfrage nicht verarbeiten: {}", err);
                                            }
                                        }
                                        request.clear();
                                        let _ = endpoint.shutdown();
                                    }
                                    // Wenn kein Endpoint bereit steht gib ein Fehler aus
                                    Err(err) => { println!("Fehler beim Erstellen des Nanomsg Endpoints: {}", err); }
                                }
                            }
                            // Wenn beim Erstellen des Sockets ein Fehler auftritt, gib eine Meldung aus.
                            Err(err) => { println!("Fehler beim Erstellen des Nanomsg Sockets: {}", err); }
                        }
                    }
                    Err(err) => { println!("Thread Nanomsg Server: Fehler beim write lock des Servers: {}", err); }
                };
                thread::sleep(Duration::from_millis(100));
            });
            let _ = nanomsg_server.join();

        });
        let _ = guard.join();
    } // Ende loop
}
