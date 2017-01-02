#![doc(html_logo_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/xmz-logo.png",
       html_favicon_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/favicon.ico",
       html_root_url = "https://gaswarnanlagen.com/")]
//! xMZ-Mod-Touch Server
//!
//! Server Teil der 'xMZ-Mod-Touch'-Platform
//!
//! Git Repository: https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server

#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod configuration;
mod error;
mod server;
mod system_commands;
mod sensor;

pub use self::configuration::*;
pub use self::error::*;
pub use self::server::*;
pub use self::sensor::*;


pub fn mount_boot() -> Result<()> {
    match system_commands::call("mount /dev/mmcblk0p1 /boot") {
        Ok(_) => Ok(()),
        Err(_) => Err(XMZError::NotAllowed),
    }
}

pub fn umount_boot() -> Result<()> {
    system_commands::call("umount /boot")?;

    Ok(())
}

pub fn run() -> Result<()> {
    #[cfg(feature = "development")]
    {
        println!("Development System");
        let config = try!(system_commands::readin("xMZ-Mod-Touch.json"));
        let configuration = Configuration::from_config(config);
        println!("{:?}", configuration)
    }

    #[cfg(not(feature = "development"))]
    {
        println!("Produktiv System");
        try!(mount_boot());

        let config = try!(system_commands::readin("/boot/xMZ-Mod-Touch.json"));
        let configuration = Configuration::from_config(config);
        println!("{:?}", configuration);

        try!(umount_boot());
    }

    Ok(())
}
