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

pub mod configuration;
mod error;
mod server;
mod system_command;
mod co_no2_kombisensor;

pub use self::configuration::Configuration;
pub use self::error::*;
pub use self::server::*;
pub use self::co_no2_kombisensor::*;

pub fn mount_boot() -> Result<()> {
    match system_command::call("mount /dev/mmcblk0p1 /boot") {
        Ok(_) => Ok(()),
        Err(_) => Err(XMZError::NotAllowed),
    }
}

pub fn umount_boot() -> Result<()> {
    system_command::call("umount /boot")?;

    Ok(())
}

pub fn run() -> Result<()> {
    let mut config = String::new();

    #[cfg(feature = "development")]
    {
        println!("Development System");
        config = try!(system_command::read_in("xMZ-Mod-Touch.json"));
    }

    #[cfg(not(feature = "development"))]
    {
        println!("Produktiv System");
        try!(mount_boot());
        config = try!(system_command::read_in("/boot/xMZ-Mod-Touch.json"));
        try!(umount_boot());
    }
    let configuration = Configuration::from_config(config);
    println!("{:?}", configuration);

    Ok(())
}
