#[macro_use] extern crate log;
extern crate env_logger;
extern crate serde_json;
extern crate xmz_server;

#[allow(unused_imports)]
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use xmz_server::*;


fn helper_print(server: &Server) {
    for kombisensor in server.get_kombisensors().iter() {
        for sensor in kombisensor.get_sensors().iter() {
            println!("{} {}\tADC: {} {:04.02} {}\t(Fehler: {})", kombisensor.get_modbus_slave_id(),
                                sensor.get_sensor_type(),
                                sensor.get_adc_value(),
                                sensor.get_concentration(),
                                sensor.get_si(),
                                kombisensor.get_error_count(),
            );
        }
    }
}

fn run() -> Result<()> {
    let config_file = try!(configuration::read_config_file());
    let mut server: Server = try!(serde_json::from_str(&config_file));

    // println!("{:#?}", server);
    try!(server.init());

    server.update_sensors()?;
    // println!("{:#?}", server);

    helper_print(&server);

    Ok(())
}

fn main() {
    env_logger::init().unwrap();

    println!("xMZ-Mod-Touch-Server Version: {}\n",
             env!("CARGO_PKG_VERSION"));

    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}
