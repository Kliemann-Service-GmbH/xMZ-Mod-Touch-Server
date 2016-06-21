use sensor::{Sensor, SensorType};

/// Module Arten
///
/// Zur Zeit wird nur eine Art Modul unterstützt
#[derive(Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ModuleType {
    /// * RAGAS_CO_NO2       - RA-GAS GmbH Kombisensor mit CO und NO Messzelle
    RAGAS_CO_NO2,
}


/// Sensorplatine mit einem oder mehreren Messzellen
pub struct Module<'a> {
    /// Typ des Sensor Moduls
    module_type: ModuleType,
    /// Vector der auf dieser Platine angeschlossenen Sensoren
    pub sensors: Vec<Sensor<'a>>,
    pub modbus_address: i32,
}



impl<'a> Module<'a> {
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
                    modbus_address: 1,
                }
            }
        }
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
