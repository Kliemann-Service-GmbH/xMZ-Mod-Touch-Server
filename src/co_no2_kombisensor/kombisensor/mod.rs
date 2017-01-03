//! Die Kombisensor Datenstruktur representiert eine Platine eines [CO-NO2-Kombisensor-Mod](https://github.com/Kliemann-Service-GmbH/CO-NO2-Kombisensor-Mod) der Firma RA-GAS
//!
use co_no2_kombisensor::sensor::{Sensor, SensorType};

/// Platine des CO-NO2-Kombisensor-Mod
#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Kombisensor {
    version: String,
    modbus_address: u8,
    sensors: Vec<Sensor>,
}

impl Kombisensor {
    pub fn new() -> Self {
        Kombisensor {
            version: "0.0.0".to_string(),
            modbus_address: 247,
            sensors: vec![Sensor::new(SensorType::NemotoNO2), Sensor::new(SensorType::NemotoCO)],
        }
    }
}
