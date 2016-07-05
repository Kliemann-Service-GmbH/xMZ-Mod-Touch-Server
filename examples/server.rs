extern crate ansi_term;
extern crate xmz_server;

use ansi_term::Style;
use ansi_term::Colour::{Green, Red};
use xmz_server::module::{Module, ModuleType};
use xmz_server::server::{Server};
use std::rc::Rc;
use std::cell::RefCell;
use std::thread;


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
    server.modules.push(Module::new(ModuleType::RAGAS_CO_NO2));
    server.modules[0].modbus_slave_id = 23;
    server.modules[1].modbus_slave_id = 24;
    server.modules[2].modbus_slave_id = 25;
    server.modules[3].modbus_slave_id = 26;
    server.modules[4].modbus_slave_id = 27;
    server.modules[5].modbus_slave_id = 28;
    server.modules[6].modbus_slave_id = 29;

    server.update_sensors();

    for modul in server.modules {
        for sensor in modul.sensors {
            println!("{}", sensor.concentration().unwrap());
        }
    }
}
