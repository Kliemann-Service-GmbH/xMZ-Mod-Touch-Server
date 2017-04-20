use std::collections::HashSet;

use sensor::Sensor;
use shift_register::ShiftRegister;
use exception::Exception;


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Kombisensor {
    sensors: Vec<Sensor>,
}
impl Kombisensor {
    pub fn new() -> Self {
        Kombisensor {
            sensors: vec![
                Sensor::new(),
                Sensor::new(),
            ]
        }
    }
    pub fn check(&mut self, num_zone: usize, exceptions: &mut HashSet<Exception>, leds: &mut ShiftRegister, relais: &mut ShiftRegister) {
        debug!("\t\t\tcheck() Kombisensor ...");
        for (num, mut sensor) in &mut self.sensors.iter_mut().enumerate() {
            sensor.check(num_zone, num, exceptions, leds, relais);
        }
    }
    
    pub fn update(&mut self, num_zone: usize, exceptions: &mut HashSet<Exception>, leds: &mut ShiftRegister, relais: &mut ShiftRegister) {
        debug!("\t\t\tupdate() Kombisensor ...");
        for (num, mut sensor) in &mut self.sensors.iter_mut().enumerate() {
            sensor.update(num_zone, num, exceptions, leds, relais);
        }
    }


    fn is_online(&self) -> bool {
        false
    }
}