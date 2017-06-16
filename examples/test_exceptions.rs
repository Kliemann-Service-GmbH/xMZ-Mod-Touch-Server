extern crate xmz_mod_touch_server;

use xmz_mod_touch_server::{Action, ShiftRegister, ShiftRegisterType};


pub struct Exception<'a> {
    actions: Vec<Box<Action<'a, ShiftRegister, u64>>>,
}

impl<'a> Exception<'a> {
    pub fn new() -> Self {
        Exception {
            actions: vec![],
        }
    }
}


fn main() {
    let leds = ShiftRegister::new(ShiftRegisterType::Simulation);
    let relais = ShiftRegister::new(ShiftRegisterType::Simulation);

    let wartungsintervall = Exception {
        actions: vec![
            Box::new(Action::new(&leds, ShiftRegister::set, 2)),
            Box::new(Action::new(&relais, ShiftRegister::clear, 1)),
        ],
    };

    let kabelbruch_zone1_kombisensor1 = Exception {
        actions: vec![
            Box::new(Action::new(&leds, ShiftRegister::set, 2)),
            Box::new(Action::new(&relais, ShiftRegister::clear, 1)),
        ],
    };

    let exceptions = vec![wartungsintervall, kabelbruch_zone1_kombisensor1];
}
