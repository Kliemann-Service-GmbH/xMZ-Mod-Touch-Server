//! Serverteil der "xMZ-Mod-Touch"-Plattform
//!
//! Der Serverteil startet extrem früh, und verwaltet die angeschlossen Sensor Module,
//! Alarmauswertung und stellt die Schnittstellen für die GUI bereit.
pub mod server;

pub use self::server::Server;
