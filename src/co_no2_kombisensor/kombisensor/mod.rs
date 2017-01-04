//! Die Kombisensor Datenstruktur representiert eine Platine eines [CO-NO2-Kombisensor-Mod](https://github.com/Kliemann-Service-GmbH/CO-NO2-Kombisensor-Mod) der Firma RA-GAS
//!
use co_no2_kombisensor::sensor::{Sensor, SensorType};

/// Platine des CO-NO2-Kombisensor-Mod
#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Kombisensor {
    #[serde(default)]
    version: String,
    modbus_address: u8,
    #[serde(default)]
    sensors: Vec<Sensor>,
}

impl Default for Kombisensor {
    fn default() -> Self {
        Kombisensor {
            version: "0.0.0".to_string(),
            modbus_address: 247,
            sensors: vec![Sensor::new_with_type(SensorType::NemotoNO2), Sensor::new_with_type(SensorType::NemotoCO)],
        }
    }
}

impl Kombisensor {
    /// Erzeugt eine neue Instanz
    ///
    pub fn new() -> Self {
        Kombisensor { ..Default::default() }
    }

    /// Liefert die Sensoren mutable
    pub fn get_sensors_mut(&mut self) -> Vec<Sensor> {
        self.sensors.clone()
    }
}
