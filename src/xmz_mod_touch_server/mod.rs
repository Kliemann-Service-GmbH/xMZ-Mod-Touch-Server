//! Serverteil der xMZ-Mod-Touch-Server Platform
//!
//! Hier werden alle Komponenten des Servers verwaltet.
//!
extern crate chrono;

mod xmz_mod_touch_server;
pub mod configuration;
mod zone;

pub use self::xmz_mod_touch_server::XMZModTouchServer;
pub use self::zone::{Zone, ZoneStatus};
