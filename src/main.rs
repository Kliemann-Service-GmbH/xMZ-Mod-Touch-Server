// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#![feature(proc_macro)]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate xmz_server;

use xmz_server::errors::*;
use xmz_server::system_commands;

#[derive(Serialize, Deserialize, Debug)]
struct Server {
    serial_interface: String,
    baud: i32,
}

impl Server {
    pub fn new() -> Server {
        Server {
            serial_interface: "/dev/ttyS1".to_string(),
            baud: 9600,
        }
    }
}

fn run() -> Result<()> {
    // /boot mounten
    system_commands::mount()?;

    // Konfiguration einlesen
    let server = Server::new();
    let serialized = serde_json::to_string(&server).unwrap();
    println!("serialized Server: {}", serialized);
    let deserialized: Server = serde_json::from_str(&serialized).unwrap();
    println!("deserialized Server: {:?}", deserialized);

    // /boot wieder umounten
    system_commands::umount()?;

    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // If th backtrace is not generated. Try to run with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}
