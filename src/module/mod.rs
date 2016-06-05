use sensor::{Sensor, SensorType};
use std::default::Default;

enum ModuleType {
    RA_GAS_CO_NO,
}

pub struct Module {
    module_type: ModuleType,
    sensor: Vec<Sensor>,
}

impl Module {
    pub fn new() -> Self {
        let sensor1 = Sensor::new(SensorType::NEMOTO_CO);
        let sensor2 = Sensor::new(SensorType::NEMOTO_NO);
        Module {
            module_type: ModuleType::RA_GAS_CO_NO,
            sensor: vec![sensor1, sensor2]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use sensor::{Sensor, SensorType};

    #[test]
    fn default_module_has_2_sensor() {
        let module = Module::new();
        assert_eq!(module.sensor.len(), 2);
    }

    #[test]
    fn default_module_sensor1() {
        let module = Module::new();
        assert_eq!(module.sensor[0].sensor_type, SensorType::NEMOTO_CO);
        assert_eq!(module.sensor[1].sensor_type, SensorType::NEMOTO_NO);
    }
}
