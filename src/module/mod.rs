use sensor::{Sensor, SensorType};


pub struct Module<'a> {
    sensor: Vec<Sensor<'a>>,
}

impl<'a> Module<'a> {
    pub fn new() -> Self {
        Module {
            sensor: vec![
                Sensor::new(SensorType::NemotoCO),
                Sensor::new(SensorType::NemotoNO),
            ]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use sensor::{SensorType};

    #[test]
    fn default_module_has_2_sensor() {
        let module = Module::new();
        assert_eq!(module.sensor.len(), 2);
    }

    #[test]
    fn default_module_sensor1() {
        let module = Module::new();
        assert_eq!(module.sensor[0].sensor_type, SensorType::NemotoCO);
        assert_eq!(module.sensor[1].sensor_type, SensorType::NemotoNO);
    }
}
