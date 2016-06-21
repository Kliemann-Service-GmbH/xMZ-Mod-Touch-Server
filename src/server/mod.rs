/// Zonen   - Verwaltung der Störungen und Alarme
///
/// Jede Zone hat mindestens ein Alarmpunkt. Jedem dieser Alarmpunkte können Relais und LED zugewiesen werden.
/// Diese werden dann aktiviert/ deaktiviert, je nach Schaltrichtung.
pub mod zone;

use libmodbus_rs::*;
use libmodbus_rs::modbus::{Modbus};
use server::zone::{Zone, ZoneType};
use shift_register::{ShiftRegister, ShiftRegisterType};
use module::{Module, ModuleType};

pub struct Server<'a> {
    leds: ShiftRegister,
    relais: ShiftRegister,
    // Sensor Module des Servers
    pub modules: Vec<Module<'a>>,
    // Alarm/ Störzonen des Servers
    pub zones: Vec<Zone>,
    pub modbus_device: &'a str,
    pub modbus_baud: i32,
    pub modbus_parity: char,
    pub modbus_data_bit: i32,
    pub modbus_stop_bit: i32,
}

impl<'a> Server<'a> {
    /// Erzeugt eine neue Server Instanz
    ///
    pub fn new() -> Self {
        Server {
            leds: ShiftRegister::new(ShiftRegisterType::LED),
            relais: ShiftRegister::new(ShiftRegisterType::RELAIS),
            modules: vec![],
            zones: vec![
                Zone::new(ZoneType::Stoerung),
                Zone::new(ZoneType::Schwellenwert),
            ],
            modbus_device: "/dev/ttyS1",
            modbus_baud: 9600,
            modbus_parity: 'N',
            modbus_data_bit: 8,
            modbus_stop_bit: 1,
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
        self.modules.push(Module::new(ModuleType::RAGAS_CO_NO2));
        self.modules[0].modbus_address = 46;
        self.modbus_device = "/dev/ttyUSB0";
    }

    pub fn init(&mut self) {
        #[cfg(target_arch = "arm")]
        {
            self.leds.init();
            self.relais.init();
        }
        // Rufe die default Konfiguration auf
        self.default_configuration();
    }

    pub fn update_sensors(&mut self) {
        // Modbus Kontext erzeugen
        let mut modbus_context = Modbus::new_rtu(self.modbus_device, self.modbus_baud, self.modbus_parity, self.modbus_data_bit, self.modbus_stop_bit);

        for modul in &mut self.modules {
            let modbus_address = modul.modbus_address;
            //try!(modbus_context.set_slave(modul.modbus_address).map_err(|e| e.to_string()));
            let _ = modbus_context.set_slave(modul.modbus_address);
            let _ = modbus_context.set_debug(false);
            let _ = modbus_context.rtu_set_rts(MODBUS_RTU_RTS_DOWN);

            match modbus_context.connect() {
                Err(_) => { modbus_context.free(); }
                Ok(_) => {
                    for sensor in &mut modul.sensors {
                        let tab_reg = modbus_context.read_registers(sensor.modbus_register_address as i32, 1);
                        sensor.adc_value = Some(tab_reg[0]);
                    }
                }
            }
        }
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
        let modul = Module::new(ModuleType::RAGAS_CO_NO2);
        assert_eq!(server.modules.len(), 0);
        server.modules.push(modul);
        assert_eq!(server.modules.len(), 1);
    }

    #[test]
    fn kann_module_modbus_adresse_abfragen() {
        let mut server = Server::new();
        let module = Module::new(ModuleType::RAGAS_CO_NO2);
        server.modules.push(module);
        assert_eq!(server.modules.get(0).unwrap().modbus_address, 1);
    }

    #[test]
    fn kann_module_sensor1_modbus_register_address_abfragen() {
        let mut server = Server::new();
        let module = Module::new(ModuleType::RAGAS_CO_NO2);
        server.modules.push(module);
        assert_eq!(server.modules.get(0).unwrap().sensors.get(0).unwrap().modbus_register_address, 1);
        // Der zweite Sensor des Ersten Moduls (CO) hat die Modbus Register Adress
        assert_eq!(server.modules.get(0).unwrap().sensors.get(1).unwrap().modbus_register_address, 11);
    }
}
