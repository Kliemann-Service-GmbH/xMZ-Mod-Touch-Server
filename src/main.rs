extern crate xmz_server;

use xmz_server::*;


fn main() {
    println!("xMZ-Mod-Touch-Server Version: {}\n",
             env!("CARGO_PKG_VERSION"));

    if let Err(ref e) = run() {
        println!("error: {}", e);

        ::std::process::exit(1);
    }
}
