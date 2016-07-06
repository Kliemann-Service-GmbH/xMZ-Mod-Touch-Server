extern crate xmz_server;

use xmz_server::module::{Module, ModuleType};
use xmz_server::server::{Server};
use std::rc::Rc;
use std::cell::RefCell;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, RwLock};

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

    // let server = Arc::new(RwLock::new(server));
    // let server = server.clone();
    loop {
        server.update_sensors();
        for module in &server.modules {
            for sensor in &module.sensors {
                println!("{} ({}):{:.2} {}", module.modbus_slave_id, sensor.sensor_type, sensor.concentration().unwrap_or(0.0), sensor.si);
                // println!("\t\t{}", sensor.adc_value.unwrap_or(0));
            }
        }
        print!("{}[2J", 27 as char);
    }


    // loop {
    //     let server = server.clone();
    //     let thread2 = thread::spawn(move || {
    //         let server = server.read().expect("Failed to acquire read lock");
    //         for module in &server.modules {
    //             for sensor in &module.sensors {
    //                 println!("{} ({}): {}", module.modbus_slave_id, sensor.sensor_type, sensor.concentration().unwrap_or(0.0));
    //             }
    //         }
    //
    //         thread::sleep(Duration::from_millis(100));
    //     });
    // }

    // for modul in server.modules {
    //     for sensor in modul.sensors {
    //         println!("{} ({}): {}", modul.modbus_slave_id, sensor.sensor_type, sensor.concentration().unwrap());
    //     }
    // }
}
