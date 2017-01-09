extern crate xmz_server;

#[allow(unused_imports)]
use std::sync::{Arc, Mutex};
use xmz_server::*;



fn run() -> Result<()> {
    let config_file = try!(read_config_file());
    let _configuration = try!(Configuration::from_config(config_file));


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
