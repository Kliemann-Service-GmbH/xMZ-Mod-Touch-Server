use ::chrono::{DateTime, UTC};
use std::collections::HashSet;

use exception::{Exception, ExceptionType};
use zone::Zone;
use shift_register::{ShiftRegister, ShiftRegisterType};



pub const SERVER_MAX_UPTIME_SEC: i64 = 5;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct XMZServer {
    start_time: DateTime<UTC>,
    exceptions: HashSet<Exception>,
    zones: Vec<Zone>,
    leds: ShiftRegister,
    relais: ShiftRegister,
}
impl XMZServer {
    pub fn new() -> Self {
        XMZServer {
            start_time: UTC::now(),
            exceptions: HashSet::new(),
            zones: vec![
                Zone::new(),
            ],
            leds:   ShiftRegister::new(ShiftRegisterType::LED),
            relais: ShiftRegister::new(ShiftRegisterType::Relais),
        }
    }

    pub fn check(&mut self) {
        debug!("\tcheck() XMZServer ...");
        self.check_uptime();
        for (_num, mut zone) in &mut self.zones.iter_mut().enumerate() {
            zone.check(&mut self.exceptions, &mut self.leds, &mut self.relais);
        }
    }

    pub fn update(&mut self) {
        debug!("\tupdate() XMZServer ...");
        for (_num, mut zone) in &mut self.zones.iter_mut().enumerate() {
            zone.update(&mut self.exceptions, &mut self.leds, &mut self.relais);
        }
    }

    pub fn get_exceptions(&self) -> &HashSet<Exception> {
        &self.exceptions
    }

    pub fn get_exception(&self, _id: usize) -> Option<&Exception> {
        None
    }

    pub fn get_zones(&self) -> &Vec<Zone> {
        &self.zones
    }

    pub fn get_zone(&self, id: usize) -> Option<&Zone> {
        *&self.zones.get(id)
    }


    fn check_uptime(&mut self) {
        if ::chrono::UTC::now().signed_duration_since(self.start_time) > ::chrono::Duration::seconds(SERVER_MAX_UPTIME_SEC) {
            self.leds.set(2);
            self.leds.set(3);
            self.relais.clear(1);
            self.add_exception( Exception::new(ExceptionType::WartungsintervalReached) );
        }
    }

    fn add_exception(&mut self, exception: Exception) {
        if !self.exceptions.contains(&exception) {
            self.exceptions.insert(exception);
        }
    }

}