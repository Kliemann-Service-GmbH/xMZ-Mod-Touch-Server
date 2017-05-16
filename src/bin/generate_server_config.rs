extern crate env_logger;
extern crate serde_json;
extern crate xmz_mod_touch_server;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use xmz_mod_touch_server::errors::*;
use xmz_mod_touch_server::XMZModTouchServer;


const CONFIG_NAME: &'static str = "xMZ-Mod-Touch.json";

fn generate_config() -> Result<()> {
    /// Create a XMZModTouchServer and write it to a file
    ///
    /// This if for bootstrapping purposes
    let mut config = File::create(CONFIG_NAME)?;
    let xmz_mod_touch_server = XMZModTouchServer::new();
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
