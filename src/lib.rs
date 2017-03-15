#![doc(html_logo_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/xmz-logo.png",
       html_favicon_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/favicon.ico",
       html_root_url = "https://gaswarnanlagen.com/")]
//! xMZ-Mod-Touch Server
//!
//! Server Teil der 'xMZ-Mod-Touch'-Platform
//!
//! Git Repository: https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server.git

#![recursion_limit = "1024"]
#![feature(proc_macro)]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;
#[macro_use] extern crate nom;
#[macro_use] extern crate serde_derive;
extern crate env_logger;
extern crate iron;
extern crate libmodbus_rs;
extern crate rand;
extern crate serde_json;
extern crate serde;
extern crate sysfs_gpio;

// FIXME: Öffentliche API überdenken, nicht nötige `pub` entfernen
pub mod co_no2_kombisensor;
pub mod configuration;
pub mod errors;
pub mod server;
pub mod shift_register;
pub mod system_command;
pub mod zone;

// FIXME: Refactor alle unnötigen glob reexports, zu expliziten ones.
pub use self::co_no2_kombisensor::*;
pub use self::configuration::configuration::*;
pub use self::errors::*;
pub use self::server::server::{Server, ServerMode};
pub use self::shift_register::*;
pub use self::system_command::*;
pub use self::zone::*;
