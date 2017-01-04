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

/// Mounted die erste Partition der SDCard nach /boot
#[allow(dead_code)]
fn mount_boot() -> Result<()> {
    system_command::call("mount /dev/mmcblk0p1 /boot")?;

    Ok(())
}

/// Unmount /boot
#[allow(dead_code)]
fn umount_boot() -> Result<()> {
    system_command::call("umount /boot")?;

    Ok(())
}


/// Diese Funktion liest die Konfigurationsdatei ein, je nach Umgebung
///
/// Entweder wird das programm im `development` Modus aufgerufen, hier wird die Konfigurationsdatei
/// lokal gesucht und gelesen.
/// Oder aber das Programm wird im `produktiv` Modus (not(feature = "development")) ausgeführt,
/// in diesem wird zunächste /boot gemounted, anschließend die Konfigurationsdatei eingelesen
/// und zum Schluss /boot umounted.
#[allow(unused_assignments)]
fn read_config_file() -> Result<String> {
    let mut config_file = String::new();

    #[cfg(feature = "development")]
    {
        println!("Development System");
        config_file = try!(system_command::read_in("xMZ-Mod-Touch.json"));
    }
    #[cfg(not(feature = "development"))]
    {
        println!("Produktiv System");
        try!(mount_boot());
        config_file = try!(system_command::read_in("/boot/xMZ-Mod-Touch.json"));
        try!(umount_boot());
    }

    Ok(config_file)
}

/// Einsprungpunkt in die Lib
pub fn run() -> Result<()> {
    let config_file = try!(read_config_file());

    let configuration = try!(Configuration::from_config(config_file));
    println!("{:?}", configuration);

    for kombisensor in configuration.get_kombisensors() {
        println!("{:?}", kombisensor);
    }

    Ok(())
}
