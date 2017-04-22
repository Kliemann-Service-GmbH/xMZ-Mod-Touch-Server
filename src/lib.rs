#![doc(html_logo_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/xmz-logo.png",
       html_favicon_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/favicon.ico",
       html_root_url = "https://gaswarnanlagen.com/")]
//! `xMZ-Mod-Touch Server`
//!
//! Server Teil der `xMZ-Mod-Touch`-Platform
//!
//! Git Repository: [https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server.git](https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server.git)

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
extern crate chrono;
extern crate env_logger;
extern crate iron;
extern crate rand;
extern crate router;
extern crate serde_json;
extern crate sysfs_gpio;

pub mod error;
pub mod exception;
pub mod json_api;
pub mod kombisensor;
pub mod sensor;
pub mod shift_register;
pub mod xmz_mod_touch_server;
pub mod zone;

pub use self::xmz_mod_touch_server::XMZModTouchServer;
pub use self::xmz_mod_touch_server::configuration;
pub use self::shift_register::{ShiftRegister, ShiftRegisterType};