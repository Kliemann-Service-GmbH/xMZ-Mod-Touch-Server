use shift_register::{ShiftRegister, ShiftRegisterType};
use std::collections::HashSet;
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use zones::Zone;

use exceptions::{Action, Exception, ExceptionType};

#[derive(Clone)]
#[derive(Debug)]
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
        debug!("Server::update() ...");

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
        debug!("Server::check_exceptions() ...");
        let mut server = Rc::new(self);

        // Wartungsintervall
        let server1 = server.clone();
        {
            check_wartungsintervall(server1);
        }

        // Zonen -> Kombisensoren
        let server1 = server.clone();
        {
            check_zones(server1);
        }

        // Zonen -> Kombisensoren -> Sensore
        thread::sleep(Duration::from_millis(100));
    }
}


fn check_wartungsintervall(server: Rc<&mut Server>) {
    let mut server = server;

    println!("{:?}", server)
}


fn check_zones(server: Rc<&mut Server>) {
    println!("{:?}", server)
}
