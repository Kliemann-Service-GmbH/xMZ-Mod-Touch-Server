
pub enum SensorType {
    NemotoNO2,
    NemotoCO,
}

pub struct Sensor {
    sensor_type: SensorType,
}

impl Sensor {
    pub fn new(sensor_type: SensorType) -> Self {
        match sensor_type {
            SensorType::NemotoNO2 => Sensor { sensor_type: sensor_type },
            SensorType::NemotoCO => Sensor { sensor_type: sensor_type },
        }
    }

    pub fn update(&mut self, num: usize) {
        println!("Sensor[{}]::update() ...", num);
    }
}
