extern crate env_logger;
extern crate xmz_mod_touch_server;
extern crate serde_json;

use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use xmz_mod_touch_server::{XMZModTouchServer, json_api, configuration};
use xmz_mod_touch_server::errors::*;


pub const UPDATE_INTERVALL_MS: u64 = 100;


/// `start_basic_configuration` - Aufruf der Basis Konfiguration des XMZModTouchServer
///
fn start_basic_configuration(xmz_mod_touch_server: Arc<Mutex<XMZModTouchServer>>)
                -> Result<()>
{
    loop {
        if let Ok(mut xmz_mod_touch_server) = xmz_mod_touch_server.lock() {
            xmz_mod_touch_server.basic_configuration();
            break;
        }
    }

    Ok(())
}

/// `start_update`  - Starte die Update Thread des XMZModTouchServer
///
fn start_update(xmz_mod_touch_server: Arc<Mutex<XMZModTouchServer>>)
                -> Result<()>
{
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

            thread::sleep(Duration::from_millis(UPDATE_INTERVALL_MS));
        }
    });

    Ok(())
}

/// `start_web_interface` - Startet das JSON Web API
///
fn start_web_interface(xmz_mod_touch_server: Arc<Mutex<XMZModTouchServer>>)
                       -> Result<()> {
    json_api::init(xmz_mod_touch_server)?;

    Ok(())
}

// Starte die Aufgaben des Server Prozesses
fn run() -> Result<()> {
    /// Server Konfiguration aus Konfig File auslesen
    let xmz_mod_touch_server = Arc::new(Mutex::new(XMZModTouchServer::new_from_config()?));


    start_basic_configuration(xmz_mod_touch_server.clone())?;

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
