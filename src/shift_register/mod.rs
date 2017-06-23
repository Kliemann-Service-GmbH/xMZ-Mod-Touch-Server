//! Shiftregister Ansteuerung (LEDs und Relais)
//!
//! Die Relais und LEDs der XMZModTouchServer Platform sind mit 8bit serial-in paralel-out Shiftregistern
//! angeschlossen.
//!
mod shift_register;

pub use self::shift_register::{ShiftRegister, ShiftRegisterType};
