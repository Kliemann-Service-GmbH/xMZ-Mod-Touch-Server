/// Dieses Beispiel dient auch dazu eine valide Konfiguration aus den einzelenen Modulen der
/// Software zu erstellen.

extern crate xmz_server;

use std::fs::File;
use std::io::prelude::*;
use xmz_server::*;

fn run() -> Result<()> {
    let configuration = Configuration {
        server: Server { serial_interface: "/dev/ttyUSB0".to_string(), baud: 9600 },
        kombisensors: vec![
            Kombisensor::new(),
            Kombisensor::new(),
            Kombisensor::new(),
            Kombisensor::new(),
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
