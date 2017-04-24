use std::collections::HashSet;

use exception::{Exception, ExceptionType};
use shift_register::ShiftRegister;


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Sensor {
    value: u64,
    reverse: bool, // Boolen um die Richtung bei der Simulation zu halten.
    error_count: u64,
}
impl Sensor {
    pub fn new() -> Self {
        Sensor {
            value: 0,
            reverse: false,
            error_count: 5,
        }
    }

    pub fn check(&mut self,
                 num_zone: usize,
                 num: usize,
                 exceptions: &mut HashSet<Exception>,
                 leds: &mut ShiftRegister,
                 relais: &mut ShiftRegister) {
        debug!("\t\t\t\tcheck() Sensor ...");
        self.check_direct_value(num_zone, num, exceptions, leds, relais);
    }

    pub fn update(&mut self,
                  _num_zone: usize,
                  _num: usize,
                  _exceptions: &mut HashSet<Exception>,
                  _leds: &mut ShiftRegister,
                  _relais: &mut ShiftRegister) {
        debug!("\t\t\t\tupdate() Sensor ...");
        if self.value == 300 {
            self.reverse = true
        }
        if !self.reverse {
            self.value += 1
        } else {
            self.value -= 1
        }
        if self.value == 0 {
            self.reverse = false
        }
    }

    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::sensor::Sensor;
    ///
    /// let mut sensor = Sensor::new();
    /// sensor.reset_error_count();
    /// ```
    pub fn reset_error_count(&mut self) {
        self.error_count = 0;
    }

    fn check_direct_value(&mut self,
                          num_zone: usize,
                          num: usize,
                          exceptions: &mut HashSet<Exception>,
                          leds: &mut ShiftRegister,
                          relais: &mut ShiftRegister) {
        let direktwert_ueberschritten = Exception::new(ExceptionType::SensorDirectValue {
            zone: num_zone,
            sensor: num,
        });
        if self.value >= 150 {
            if !exceptions.contains(&direktwert_ueberschritten) {
                exceptions.insert(direktwert_ueberschritten);
            }
            leds.set(5).ok();
            leds.set(6).ok();
            leds.set(7).ok();
            relais.set(2).ok();
            relais.set(3).ok();
            relais.set(4).ok();
        } else if self.value < 150 {
            if exceptions.contains(&direktwert_ueberschritten) {
                exceptions.remove(&direktwert_ueberschritten);
            }
            leds.clear(5).ok();
            leds.clear(6).ok();
            leds.clear(7).ok();
            relais.clear(2).ok();
            relais.clear(3).ok();
            relais.clear(4).ok();
        }
    }
}
impl Default for Sensor {
    fn default() -> Self {
        Self::new()
    }
}
