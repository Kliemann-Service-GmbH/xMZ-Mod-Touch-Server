#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod error;
mod server;
pub mod system_commands;
mod configuration;

pub use self::configuration::*;
pub use self::server::*;
pub use self::error::*;


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

pub fn run() -> Result<()> {
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
