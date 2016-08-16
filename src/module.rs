use sensor::{Sensor, SensorType};
use std::fmt;


/// Sensorplatine mit einem oder mehreren Messzellen
#[derive(Debug)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct Module {
    /// Typ des Sensor Moduls
    module_type: ModuleType,
    /// Vector der auf dieser Platine angeschlossenen Sensoren
    pub sensors: Vec<Sensor>,
    modbus_slave_id: i32,
}

/// Module Arten
///
/// Zur Zeit wird nur eine Art Modul unterstützt
#[derive(Debug, Eq, PartialEq)]
#[derive(RustcDecodable, RustcEncodable)]
pub enum ModuleType {
    /// * RAGAS_CO_NO2       - RA-GAS GmbH Kombisensor mit CO und NO Messzelle
    RAGAS_CO_NO2,
}


#[derive(Debug, Eq, PartialEq)]
pub enum ModuleError {
    InvalidModbusSlaveId,
}

impl fmt::Display for ModuleError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            ModuleError::InvalidModbusSlaveId => "Invalid Modbus Slave ID",
        })
    }
}


impl Module {
    /// Erzeugt ein neue Sensorplatine
    ///
    /// # Attribute
    /// * `module_type`         - Art des Moduls
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::module::{Module, ModuleType};
    ///
    /// let module1 = Module::new(ModuleType::RAGAS_CO_NO2);
    /// assert_eq!(module1.sensors.len(), 0);
    /// ```
    pub fn new(module_type: ModuleType) -> Self {
        match module_type {
            ModuleType::RAGAS_CO_NO2 => {
                Module {
                    module_type: ModuleType::RAGAS_CO_NO2,
                    sensors: vec![],
                    modbus_slave_id: 1,
                }
            }
        }
    }

    /// Setzt die Modbus Slave ID
    ///
    /// Gültige Modbus Adressen sind 0..256, wobei die Modbus Adresse 0 die Broadcast Adresse ist,
    /// diese ist für Module nicht erlaubt.
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::module::{Module, ModuleType};
    ///
    /// let mut module = Module::new(ModuleType::RAGAS_CO_NO2);
    /// assert_eq!(module.modbus_slave_id(), 1);
    /// module.set_modbus_slave_id(256);
    /// assert_eq!(module.modbus_slave_id(), 256);
    /// ```
    ///
    /// Erlaubte Modbus Slave ID's liegen zwischen 1 und 256 (inclusive 256).
    /// Unerlaubte Werte werden einfach nicht geschrieben.
    ///
    /// ```
    /// use xmz_server::module::{Module, ModuleType, ModuleError};
    ///
    /// let mut module = Module::new(ModuleType::RAGAS_CO_NO2);
    /// assert_eq!(module.modbus_slave_id(), 1);
    /// assert_eq!(module.set_modbus_slave_id(0), Err(ModuleError::InvalidModbusSlaveId));
    /// assert_eq!(module.modbus_slave_id(), 1);
    ///
    /// assert_eq!(module.set_modbus_slave_id(257), Err(ModuleError::InvalidModbusSlaveId));
    /// assert_eq!(module.modbus_slave_id(), 1);
    /// ```
    pub fn set_modbus_slave_id(&mut self, slave_id: i32) -> Result<i32, ModuleError> {
        match slave_id {
            slave_id @ 1...256 => {
                self.modbus_slave_id = slave_id;
                Ok(slave_id)
            }
            _ => Err(ModuleError::InvalidModbusSlaveId),
        }
    }

    /// Liefert die Modbus Slave ID
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::module::{Module, ModuleType};
    ///
    /// let module = Module::new(ModuleType::RAGAS_CO_NO2);
    /// assert_eq!(module.modbus_slave_id(), 1);
    /// ```
    pub fn modbus_slave_id(&self) -> i32 {
        self.modbus_slave_id
    }

    /// Liefert den Module Type als String
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::module::{Module, ModuleType};
    ///
    /// let module = Module::new(ModuleType::RAGAS_CO_NO2);
    /// assert_eq!(module.module_type(), "RAGAS_CO_NO2".to_string());
    /// ```
    pub fn module_type(&self) -> String {
        format!("{:?}", self.module_type)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use sensor::SensorType;

    #[test]
    fn defaults() {
        let module = Module::new(ModuleType::RAGAS_CO_NO2);
        assert_eq!(module.module_type, ModuleType::RAGAS_CO_NO2);
    }

    #[test]
    fn default_module_has_0_sensors() {
        let module = Module::new(ModuleType::RAGAS_CO_NO2);
        assert_eq!(module.sensors.len(), 0);
    }
}
