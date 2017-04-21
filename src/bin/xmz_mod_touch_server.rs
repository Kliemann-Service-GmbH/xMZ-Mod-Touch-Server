extern crate env_logger;
extern crate xmz_mod_touch_server;
extern crate serde_json;

use xmz_mod_touch_server::xmz_mod_touch_server::XMZModTouchServer;
use xmz_mod_touch_server::error::XMZModTouchServerError;
use xmz_mod_touch_server::json_api;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;


fn start_update(xmz_mod_touch_server: Arc<Mutex<XMZModTouchServer>>)
                -> Result<(), XMZModTouchServerError> {
    thread::spawn(move || {
        loop {

            {
                // DIESER SCOPE IST SEHR WICHTIG! Ohne diesen würde der xmz_mod_touch_server.lock() niemals beendet!

                if let Ok(mut xmz_mod_touch_server) = xmz_mod_touch_server.lock() {
                    // Ausnahmen prüfen
                    xmz_mod_touch_server.check();
                    // XMZModTouchServer Kombonenten aktualisieren, Kombisensoren auslesen, ....
                    xmz_mod_touch_server.update();

                    // println!("{:#?}", &*xmz_mod_touch_server);
                }

            } // xmz_mod_touch_server.lock() frei gegeben

            thread::sleep(Duration::from_millis(100));
        }
    });

    Ok(())
}

fn start_web_interface(xmz_mod_touch_server: Arc<Mutex<XMZModTouchServer>>)
                       -> Result<(), XMZModTouchServerError> {
    json_api::init(xmz_mod_touch_server)?;

    Ok(())
}

fn run() -> Result<(), XMZModTouchServerError> {
    /// Create a XMZModTouchServer and write it to a file
    ///
    /// This if for bootstrapping purposes
    use std::fs::File;
    use std::io::prelude::*;
    let mut config = File::create("xmz_mod_touch_server_configuration.json")?;
    let xmz_mod_touch_server = XMZModTouchServer::new();
    let xmz_mod_touch_server_json = serde_json::to_string_pretty(&xmz_mod_touch_server)?;
    config.write_all(xmz_mod_touch_server_json.as_bytes())?;

    /// Server Konfiguration aus Konfig File auslesen
    let xmz_mod_touch_server: XMZModTouchServer = serde_json::from_str(include_str!("../../xmz_mod_touch_server_configuration.\
                                                                                     json"))?;
    let xmz_mod_touch_server = Arc::new(Mutex::new(xmz_mod_touch_server));
    /// Manuelle Erstellung des Servers
    /// let mut xmz_mod_touch_server = Arc::new(Mutex::new(XMZModTouchServer::new()));

    // Update thread
    start_update(xmz_mod_touch_server.clone())?;

    // IPC/ Web Interface
    start_web_interface(xmz_mod_touch_server.clone())?;

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
