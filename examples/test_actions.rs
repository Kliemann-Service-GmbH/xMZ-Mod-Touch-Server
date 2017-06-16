extern crate xmz_mod_touch_server;

use xmz_mod_touch_server::{ShiftRegister, ShiftRegisterType};
use xmz_mod_touch_server::{Action};



fn main() {
    let leds = ShiftRegister::new(ShiftRegisterType::Simulation);
    let relais = ShiftRegister::new(ShiftRegisterType::Simulation);

    let action1 = Action::new(&leds, ShiftRegister::set, 2);
    let action2 = Action::new(&relais, ShiftRegister::clear, 1);

    let actions: Vec<Box<Action<ShiftRegister, u64>>> = vec![Box::new(action1), Box::new(action2)];

    // Startbedingungen
    relais.set(1);
    assert_eq!(leds.get_data().unwrap(), 0b0);
    assert_eq!(relais.get_data().unwrap(), 0b1);

    // action1.run();
    // action2.run();
    for action in actions {
        action.run();
    }

    // Nachdem die Actionen ausgeführt wurden, Endbedingen prüfen
    assert_eq!(leds.get_data().unwrap(), 0b10);
    assert_eq!(relais.get_data().unwrap(), 0b0);
}
