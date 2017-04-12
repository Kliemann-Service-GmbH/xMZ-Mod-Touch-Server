use std::time::{Instant, Duration};
use std::thread;
use std::rc::Rc;
use std::cell::RefCell;


static const SERVER_MAX_UPTIME: u64 = 5; // Sekunden == 365 Tage

#[derive(Debug)]
struct Server {
    start_time: Instant,
    uptime: u64,
    leds: Rc<RefCell<ShiftRegister>>,
    relais: Rc<RefCell<ShiftRegister>>,
}
impl Server {
    fn new() -> Self {
        Server {
            start_time: Instant::now(),
            uptime: 0,
        }
    }
    fn check_uptime(&self) {
        if self.uptime() > SERVER_MAX_UPTIME {

        }
    }
    fn check(&self) {
        println!("Check Server ...");
    }
    fn update(&self) {
        println!("Update Server ...");
    }
}

#[derive(Debug)]
struct Kombisensor {

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



fn main() {
    let server = Server::new();

    loop {
        server.check();

        server.update();

        thread::sleep(Duration::from_millis(500));
    }


}
