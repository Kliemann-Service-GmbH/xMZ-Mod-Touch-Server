use std::thread;
use std::time::Duration;

use zones::Zone;
use shift_register::{ShiftRegister, ShiftRegisterType};

pub struct Server {
    zones: Vec<Zone>,
    leds: ShiftRegister,
    relais: ShiftRegister,
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
        }
    }

    pub fn update(&mut self) {
        println!("Server::update() ...");
        
        for (num, zone) in &mut self.zones.iter_mut().enumerate() {
            zone.update(num);
        }
        thread::sleep(Duration::from_millis(500));
    }

    pub fn check_exceptions(&mut self) {
        println!("Server::check_exceptions() ...");
        thread::sleep(Duration::from_millis(100));
    }
}
