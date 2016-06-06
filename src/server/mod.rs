//! xMZ-Mod-Touch Server
//!
//! Das ist der zentrale Teil der 'xMZ-Mod-Touch'-Plattform.
mod zone;

use module::Module;
use xmz_shift_register::{ShiftRegister, RegisterType};

pub struct Server {
    leds: ShiftRegister,
    relais: ShiftRegister,
    modules: Vec<Module>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            leds: ShiftRegister::new(RegisterType::LED),
            relais: ShiftRegister::new(RegisterType::RELAIS),
            modules: vec!()
        }
    }

    /// Default Konfiguration des Servers
    pub fn default_configuration(&mut self) {
        self.relais.set(1);
        self.leds.set(1);
        self.leds.set(3);
        #[cfg(target_arch = "arm")]
        {
            self.leds.shift_out();
            self.relais.init();
        }
    }

    pub fn init(&mut self) {
        #[cfg(target_arch = "arm")]
        {
            self.leds.init();
            self.relais.init();
        }

        self.default_configuration();
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn basics() {
        assert!(true);
    }
}
