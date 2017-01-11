//! Die Kombisensor Datenstruktur representiert eine Platine eines [CO-NO2-Kombisensor-Mod](https://github.com/Kliemann-Service-GmbH/CO-NO2-Kombisensor-Mod) der Firma RA-GAS
//!
use co_no2_kombisensor::sensor::{Sensor, SensorType};
// use errors::*;

/// Platine des CO-NO2-Kombisensor-Mod
#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Kombisensor {
    #[serde(default)]
    version: String,
    modbus_slave_id: u8,
    #[serde(default)]
    sensors: Vec<Sensor>,
}

impl Default for Kombisensor {
    fn default() -> Self {
        Kombisensor {
            version: "0.0.0".to_string(),
            modbus_slave_id: 247,
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

    /// Liefert die Modbus Adresse des Kombisensors
    ///
    /// Jede Platine, eine Kombisensor mit mehreren Sensormeßzellen, hat eine Modbus Slave ID
    /// die so genannte Modbus Adresse.
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let kombisensor = Kombisensor::new();
    /// assert_eq!(kombisensor.get_modbus_slave_id(), 247);
    /// ```
    pub fn get_modbus_slave_id(&self) -> u8 {
        self.modbus_slave_id
    }

    /// Liefert die Sensoren als Referenze zu einem Vector
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    /// ```
    pub fn get_sensors(&self) -> &Vec<Sensor> {
        self.sensors.as_ref()
    }

    /// Liefert die Sensoren in einem mutablen Vector Slice
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    /// ```
    pub fn get_sensors_mut(&mut self) -> &mut Vec<Sensor> {
        self.sensors.as_mut()
    }

}
