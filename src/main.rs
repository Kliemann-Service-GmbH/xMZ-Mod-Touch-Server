extern crate xmz_server;
extern crate serde_json;

use xmz_server::*;
use xmz_server::system_commands;


pub fn mount_boot() -> Result<()> {
    match system_commands::call("mount /dev/mmcblk0p1 /boot") {
        Ok(_) => Ok(()),
        Err(err) => Err(XMZError::NotAllowed),
    }
}

pub fn umount_boot() -> Result<()> {
    system_commands::call("umount /boot")?;

    Ok(())
}

fn run() -> Result<()> {
    let server = Server::new();

    #[cfg(feature = "development")]
    {
        println!("Development System");
    }

    #[cfg(not(feature = "development"))]
    {
        println!("Produktiv System");
        try!(mount_boot().map(|_| {
            println!("Lese nun Configuration");
        }));
    }

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
