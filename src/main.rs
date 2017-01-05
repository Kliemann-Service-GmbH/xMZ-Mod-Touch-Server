extern crate xmz_server;

use std::sync::{Arc, Mutex};
use xmz_server::*;



fn run() -> Result<()> {
    let config_file = try!(read_config_file());

    let configuration = try!(Configuration::from_config(config_file));

    let kombisensors: Arc<Mutex<Vec<Kombisensor>>> = Arc::new(Mutex::new(configuration.get_kombisensors()));

    let kombisensors = kombisensors.clone();
    {
        let mut kombisensors = kombisensors.lock().unwrap();
        for kombisensor in kombisensors.iter_mut() {
            let sensor1 = kombisensor.get_sensor_mut(0)?;
            println!("1. Messzelle; adc_value: {:?}", sensor1.get_adc_value());
        }
    }

    Ok(())
}

fn main() {
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
