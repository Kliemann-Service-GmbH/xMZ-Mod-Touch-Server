extern crate xmz_server;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use xmz_server::module::{Module, ModuleType};
use xmz_server::server::{Server};

fn main() {
    let mut server = Server::new();
    server.default_configuration();

    let server = Arc::new(RwLock::new(server));
    // Verschiedene Server Instanzen erzeugen, diese werden später in den Threads erneut geklont.
    let server1 = server.clone();
    let server2 = server.clone();
    let server3 = server.clone();

    let guard = thread::spawn(move || {
        loop {

            // 1. Task Update der Sensoren
            let server1 = server1.clone();
            let update_task = thread::spawn(move || {
                let mut server1 = server1.write().expect("Fehler beim write lock des Servers");
                server1.update_sensors();
            });
            // update_task.join();

            // 2. Thread zur Zeit Ausgabe der Sensorwerte
            let server2 = server2.clone();
            let worker_task = thread::spawn(move || {
                match server2.read() {
                    Ok(server) => {
                        for module in &server.modules[..] {
                            for sensor in module.sensors.iter() {
                                println!("{}: ({}) {:.2} {} [{}]", module.modbus_slave_id, sensor.sensor_type, sensor.concentration().unwrap_or(0.0), sensor.si, sensor.adc_value.unwrap_or(0));
                            }
                        }
                        print!("{}[2J", 27 as char);
                    }
                    Err(err) => { println!("Error while lock: {}", err) }
                };
                thread::sleep(Duration::from_millis(1000));
            });
            worker_task.join();

            // 3. Task
            let server3 = server3.clone();
            let mut i = 1;
            let nanomsg_server = thread::spawn(move || {
                let mut server = server3.write().expect("Fehler beim write lock des Servers");
                i += 1;
                server.leds.set(i);
                server.leds.shift_out();
                //println!("Tick");
            });
            nanomsg_server.join();

        } // Ende loop
    });
    guard.join();
}
