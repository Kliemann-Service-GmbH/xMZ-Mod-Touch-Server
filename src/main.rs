extern crate xmz_server;
extern crate serde_json;

use xmz_server::errors::*;
use xmz_server::system_commands;
use xmz_server::server::Server;

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
    println!("xMZ-Mod-Touch-Server Version: {}\n", env!("CARGO_PKG_VERSION"));

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
