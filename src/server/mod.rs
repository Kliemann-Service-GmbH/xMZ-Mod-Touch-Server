//! xMZ-Mod-Touch Server
//!
//! Das ist der zentrale Teil der 'xMZ-Mod-Touch'-Plattform.
mod zone;
use server::zone::{Zone, ZoneType};
use module::Module;
use xmz_shift_register::{ShiftRegister, RegisterType};

pub struct Server {
    leds: ShiftRegister,
    relais: ShiftRegister,
    module: Vec<Module>,
    zonen: Vec<Zone>
}

impl Server {
    pub fn new() -> Self {
        Server {
            leds: ShiftRegister::new(RegisterType::LED),
            relais: ShiftRegister::new(RegisterType::RELAIS),
            module: vec!(),
            zonen: vec![
                Zone::new(ZoneType::Stoerung),
                Zone::new(ZoneType::Schwellenwert),
            ],
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
    use server::Server;
    
    #[test]
    fn server_default_werte() {
        let server = Server::new();

        assert_eq!(server.zonen.len(), 2);
    }
}
