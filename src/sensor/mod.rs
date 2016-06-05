#[derive(Debug,Eq,PartialEq)]
pub enum SensorType {
    NEMOTO_CO,
    NEMOTO_NO,
}

pub struct Sensor {
    pub adc_value: u32,
    pub sensor_type: SensorType,
}

impl Sensor {
    pub fn new(sensor_type: SensorType) -> Self {
        Sensor { adc_value: 0, sensor_type: sensor_type }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn basics() {
        assert!(true);
    }
}
