#[macro_use] extern crate log;
extern crate env_logger;
extern crate xmz_mod_touch_server;
extern crate serde_json;

use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use xmz_mod_touch_server::server::Server;
use xmz_mod_touch_server::json_api;
use xmz_mod_touch_server::errors::*;

pub const UPDATE_INTERVALL_MS: u64 = 100;


/// `start_basic_configuration` - Aufruf der Basis Konfiguration des erver
///
fn start_basic_configuration(server: Arc<Mutex<Server>>) -> Result<()> {
    loop {
        if let Ok(mut server) = server.lock() {
            info!("start_basic_configuration() erfolgreich");
            server.basic_configuration()?;
            break;
        }
    }

    Ok(())
}

/// `start_update`  - Starte die Update Thread des erver
///
fn start_update(server: Arc<Mutex<Server>>) -> Result<()> {
    thread::spawn(move || {
        loop {
            {
                // DIESER SCOPE IST SEHR WICHTIG! Ohne diesen würde der server.lock() niemals beendet!
                if let Ok(mut server) = server.lock() {
                    // Ausnahmen prüfen
                    server.check();
                    // erver Kombonenten aktualisieren, Kombisensoren auslesen, ....
                    server.update();
                    // println!("{:#?}", &*server);
                }

            } // server.lock() frei gegeben

            thread::sleep(Duration::from_millis(UPDATE_INTERVALL_MS));
        }
    });

    Ok(())
}

/// `start_web_interface` - Startet das JSON Web API
///
fn start_web_interface(server: Arc<Mutex<Server>>) -> Result<()> {
    json_api::init(server)?;

    Ok(())
}

// Starte die Aufgaben des Server Prozesses
fn run() -> Result<()> {
    /// Server Konfiguration aus Konfig File auslesen
    let server = Arc::new(Mutex::new(Server::new_from_config()?));

    start_basic_configuration(server.clone())?;

    // Update thread
    start_update(server.clone())?;

    // IPC/ Web Interface
    start_web_interface(server.clone())?;

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
