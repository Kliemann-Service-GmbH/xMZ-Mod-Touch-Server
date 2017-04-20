extern crate env_logger;
extern crate xmz_server;
extern crate serde_json;

use xmz_server::xmz_server::XMZServer;
use xmz_server::error::XMZServerError;
use xmz_server::json_api;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;


fn start_update(xmz_server: Arc<Mutex<XMZServer>>) -> Result<(), XMZServerError> {
    thread::spawn(move || {
        loop {

            { // DIESER SCOPE IST SEHR WICHTIG! Ohne diesen würde der xmz_server.lock() niemals beendet!
            
                if let Ok(mut xmz_server) = xmz_server.lock() {
                    // Ausnahmen prüfen
                    xmz_server.check();
                    // XMZServer Kombonenten aktualisieren, Kombisensoren auslesen, ....
                    xmz_server.update();

                    // println!("{:#?}", &*xmz_server);
                }

            } // xmz_server.lock() frei gegeben

            thread::sleep(Duration::from_millis(100));
        }
    });

    Ok(())
}

fn start_web_interface(xmz_server: Arc<Mutex<XMZServer>>) -> Result<(), XMZServerError> {
    json_api::init(xmz_server)?;

    Ok(())
}

fn run() -> Result<(), XMZServerError> {   
    /// Create a XMZServer and write it to a file
    ///
    /// This if for bootstrapping purposes
    use std::fs::File;
    use std::io::prelude::*;
    let mut config = File::create("xmz_server_configuration.json")?;
    let xmz_server = XMZServer::new();
    let xmz_server_json = serde_json::to_string_pretty(&xmz_server)?;
    config.write_all(xmz_server_json.as_bytes())?;

    /// Server Konfiguration aus Konfig File auslesen
    let xmz_server: XMZServer = serde_json::from_str(include_str!("../../xmz_server_configuration.json"))?;
    let xmz_server = Arc::new(Mutex::new(xmz_server));
    /// Manuelle Erstellung des Servers
    // let mut xmz_server = Arc::new(Mutex::new(XMZServer::new()));

    // Update thread
    start_update(xmz_server.clone())?;

    // IPC/ Web Interface
    start_web_interface(xmz_server.clone())?;

    Ok(())
}

fn main() {
    // Initalisiere Logger (erst nach diesem Aufruf sind `trace!()`, `debug!()` usw. functional)
    env_logger::init().unwrap();

    println!("Starte '{}' Version: {}\n",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"));

    if let Err(ref e) = run() {
        println!("error: {}", e);

        if let Some(cause) = e.cause() {
            println!("caused by: {}", cause);
        }

        ::std::process::exit(1);
    }
}
