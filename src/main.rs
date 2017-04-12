//
// * `#[derive(Serialize, Deserialize)]`
//
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
extern crate env_logger;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashSet;
use chrono::{DateTime, UTC};


pub const SERVER_MAX_UPTIME_SEC: i64 = 5;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct XMZServer {
    start_time: DateTime<UTC>,
    exceptions: HashSet<Exception>,
    zones: Vec<Zone>,
    leds: ShiftRegister,
    relais: ShiftRegister,
}
impl XMZServer {
    fn new() -> Self {
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
    fn check_uptime(&mut self) {
        if chrono::UTC::now().signed_duration_since(self.start_time) > chrono::Duration::seconds(SERVER_MAX_UPTIME_SEC) {
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
    fn check(&mut self) {
        debug!("\tcheck() XMZServer ...");
        self.check_uptime();
        for (num, mut zone) in &mut self.zones.iter_mut().enumerate() {
            zone.check(&mut self.exceptions, &mut self.leds, &mut self.relais);
        }
    }
    fn update(&mut self) {
        debug!("\tupdate() XMZServer ...");
        for (num, mut zone) in &mut self.zones.iter_mut().enumerate() {
            zone.update(&mut self.exceptions, &mut self.leds, &mut self.relais);
        }
    }
}
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Zone {
    kombisensors: Vec<Kombisensor>,
}
impl Zone {
    fn new() -> Self {
        Zone {
            kombisensors: vec![
                Kombisensor::new(),
                Kombisensor::new(),
            ]
        }
    }
    fn check(&mut self, exceptions: &mut HashSet<Exception>, leds: &mut ShiftRegister, relais: &mut ShiftRegister) {
        debug!("\t\tcheck() Zone ...");
        for (num, mut kombisensor) in &mut self.kombisensors.iter_mut().enumerate() {
            kombisensor.check(num, exceptions, leds, relais);
        }
    }
    fn update(&mut self, exceptions: &mut HashSet<Exception>, leds: &mut ShiftRegister, relais: &mut ShiftRegister) {
        debug!("\t\tupdate() Zone ...");
        for (num, mut kombisensor) in &mut self.kombisensors.iter_mut().enumerate() {
            kombisensor.update(num, exceptions, leds, relais);
        }
    }
}
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Kombisensor {
    sensors: Vec<Sensor>,
}
impl Kombisensor {
    fn new() -> Self {
        Kombisensor {
            sensors: vec![
                Sensor::new(),
                Sensor::new(),
            ]
        }
    }
    fn is_online(&self) -> bool {
        false
    }
    fn check(&mut self, num_zone: usize, exceptions: &mut HashSet<Exception>, leds: &mut ShiftRegister, relais: &mut ShiftRegister) {
        debug!("\t\t\tcheck() Kombisensor ...");
        for (num, mut sensor) in &mut self.sensors.iter_mut().enumerate() {
            sensor.check(num_zone, num, exceptions, leds, relais);
        }
    }
    fn update(&mut self, num_zone: usize, exceptions: &mut HashSet<Exception>, leds: &mut ShiftRegister, relais: &mut ShiftRegister) {
        debug!("\t\t\tupdate() Kombisensor ...");
        for (num, mut sensor) in &mut self.sensors.iter_mut().enumerate() {
            sensor.update(num_zone, num, exceptions, leds, relais);
        }
    }
}
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Sensor {
    value: u64,
    reverse: bool,   // Boolen um die Richtung bei der Simulation zu halten.
}
impl Sensor {
    fn new() -> Self {
        Sensor {
            value: 0,
            reverse: false,
        }
    }
    fn check_direct_value(&mut self, num_zone: usize, num: usize, exceptions: &mut HashSet<Exception>, leds: &mut ShiftRegister, relais: &mut ShiftRegister) {
        let direktwert_ueberschritten = Exception::new(ExceptionType::SensorDirectValue { zone: num_zone, sensor: num });
        if self.value >= 150 {
            if !exceptions.contains(&direktwert_ueberschritten) { exceptions.insert(direktwert_ueberschritten); }
            leds.set(1);
            relais.set(3);
        } else if self.value < 150 {
            if exceptions.contains(&direktwert_ueberschritten) { exceptions.remove(&direktwert_ueberschritten); }
            leds.clear(1);
            relais.clear(3);
        }
    }
    fn check(&mut self, num_zone: usize, num: usize, exceptions: &mut HashSet<Exception>, leds: &mut ShiftRegister, relais: &mut ShiftRegister) {
        debug!("\t\t\t\tcheck() Sensor ...");
        self.check_direct_value(num_zone, num, exceptions, leds, relais);
    }
    fn update(&mut self, num_zone: usize, num: usize, exceptions: &mut HashSet<Exception>, leds: &mut ShiftRegister, relais: &mut ShiftRegister) {
        debug!("\t\t\t\tupdate() Sensor ...");
        if self.value == 300 { self.reverse = true }
        if !self.reverse { self.value += 1 } else { self.value -= 1 }
        if self.value == 0 { self.reverse = false }
    }
}
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
enum ShiftRegisterType {
    LED,
    Relais,
}
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct ShiftRegister {
    shift_register_type: ShiftRegisterType,
    data: u64,
}
impl ShiftRegister {
    fn new(shift_register_type: ShiftRegisterType) -> Self {
        match shift_register_type {
            ShiftRegisterType::LED => ShiftRegister { shift_register_type: shift_register_type, data: 0 },
            ShiftRegisterType::Relais => ShiftRegister { shift_register_type: shift_register_type, data: 0 },
        }
    }
    fn set(&mut self, num: usize) {
        debug!("{:?}: SET Pin: {}", self.shift_register_type, num);
        self.data |= 1 << num -1;
    }
    fn clear(&mut self, num: usize) {
        debug!("{:?}: CLEAR Pin: {}", self.shift_register_type, num);
        self.data &= !(1 << num - 1);
    }
}
#[derive(Debug)]
#[derive(Hash, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
enum ExceptionType {
    WartungsintervalReached,
    KombisensorOffline { zone: usize },
    SensorDirectValue { zone: usize, sensor: usize },
}
#[derive(Debug)]
#[derive(Hash, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
struct Exception {
    exception_type: ExceptionType,
}
impl Exception {
    fn new(exception_type: ExceptionType) -> Self {
        Exception {
            exception_type: exception_type,
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


fn main() {
    let mut xmz_server = Arc::new(Mutex::new(XMZServer::new()));

    let xmz_server1 = xmz_server.clone();
    std::thread::spawn(move || {
        rocket::ignite().mount("/", routes![index]).launch();
    });

    let xmz_server2 = xmz_server.clone();
    // for _ in 0..100 {
    loop {
        if let Ok(mut xmz_server) = xmz_server2.lock() {
            // Ausnahmen prüfen
            xmz_server.check();

            // XMZServer Kombonenten aktualisieren, Kombisensoren auslesen, ....
            xmz_server.update();

            // println!("{:#?}", &*xmz_server);

            thread::sleep(Duration::from_millis(100));
        }
    }
}
