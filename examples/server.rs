extern crate ansi_term;
extern crate xmz_server;

use ansi_term::Style;
use ansi_term::Colour::{Green, Red};
use xmz_server::module::{Module, ModuleType};
use xmz_server::server::{Server};

fn main() {
    let mut server = Server::new();
    server.init();

    server.modbus_device = "/dev/ttyUSB0";

    server.modules.push(Module::new(ModuleType::RAGAS_CO_NO2));
    server.modules[0].modbus_address = 46;
    server.modules.push(Module::new(ModuleType::RAGAS_CO_NO2));
    server.modules[1].modbus_address = 23;
    server.modules.push(Module::new(ModuleType::RAGAS_CO_NO2));
    server.modules[2].modbus_address = 24;
    server.modules.push(Module::new(ModuleType::RAGAS_CO_NO2));
    server.modules[3].modbus_address = 25;
    server.modules.push(Module::new(ModuleType::RAGAS_CO_NO2));
    server.modules[4].modbus_address = 26;
    server.modules.push(Module::new(ModuleType::RAGAS_CO_NO2));
    server.modules[5].modbus_address = 27;
    server.modules.push(Module::new(ModuleType::RAGAS_CO_NO2));
    server.modules[6].modbus_address = 28;

    server.update_sensors();

    for modul in server.modules {
        let mut index = 1;
        println!("_____________________________________________________");
        println!("Modul {} ", Style::new().bold().paint(modul.modbus_address.to_string()));

        for sensor in modul.sensors {
            let c = sensor.concentration().unwrap();
            let concentration = format!("{:.2}", c);
            let concentration: ansi_term::ANSIString = Style::new().bold().paint(concentration);

            let sensor_type = match sensor.sensor_type {
                xmz_server::sensor::SensorType::NemotoNO2 => "RA-GAS NO2".to_string(),
                xmz_server::sensor::SensorType::NemotoCO =>  "RA-GAS  CO".to_string(),
            };
            let sensor_type: ansi_term::ANSIString = Style::new().bold().paint(sensor_type);

            println!("\t|Sensor {} {} ------------------------", index, sensor_type);
            println!("\t----------|Konzentration: {} {} | {} ADC", concentration, sensor.si, Style::new().bold().paint(sensor.adc_value.unwrap().to_string()));
            index += 1;
        }
    }
}
