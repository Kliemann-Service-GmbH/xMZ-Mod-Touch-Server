use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashSet;


#[derive(Debug)]
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
    fn update(&mut self) {
        if self.value == 300 { self.reverse = true }
        if !self.reverse { self.value += 1 } else { self.value -= 1 }
        if self.value == 0 { self.reverse = false }
    }
    fn check(&mut self) {
        // println!("Sensor checked!");
    }
}

#[derive(Debug)]
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
    fn update(&mut self) {
        println!("Kombisensor update ...");
        for mut sensor in &mut self.sensors {
            sensor.update();
        }
    }

    fn is_online(&self) -> bool {
        false
    }
}

#[derive(Debug)]
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
    fn update(&mut self) {
        println!("Zone update ...");
        for mut kombisensor in &mut self.kombisensors {
            kombisensor.update();
        }
    }
}

#[derive(Debug)]
enum ShiftRegisterType {
    LED,
    Relais,
}
#[derive(Debug)]
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
    fn set(&mut self, num: u64) {
        println!("{:?}: SET Pin: {}", self.shift_register_type, num);
        self.data |= 1 << num -1;
    }
    fn clear(&mut self, num: u64) {
        println!("{:?}: CLEAR Pin: {}", self.shift_register_type, num);
        self.data &= !(1 << num - 1);
    }
}

#[derive(Debug)]
struct Server {
    start_time: Instant,
    uptime: u64,
    exceptions: HashSet<Exception>,
    zones: Vec<Zone>,
    leds: ShiftRegister,
    relais: ShiftRegister,
}
impl Server {
    fn new() -> Self {
        Server {
            start_time: Instant::now(),
            uptime: 0,
            exceptions: HashSet::new(),
            zones: vec![
                Zone::new(),
            ],
            leds:   ShiftRegister::new(ShiftRegisterType::LED),
            relais: ShiftRegister::new(ShiftRegisterType::Relais),
        }
    }
    fn update(&mut self) {
        println!("Server update ...");
        self.uptime = self.start_time.elapsed().as_secs();
        for mut zone in &mut self.zones {
            zone.update();
        }
    }

    fn check_uptime(&mut self) {
        if self.uptime > 5 {
            self.leds.set(2);
            self.leds.set(3);
            self.relais.clear(1);
            self.add_exception( Exception::new(ExceptionType::WartungsintervalReached) );
        }
    }
    fn check_kombisensors(&self, zone: &mut Zone) {
        for mut kombisensor in &mut zone.kombisensors {

        }
    }

    fn check_zones(&mut self) {
        for mut zone in &mut self.zones {
            self.check_kombisensors(&mut zone);
        }
    }
    fn check(&mut self) {
        self.check_uptime();
        self.check_zones();
    }
    fn add_exception(&mut self, exception: Exception) {
        if !self.exceptions.contains(&exception) {
            self.exceptions.insert(exception);
        }
    }
    fn uptime(&self) -> u64 {
        self.uptime
    }
}
#[derive(Debug)]
#[derive(Hash, Eq, PartialEq)]
enum ExceptionType {
    WartungsintervalReached,
    KombisensorOffline { zone: u32 },
}
#[derive(Debug)]
#[derive(Hash, Eq, PartialEq)]
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




fn main() {
    let mut server = Server::new();

    loop {
        // Ausnahmen prüfen
        server.check();

        // Server Kombonenten aktualisieren, Kombisensoren auslesen, ....
        server.update();

        println!("{:#?}", &server);

        thread::sleep(Duration::from_millis(500));
    }
}
