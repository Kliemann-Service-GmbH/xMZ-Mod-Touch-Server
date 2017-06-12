extern crate xmz_mod_touch_server;

use xmz_mod_touch_server::{ShiftRegister, ShiftRegisterType};


struct Action<'a> {
    reg: &'a ShiftRegister,
    fun: fn(&ShiftRegister, u64) -> Result<(), xmz_mod_touch_server::errors::Error>,
    pin: u64,
}
impl<'a> Action<'a> {
    fn run(& self) {
        (self.fun)(self.reg, self.pin);
    }
}

fn main() {
    let leds = ShiftRegister::new(ShiftRegisterType::Simulation);
    let relais = ShiftRegister::new(ShiftRegisterType::Simulation);

    let action1 = Action {
        reg: &leds,
        fun: ShiftRegister::set,
        pin: 2,
    };

    let action2 = Action {
        reg: &relais,
        fun: ShiftRegister::clear,
        pin: 1,
    };

    assert_eq!(leds.get_data().unwrap(), 0b0);
    assert_eq!(relais.get_data().unwrap(), 0b0);

    action1.run();
    action2.run();

    assert_eq!(leds.get_data().unwrap(), 0b10);
    assert_eq!(relais.get_data().unwrap(), 0b1);
}
