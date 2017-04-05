use exceptions::{Exception, ExceptionType};
use shift_register::{ShiftRegister, ShiftRegisterType};
use std::thread;
use std::time::Duration;
use std::time::Instant;
use zones::Zone;
use std::collections::HashSet;


pub struct Server {
    zones: Vec<Zone>,
    leds: ShiftRegister,
    relais: ShiftRegister,
    startup_time: Instant,
    exceptions: HashSet<Exception>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            zones: vec![
                Zone::new(),
                Zone::new(),
            ],
            leds: ShiftRegister::new(ShiftRegisterType::LED),
            relais: ShiftRegister::new(ShiftRegisterType::Relais),
            startup_time: Instant::now(),
            exceptions: HashSet::new(),
        }
    }
    /// Getter
    pub fn exceptions(&self) -> &HashSet<Exception> {
        &self.exceptions
    }

    pub fn update(&mut self) {
        info!("Server::update() ...");

        for (num, zone) in &mut self.zones.iter_mut().enumerate() {
            zone.update(num);
        }
        thread::sleep(Duration::from_millis(500));
    }

    pub fn uptime(&self) -> f64 {
        let elapsed = self.startup_time.elapsed();
        let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1_000_000_000.0);

        sec
    }

    pub fn check_exceptions(&mut self) {
        info!("Server::check_exceptions() ...");

        // Wartungsintervall
        // if self.uptime() > 365.0f64*24.0*60.0*60.0 { info!("Ein Jahr is rumm!"); }
        // info!("{} {}", 1.0f64*1.0*1.0*10.0, self.uptime());
        if self.uptime() > 1.0f64*1.0*1.0*10.0 {
            let exception = Exception::new(ExceptionType::Wartungsintervall);
            if !self.exceptions.contains(&exception) {
                self.exceptions.insert(exception);
            }
        }

        thread::sleep(Duration::from_millis(100));
    }
}
