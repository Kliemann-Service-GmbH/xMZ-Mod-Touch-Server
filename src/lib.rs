#![doc(html_logo_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/xmz-logo.png",
       html_favicon_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/favicon.ico",
       html_root_url = "https://gaswarnanlagen.com/")]
//! xMZ-Mod-Touch Server
//!
//! Server Teil der 'xMZ-Mod-Touch'-Platform
//!
//! Git Repository: https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server

#![recursion_limit = "1024"]
#![feature(proc_macro)]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate sysfs_gpio;
extern crate rand;

// FIXME: Öffentliche API überdenken, nicht nötige `pub` entfernen
pub mod alarmgruppe;
pub mod co_no2_kombisensor;
pub mod configuration;
pub mod errors;
pub mod server;
pub mod shift_register;
pub mod system_command;
pub mod zone;

// FIXME: Refactor alle unnötigen glob reexports, zu expliziten ones.
pub use self::alarmgruppe::*;
pub use self::co_no2_kombisensor::*;
pub use self::configuration::Configuration;
pub use self::errors::*;
pub use self::server::*;
pub use self::shift_register::*;
pub use self::zone::*;

#[allow(unused_imports)]
use errors::*;


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
pub fn read_config_file() -> Result<String> {
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
        // Hier kann nicht einfach ein try!(system_command::read_in(..)) angewannt werden,
        // da
        config_file = match system_command::read_in("/boot/xMZ-Mod-Touch.json") {
            Ok(config_file) => config_file,
            Err(_) => {
                try!(umount_boot());
                String::new()
            },
        };
        try!(umount_boot());
    }

    Ok(config_file)
}
