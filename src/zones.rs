use server::Server;
use kombisensors::Kombisensor;
use std::rc::Rc;

#[derive(Clone)]
#[derive(Debug)]
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
        debug!("Zone[{}]::update() ...", num);

        for (num, kombisensor) in &mut self.kombisensors.iter_mut().enumerate() {
            kombisensor.update(num);
        }
    }

    pub fn check_exceptions(&self, server: Rc<&mut Server>) {
        println!("Check Exception in Zone");
    }
}
