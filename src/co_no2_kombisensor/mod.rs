//! CO/ NO2 Kombisensor mit Modbus Interface der Firma RA-Gas GmbH
//!
//! Die Senosr Platine verfügt über einen Modbus Transceiver und 2 Messzellen, je eine, für
//! Kohlenmonoxid (CO) und Stickstoffdioxid (NO2).
pub mod sensor;
pub mod kombisensor;

pub use self::sensor::{Sensor, SensorType, SI};
pub use self::kombisensor::{Kombisensor, to_bytes};
