use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};
use std::fmt;


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
            ShiftRegisterType::LED => ShiftRegister { shift_register_type: shift_register_type, data: 0, },
            ShiftRegisterType::Relais => ShiftRegister { shift_register_type: shift_register_type, data: 0, },
        }
    }
    fn set(&mut self, num: u64) {
        println!("{:?}: SET Pin: {}", self.shift_register_type, num);
    }
    fn clear(&mut self, num: u64) {
        println!("{:?}: CLEAR Pin: {}", self.shift_register_type, num);
    }
}
#[derive(Debug)]
struct Server {
    exceptions: Rc<RefCell<Vec<Box<Exception>>>>,
    leds: Rc<RefCell<ShiftRegister>>,
    relais: Rc<RefCell<ShiftRegister>>,
}
impl Server {
    fn new() -> Self {
        Server {
            exceptions: Rc::new(RefCell::new(vec![])),
            leds: Rc::new(RefCell::new(ShiftRegister::new(ShiftRegisterType::LED))),
            relais: Rc::new(RefCell::new(ShiftRegister::new(ShiftRegisterType::Relais))),
        }
    }
    fn add_exception(&self, exception: Box<Exception>) {
        self.exceptions.borrow_mut().push(exception);
    }
    fn check_exceptions(&self) {
        for exception in self.exceptions.borrow().iter() {
            println!("Prüfe Ausnahmen ...");
            exception.check();
        }
    }
    fn update(&self) {
        println!("Update Server ...");
    }
}

trait Exception {
    fn check(&self);
}
#[derive(Debug)]
struct Wartungsintervall {
    server: Rc<RefCell<Server>>,
}
impl Wartungsintervall {
    fn new(server: Rc<RefCell<Server>>) -> Self {
        Wartungsintervall {
            server: server,
        }
    }
}
impl Exception for Wartungsintervall {
    fn check(&self) {
        self.server.borrow().leds.borrow_mut().set(3);
        self.server.borrow().relais.borrow_mut().clear(1);
    }
}

impl  fmt::Debug for Box<Exception> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Box<Exception>")
    }
}



fn main() {
    let server = Rc::new(RefCell::new(Server::new()));
    let Wartungsintervall = Wartungsintervall::new(server.clone());
    server.borrow().add_exception(Box::new(Wartungsintervall));

    println!("{:#?}", server);

    loop {
        server.borrow().check_exceptions();

        server.borrow().update();

        thread::sleep(Duration::from_millis(500));
    }

}
