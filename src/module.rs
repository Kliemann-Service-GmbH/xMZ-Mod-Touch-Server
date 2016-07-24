use sensor::{Sensor, SensorType};
use std::fmt;


/// Sensorplatine mit einem oder mehreren Messzellen
#[derive(Debug)]
#[derive(RustcEncodable)]
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
#[derive(RustcEncodable)]
pub enum ModuleType {
    /// * RAGAS_CO_NO2       - RA-GAS GmbH Kombisensor mit CO und NO Messzelle
    RAGAS_CO_NO2,
}



#[derive(Debug)]
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
    /// assert_eq!(module1.sensors.len(), 2);
    /// ```
    pub fn new(module_type: ModuleType) -> Self {
        match module_type {
            ModuleType::RAGAS_CO_NO2 => {
                Module {
                    module_type: ModuleType::RAGAS_CO_NO2,
                    sensors: vec![
                        Sensor::new(SensorType::NemotoNO2),
                        Sensor::new(SensorType::NemotoCO),
                    ],
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
    /// ```
    pub fn set_modbus_slave_id(&mut self, slave_id: i32) -> Result<i32, ModuleError> {
        match slave_id {
            slave_id @ 1 ... 256 => {
                self.modbus_slave_id = slave_id;
                Ok(slave_id)
            },
            _ => { Err(ModuleError::InvalidModbusSlaveId) }
        }
    }

    /// Liefert die Modbus Slave ID
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn get_modbus_slave_id(&self) -> i32 {
        self.modbus_slave_id
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use sensor::{SensorType};

    #[test]
    fn defaults() {
        let module = Module::new(ModuleType::RAGAS_CO_NO2);
        assert_eq!(module.module_type, ModuleType::RAGAS_CO_NO2);
    }

    #[test]
    fn default_module_has_2_sensors() {
        let module = Module::new(ModuleType::RAGAS_CO_NO2);
        assert_eq!(module.sensors.len(), 2);
    }

    #[test]
    fn default_module_sensor_config() {
        let module = Module::new(ModuleType::RAGAS_CO_NO2);
        assert_eq!(module.sensors[0].sensor_type, SensorType::NemotoNO2);
        assert_eq!(module.sensors[1].sensor_type, SensorType::NemotoCO);
    }
}
