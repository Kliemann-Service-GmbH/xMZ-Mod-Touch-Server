extern crate serde_json;
extern crate xmz_server;

#[allow(unused_imports)]
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use xmz_server::*;


fn run() -> Result<()> {
    let config_file = try!(configuration::read_config_file());
    let mut server: Server = try!(serde_json::from_str(&config_file));

    try!(server.init());

    // Die Server Instanz wird nun in ein Arc<Mutex<T>> gepackt (shared (Arc) mutable (Mutex) state)
    let server = Arc::new(Mutex::new(server));

    loop {
        let server_update_sensors = server.clone();
        let server_request_handler = server.clone();

        // 1. Thread zum Update der Sensoren via modbus_stop_bit
        //
        // Dieser Thread muss mindestens einmal durchlauden werden pro loop Zyklus, desshalb
        // hat dieser Thread einen Namen `thread_update_sensors` und desshalb wird der Thread
        // am Ende gejoined `thread_update_sensors.join()`
        let thread_update_sensors = thread::spawn(move || {
            server_update_sensors.lock().map(|mut server| {
                server.update_sensors()
                    .map(|mut server| println!("thread_update_sensors: {:#?}", server))
                    .map_err(|err| println!("error: {}", err));
            });
            thread::sleep_ms(100);
        }).join();
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
