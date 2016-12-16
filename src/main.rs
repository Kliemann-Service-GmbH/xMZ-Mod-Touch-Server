extern crate xmz_server;

use xmz_server::errors::*;

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
    println!("xMZ-Mod-Touch-Server is running...");
}

fn run() -> Result<()> {
    use std::fs::File;

    // This operation will fail
    File::open("xMZ-Mod-Touch-configuration.toml")
        .chain_err(|| "unable to open the configuration file.")?;

    Ok(())
}
