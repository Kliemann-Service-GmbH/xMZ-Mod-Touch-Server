extern crate xmz_server;
extern crate serde_json;

use xmz_server::*;
use xmz_server::system_commands;


fn run() -> Result<()> {
    let server = Server::new();

    // #[cfg(feature = "development")]
    // #[cfg(not(feature = "development"))]

    Ok(())
}

fn main() {
    println!("xMZ-Mod-Touch-Server Version: {}\n",
             env!("CARGO_PKG_VERSION"));

    if let Err(ref e) = run() {
        println!("error: {}", e);

        ::std::process::exit(1);
    }
}
