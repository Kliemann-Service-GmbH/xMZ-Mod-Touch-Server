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
    #[serde(default)]
    error_count: u64,
}

impl Default for Kombisensor {
    fn default() -> Self {
        Kombisensor {
            version: "0.0.0".to_string(),
            modbus_slave_id: 247,
            sensors: vec![Sensor::new_with_type(SensorType::NemotoNO2), Sensor::new_with_type(SensorType::NemotoCO)],
            error_count: 0,
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

    /// Liefert den Fehlerzähler zurück
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let kombisensor = Kombisensor::new();
    /// assert_eq!(kombisensor.get_error_count(), 0);
    /// ```
    pub fn get_error_count(&self) -> u64 {
        self.error_count
    }

    /// Erhöht den Fehlerzähler um Eins
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let mut kombisensor = Kombisensor::new();
    /// assert_eq!(kombisensor.get_error_count(), 0);
    /// kombisensor.inc_error_count();
    /// assert_eq!(kombisensor.get_error_count(), 1);
    /// ```
    pub fn inc_error_count(&mut self) {
        self.error_count += 1;
    }

    /// Setzt den Fehlerzähler auf Null zurück
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let mut kombisensor = Kombisensor::new();
    /// kombisensor.inc_error_count();
    /// assert_eq!(kombisensor.get_error_count(), 1);
    /// kombisensor.reset_error_count();
    /// assert_eq!(kombisensor.get_error_count(), 0);
    /// ```
    pub fn reset_error_count(&mut self) {
        self.error_count = 0;
    }

}
