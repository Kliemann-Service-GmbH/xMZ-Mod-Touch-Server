use kombisensors::Kombisensor;

pub struct Zone {
    kombisensors: Vec<Kombisensor>,
}

impl Zone {
    pub fn new() -> Self {
        Zone {
            kombisensors: vec![
                Kombisensor::new(),
                Kombisensor::new(),
            ],
        }
    }

    /// Siehe `Server::update()`
    pub fn update(&mut self, num: usize) {
        println!("Zone[{}]::update() ...", num);

        for (num, kombisensor) in &mut self.kombisensors.iter_mut().enumerate() {
            kombisensor.update(num);
        }
    }
}
