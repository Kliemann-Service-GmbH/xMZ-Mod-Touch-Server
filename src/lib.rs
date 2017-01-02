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
        let configuration = Configuration {
            server: Server {
                serial_interface: "/dev/ttyS1".to_string(),
                baud: 9600,
            },
            sensors: vec![Sensor::new(), Sensor::new(), Sensor::new(), Sensor::new(), ]
        };
        println!("{}", configuration.as_str());
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
