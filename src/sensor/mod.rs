#[derive(Debug,Eq,PartialEq)]
pub enum SensorType {
    NemotoCO,
    NemotoNO,
}

pub struct Sensor {
    pub adc_value: Option<u32>,
    pub sensor_type: SensorType,
}

impl Sensor {
    pub fn new(sensor_type: SensorType) -> Self {
        Sensor { adc_value: None, sensor_type: sensor_type }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn basics() {
        assert!(true);
    }
}
