/// Zonen   - Verwaltung der Störungen und Alarme
///
/// Jede Zone hat mindestens ein Alarmpunkt. Jedem dieser Alarmpunkte können Relais und LED zugewiesen werden.
/// Diese werden dann aktiviert/ deaktiviert, je nach Schaltrichtung.
pub mod zone;

use server::zone::{Zone, ZoneType};
use module::Module;
use shift_register::{ShiftRegister, ShiftRegisterType};

pub struct Server<'a> {
    leds: ShiftRegister,
    relais: ShiftRegister,
    module: Vec<Module<'a>>,
    pub zones: Vec<Zone>
}

impl<'a> Server<'a> {
    pub fn new() -> Self {
        Server {
            leds: ShiftRegister::new(ShiftRegisterType::LED),
            relais: ShiftRegister::new(ShiftRegisterType::RELAIS),
            module: vec![],
            zones: vec![
                Zone::new(ZoneType::Stoerung),
                Zone::new(ZoneType::Schwellenwert),
            ],
        }
    }

    /// Default Konfiguration des Servers
    fn default_configuration(&mut self) {
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
    use module::{Module, ModuleType};

    #[test]
    fn default_werte() {
        let server = Server::new();
        assert_eq!(server.zones.len(), 2);
    }

    #[test]
    fn add_one_module() {
        let mut server = Server::new();
        let module = Module::new(ModuleType::RAGAS_CO_NO2);
        server.init();
        assert_eq!(server.module.len(), 0);
        server.module.push(module);
        assert_eq!(server.module.len(), 1);
    }

    #[test]
    fn kann_module_modbus_adresse_abfragen() {
        let mut server = Server::new();
        let module = Module::new(ModuleType::RAGAS_CO_NO2);
        server.init();
        assert_eq!(server.module.len(), 0);
        server.module.push(module);
        assert_eq!(server.module.len(), 1);
    }
}
