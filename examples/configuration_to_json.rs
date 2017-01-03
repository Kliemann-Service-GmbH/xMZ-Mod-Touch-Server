extern crate xmz_server;

use std::fs::File;
use std::io::prelude::*;
use xmz_server::*;

fn run() -> Result<()> {
    let configuration = Configuration {
        server: Server::new(),
        sensors: vec![
            Sensor::new(SensorType::NemotoNO2),
            Sensor::new(SensorType::NemotoCO),
            Sensor::new(SensorType::NemotoNO2),
            Sensor::new(SensorType::NemotoCO),
        ],
    };

    let configuration_json = try!(configuration.to_json());
    let mut f = try!(File::create("Configuration.json"));
    try!(f.write(configuration_json.as_bytes()));

    Ok(())
}

fn main() {
    match run() {
        Ok(()) => {},
        Err(err) => println!("Error: {}", err),
    }
}
