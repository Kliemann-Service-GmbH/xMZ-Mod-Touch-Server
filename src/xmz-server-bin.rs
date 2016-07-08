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
    server.init();

    server.modbus_device = "/dev/ttyUSB0";
    server.modules.push(Module::new(ModuleType::RAGAS_CO_NO2));
    server.modules.push(Module::new(ModuleType::RAGAS_CO_NO2));
    server.modules.push(Module::new(ModuleType::RAGAS_CO_NO2));
    server.modules.push(Module::new(ModuleType::RAGAS_CO_NO2));
    server.modules.push(Module::new(ModuleType::RAGAS_CO_NO2));
    server.modules.push(Module::new(ModuleType::RAGAS_CO_NO2));
    server.modules[0].modbus_slave_id = 23;
    server.modules[1].modbus_slave_id = 24;
    server.modules[2].modbus_slave_id = 25;
    server.modules[3].modbus_slave_id = 26;
    server.modules[4].modbus_slave_id = 27;
    server.modules[5].modbus_slave_id = 28;

    let server = Arc::new(RwLock::new(server));
    let server1 = server.clone();
    let server2 = server.clone();

    let guard = thread::spawn(move || {
        loop {
            let server1 = server1.clone();
            let update_task = thread::spawn(move || {
                let mut server1 = server1.write().unwrap();
                println!("Update");
                server1.update_sensors();
                // thread::sleep(Duration::from_millis(2000));
            });

            let server2 = server2.clone();
            let worker_task = thread::spawn(move || {
                match server2.read() {
                    Ok(server) => {
                        println!("Do work");
                        println!("{}", &server.modules[0].sensors[0].adc_value.unwrap_or(0));
                    }
                    Err(err) => { println!("Error while lock: {}", err) }
                }
                thread::sleep(Duration::from_millis(1000));
            });
            worker_task.join();
        }
    });
    guard.join();
}
