extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashSet;

use futures::{Future, Stream};
use tokio_io::{io, AsyncRead};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;


#[derive(Debug)]
struct XMZServer {
    start_time: Instant,
    uptime: u64,
    exceptions: HashSet<Exception>,
    zones: Vec<Zone>,
    leds: ShiftRegister,
    relais: ShiftRegister,
}
impl XMZServer {
    fn new() -> Self {
        XMZServer {
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
    fn check_uptime(&mut self) {
        if self.uptime > 5 {
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
    fn uptime(&self) -> u64 {
        self.uptime
    }
    fn check(&mut self) {
        println!("\tcheck() XMZServer ...");
        self.check_uptime();
        for (num, mut zone) in &mut self.zones.iter_mut().enumerate() {
            zone.check(&mut self.exceptions, &mut self.leds, &mut self.relais);
        }
    }
    fn update(&mut self) {
        println!("\tupdate() XMZServer ...");
        self.uptime = self.start_time.elapsed().as_secs();
        for (num, mut zone) in &mut self.zones.iter_mut().enumerate() {
            zone.update(&mut self.exceptions, &mut self.leds, &mut self.relais);
        }
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
    fn check(&mut self, exceptions: &mut HashSet<Exception>, leds: &mut ShiftRegister, relais: &mut ShiftRegister) {
        println!("\t\tcheck() Zone ...");
        for (num, mut kombisensor) in &mut self.kombisensors.iter_mut().enumerate() {
            kombisensor.check(num, exceptions, leds, relais);
        }
    }
    fn update(&mut self, exceptions: &mut HashSet<Exception>, leds: &mut ShiftRegister, relais: &mut ShiftRegister) {
        println!("\t\tupdate() Zone ...");
        for (num, mut kombisensor) in &mut self.kombisensors.iter_mut().enumerate() {
            kombisensor.update(num, exceptions, leds, relais);
        }
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
    fn is_online(&self) -> bool {
        false
    }
    fn check(&mut self, num_zone: usize, exceptions: &mut HashSet<Exception>, leds: &mut ShiftRegister, relais: &mut ShiftRegister) {
        println!("\t\t\tcheck() Kombisensor ...");
        for (num, mut sensor) in &mut self.sensors.iter_mut().enumerate() {
            sensor.check(num_zone, num, exceptions, leds, relais);
        }
    }
    fn update(&mut self, num_zone: usize, exceptions: &mut HashSet<Exception>, leds: &mut ShiftRegister, relais: &mut ShiftRegister) {
        println!("\t\t\tupdate() Kombisensor ...");
        for (num, mut sensor) in &mut self.sensors.iter_mut().enumerate() {
            sensor.update(num_zone, num, exceptions, leds, relais);
        }
    }
}
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
    fn check_direct_value(&mut self, num_zone: usize, num: usize, exceptions: &mut HashSet<Exception>, leds: &mut ShiftRegister, relais: &mut ShiftRegister) {
        if self.value > 100 {
            let direktwert_ueberschritten = Exception::new(ExceptionType::SensorDirectValue { zone: num_zone, sensor: num });
            if !exceptions.contains(&direktwert_ueberschritten) { exceptions.insert(direktwert_ueberschritten); }
            leds.set(1);
            relais.set(3);
        }
    }
    fn check(&mut self, num_zone: usize, num: usize, exceptions: &mut HashSet<Exception>, leds: &mut ShiftRegister, relais: &mut ShiftRegister) {
        println!("\t\t\t\tcheck() Sensor ...");
        self.check_direct_value(num_zone, num, exceptions, leds, relais);
    }
    fn update(&mut self, num_zone: usize, num: usize, exceptions: &mut HashSet<Exception>, leds: &mut ShiftRegister, relais: &mut ShiftRegister) {
        println!("\t\t\t\tupdate() Sensor ...");
        if self.value == 300 { self.reverse = true }
        if !self.reverse { self.value += 1 } else { self.value -= 1 }
        if self.value == 0 { self.reverse = false }
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
    fn set(&mut self, num: usize) {
        println!("{:?}: SET Pin: {}", self.shift_register_type, num);
        self.data |= 1 << num -1;
    }
    fn clear(&mut self, num: usize) {
        println!("{:?}: CLEAR Pin: {}", self.shift_register_type, num);
        self.data &= !(1 << num - 1);
    }
}
#[derive(Debug)]
#[derive(Hash, Eq, PartialEq)]
enum ExceptionType {
    WartungsintervalReached,
    KombisensorOffline { zone: usize },
    SensorDirectValue { zone: usize, sensor: usize },
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
    let mut xmz_server = Arc::new(Mutex::new(XMZServer::new()));


    std::thread::spawn(move || {
        // Create the event loop that will drive this server
        let mut core = Core::new().unwrap();
        let handle = core.handle();

        // Bind the server's sockte
        let addr = "127.0.0.1:12345".parse().unwrap();
        let tcp = TcpListener::bind(&addr, &handle).unwrap();

        // Iterate incoming connections
        let server = tcp.incoming().for_each(|(tcp, _)| {
            // Split up the read and write halves
            let (reader, writer) = tcp.split();

            // Future of the copy
            let bytes_copied = io::copy(reader, writer);

            // ... after which we'll print what happened
            let handle_conn = bytes_copied.map(|(n, _, _)| {
                println!("wrote {} bytes", n)
            }).map_err(|err| {
                println!("IO error: {:?}", err)
            });

            // Spawn the future as a concurrent task
            handle.spawn(handle_conn);

            Ok(())
        });

        // Spin up the server on the event loop
        core.run(server).unwrap();
    });

    let xmz_server2 = xmz_server.clone();
    loop {
        if let Ok(mut xmz_server) = xmz_server2.lock() {
            // Ausnahmen prüfen
            xmz_server.check();

            // XMZServer Kombonenten aktualisieren, Kombisensoren auslesen, ....
            xmz_server.update();

            println!("{:#?}", &*xmz_server);

            thread::sleep(Duration::from_millis(100));
        }
    }
}
