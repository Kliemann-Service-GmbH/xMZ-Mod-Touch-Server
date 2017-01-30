//! Steuert die Relais und LED'sensor
//!
//! Die Relais und LED sind über 8bit serielle Shift Register angeschlossen. Dieser Teil der
//! Software dient zur Verwaltung und Kontrolle dieser.
pub mod shift_register;

pub use self::shift_register::{ShiftRegister, ShiftRegisterType};
