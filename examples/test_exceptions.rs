extern crate xmz_mod_touch_server;

use xmz_mod_touch_server::{Action, ShiftRegister, ShiftRegisterType};


pub struct Exception<'a>
{
    actions: Vec<Box<Action<'a, ShiftRegister, u64>>>,
}

impl<'a> Exception<'a>
{
    pub fn new(actions: Vec<Box<Action<'a, ShiftRegister, u64>>>) -> Self {
        Exception {
            actions,
        }
    }
}


fn main() {
    let leds = ShiftRegister::new(ShiftRegisterType::Simulation);
    let relais = ShiftRegister::new(ShiftRegisterType::Simulation);

    use xmz_mod_touch_server::XMZModTouchServer;
    let mut xmz_mod_touch_server = XMZModTouchServer::new_from_config().unwrap();
    xmz_mod_touch_server.set_max_wartungsintervall_days(0);
    // drop mutable
    let xmz_mod_touch_server = xmz_mod_touch_server;



    let wartungsintervall = Exception::new(
        vec![
            Box::new(Action::new(&leds, ShiftRegister::set, 2)),
            Box::new(Action::new(&relais, ShiftRegister::clear, 1)),
        ],
    );

    let kabelbruch_zone1_kombisensor1 = Exception::new(
        vec![
            Box::new(Action::new(&leds, ShiftRegister::set, 2)),
            Box::new(Action::new(&relais, ShiftRegister::clear, 1)),
        ],
    );

    let exceptions = vec![wartungsintervall, kabelbruch_zone1_kombisensor1];

}
