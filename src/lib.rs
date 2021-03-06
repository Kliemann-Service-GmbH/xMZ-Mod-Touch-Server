#![doc(html_logo_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/xmz-logo.png",
       html_favicon_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/favicon.ico",
       html_root_url = "https://gaswarnanlagen.com/")]
//! `xMZ-Mod-Touch Server`
//!
//! Server Teil der `xMZ-Mod-Touch`-Platform
//!
//! Git Repository: [https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server.git](https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server.git)

// `error_chain!` can recurse deeply(3)
#![recursion_limit = "1024"]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
extern crate chrono;
extern crate env_logger;
extern crate iron;
extern crate libmodbus_rs;
extern crate rand;
extern crate router;
extern crate serde_json;
extern crate sysfs_gpio;

pub mod errors;
pub mod exception;
pub mod json_api;
pub mod server;
pub mod shift_register;

pub use self::exception::{Action, Check, Exception, ExceptionType};
pub use self::server::{Server, ServerType};
pub use self::server::zone::{Zone, ZoneStatus};
pub use self::server::zone::kombisensor::{Kombisensor, KombisensorType};
pub use self::server::zone::kombisensor::sensor::{Sensor, SensorType, SI};
pub use self::shift_register::{ShiftRegister, ShiftRegisterType};
