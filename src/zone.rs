use exception::Exception;
use kombisensor::Kombisensor;
use shift_register::ShiftRegister;
use std::collections::HashSet;


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
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

    pub fn check(&mut self,
                 exceptions: &mut HashSet<Exception>,
                 leds: &mut ShiftRegister,
                 relais: &mut ShiftRegister) {
        debug!("\t\tcheck() Zone ...");
        for (num, mut kombisensor) in &mut self.kombisensors.iter_mut().enumerate() {
            kombisensor.check(num, exceptions, leds, relais);
        }
    }

    pub fn update(&mut self,
                  exceptions: &mut HashSet<Exception>,
                  leds: &mut ShiftRegister,
                  relais: &mut ShiftRegister) {
        debug!("\t\tupdate() Zone ...");
        for (num, mut kombisensor) in &mut self.kombisensors.iter_mut().enumerate() {
            kombisensor.update(num, exceptions, leds, relais);
        }
    }

    pub fn get_kombisensors(&self) -> &Vec<Kombisensor> {
        &self.kombisensors
    }

    pub fn get_kombisensors_mut(&mut self) -> &mut Vec<Kombisensor> {
        &mut self.kombisensors
    }

    pub fn get_kombisensor(&self, id: usize) -> Option<&Kombisensor> {
        self.kombisensors.get(id)
    }

    pub fn get_kombisensor_mut(&mut self, id: usize) -> Option<&mut Kombisensor> {
        self.kombisensors.get_mut(id)
    }
}

impl Default for Zone {
    fn default() -> Self {
        Self::new()
    }
}
