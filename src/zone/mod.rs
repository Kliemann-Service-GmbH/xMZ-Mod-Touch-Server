//! Verschiedene Zonen der Anlage
//!
//! Zu den möglichen Zonen gehören die unterschiedlichen Alarmgruppen, denen Sensor Messzellen
//! zugeordnet werden können. Aber auch die Zone "Störung".
pub mod zone;

pub use self::zone::{Zone, ZoneType};
