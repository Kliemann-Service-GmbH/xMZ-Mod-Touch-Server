use sensors::{Sensor, SensorType};

pub struct Kombisensor {
    sensors: Vec<Sensor>,
}

impl Kombisensor {
    pub fn new() -> Self {
        Kombisensor {
            sensors: vec![
                Sensor::new(SensorType::NemotoNO2),
                Sensor::new(SensorType::NemotoCO),
            ]
        }
    }

    pub fn update(&mut self, num: usize) {
        println!("Kombisensor[{}]::update() ...", num);

        for (num, sensor) in &mut self.sensors.iter_mut().enumerate() {
            sensor.update(num);
        }
    }
}
