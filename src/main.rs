
// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate iron;
extern crate router;
extern crate serde_json;
extern crate xmz_server;

#[allow(unused_imports)]
use iron::prelude::*;
use iron::status;
use router::Router;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use xmz_server::*;
use xmz_server::errors::*;
use xmz_server::configuration;


fn server_update(server: Arc<Mutex<Server>>) -> Result<()> {
    thread::spawn(move || {
        loop {
            {
                let mut server = server.lock().unwrap();

                // server.simulation();
                server.update_sensors();

            } // Unlock weil server out of scope geht

            thread::sleep(Duration::from_millis(1000));
        }
    });

    Ok(())
}

fn server_web_interface(server: Arc<Mutex<Server>>) -> Result<()> {
    let mut router = Router::new();

    router.get("/",                 move |r: &mut Request| index(r,    &server.lock().unwrap()), "index");

    fn index(request: &mut Request, server: &Server) -> IronResult<Response> {
        debug!("{:#?}", request);
        let payload = serde_json::to_string(&server).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    // debug!("Server response on: http://localhost:3000");
    Iron::new(router).http("0.0.0.0:3000").unwrap();

    Ok(())
}

fn oneshot_server_init(server: Arc<Mutex<Server>>) -> Result<()> {
    let mut server = server.lock().unwrap();
    server.init()?;

    Ok(())
}


fn run() -> Result<()> {
    // Server Instanz aus der Konfigurationsdatei builden
    let config_file = try!(configuration::read_config_file());
    let mut server: Server = try!(serde_json::from_str(&config_file));

    // Thread save, Referenz counted Server Instanz erzeugen
    let server = Arc::new(Mutex::new(server));

    // Einmalig die Server default Konfiguration laden
    // FIXME: Evtl mit der Konfigurationsdatei obsolet machen
    oneshot_server_init(server.clone())?;

    /// Update thread
    server_update(server.clone())?;

    /// IPC/ Web stuff
    server_web_interface(server.clone())?;

    Ok(())
}

fn main() {
    // Initialisiere den Logger (erst nach diesem sind `trace!()`, `debug!()` usw funktional)
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
