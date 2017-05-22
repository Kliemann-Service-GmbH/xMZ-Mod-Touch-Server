/// Dies soll der Command Line Config Generator werden
///
/// TODO: Add clap trait
/// TODO: Add parameters config_file name and path
/// TODO: Add parameter num_zones
/// TODO: Add parameter num_kombisensors for zone_num
/// TODO: Add parameter num_sensors for kombisensor_num
/// TODO: Add parameter max_server_uptime
extern crate env_logger;
extern crate serde_json;
extern crate xmz_mod_touch_server;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use xmz_mod_touch_server::errors::*;
use xmz_mod_touch_server::{XMZModTouchServer, Kombisensor, KombisensorType};


const CONFIG_NAME: &'static str = "xMZ-Mod-Touch.json";

fn generate_config() -> Result<()> {
    /// Create a XMZModTouchServer and write it to a file
    ///
    /// This if for bootstrapping purposes

    // Config file
    let mut config = File::create(CONFIG_NAME)?;

    // The Server
    let mut xmz_mod_touch_server = XMZModTouchServer::new();
    xmz_mod_touch_server.add_zone();

    let mut kombisensor = Kombisensor::new_with_type(KombisensorType::RAGasSimulation);
    kombisensor.set_modbus_address(247);
    xmz_mod_touch_server.get_zone_mut(0).unwrap().add_kombisensor( kombisensor );
    // for i in 1..8 {
    //     let mut kombisensor = Kombisensor::new_with_type(KombisensorType::RAGas);
    //     kombisensor.set_modbus_address(i);
    //     xmz_mod_touch_server.get_zone_mut(0).unwrap().add_kombisensor( kombisensor );
    // }

    // write to config file
    let xmz_mod_touch_server_json = serde_json::to_string_pretty(&xmz_mod_touch_server)?;
    config.write_all(xmz_mod_touch_server_json.as_bytes())?;

    Ok(())
}

fn run() -> Result<()> {
    generate_config()?;

    Ok(())
}


fn main() {
    // Initalisiere Logger (erst nach diesem Aufruf sind `trace!()`, `debug!()` usw. functional)
    env_logger::init().unwrap();

    println!("Generiere Server Konfiguration '{}'", CONFIG_NAME);

    if let Err(ref e) = run() {
        println!("error: {}", e);

        if let Some(cause) = e.cause() {
            println!("caused by: {}", cause);
        }

        ::std::process::exit(1);
    }
}
