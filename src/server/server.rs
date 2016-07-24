use libmodbus_rs::*;
use libmodbus_rs::modbus::{Modbus};
use module::{Module, ModuleType};
use nanomsg::{Socket, Protocol};
use rustc_serialize::json;
use server::error::{Error};
use server::server_command::{ServerCommand};
use server::zone::{Zone, ZoneType};
use shift_register::{ShiftRegister, ShiftRegisterType};
use std::fs;
use std::io::{Read};
use std::io::{Write};
use std::str::FromStr;


pub struct Server {
    pub leds: ShiftRegister,
    pub relais: ShiftRegister,
    // Sensor Module des Servers
    pub modules: Vec<Module>,
    // Alarm/ Störzonen des Servers
    pub zones: Vec<Zone>,
    modbus_device: String,
    pub modbus_baud: i32,
    pub modbus_parity: char,
    pub modbus_data_bit: i32,
    pub modbus_stop_bit: i32,
}

impl Server {
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
            modbus_device: "/dev/ttyS1".to_string(),
            // modbus_device: "/dev/ttyUSB0".to_string(),
            modbus_baud: 9600,
            modbus_parity: 'N',
            modbus_data_bit: 8,
            modbus_stop_bit: 1,
        }
    }

    /// Wichtige Grundeinstellungen, wie das leeren der ShiftRegister Speicher
    pub fn init(&mut self) -> Result<(), Error> {
        self.leds.reset();
        self.relais.reset();

        self.leds.test();

        self.default_configuration();

        // let _device = try!(NanomsgDevice::create());
        Ok(())
    }

    /// Default Konfiguration des Servers
    fn default_configuration(&mut self) {
        self.relais.set(1);
        self.leds.set(1);
        self.leds.set(3);
        self.leds.shift_out();
        self.relais.shift_out();

        self.modules.push(Module::new(ModuleType::RAGAS_CO_NO2));
        self.modules.push(Module::new(ModuleType::RAGAS_CO_NO2));
        self.modules.push(Module::new(ModuleType::RAGAS_CO_NO2));
        self.modules.push(Module::new(ModuleType::RAGAS_CO_NO2));
        self.modules.push(Module::new(ModuleType::RAGAS_CO_NO2));
        self.modules.push(Module::new(ModuleType::RAGAS_CO_NO2));
        let _ = self.modules[0].set_modbus_slave_id(21);
        let _ = self.modules[1].set_modbus_slave_id(22);
        let _ = self.modules[2].set_modbus_slave_id(23);
        let _ = self.modules[3].set_modbus_slave_id(24);
        let _ = self.modules[4].set_modbus_slave_id(25);
        let _ = self.modules[5].set_modbus_slave_id(26);
    }

    // Public api
    //

    /// `get_modbus_device` - Liefert das aktuelle Modbus Device zurück
    ///
    /// # Examples
    /// ```
    /// use xmz_server::server::server::Server;
    ///
    /// let server = Server::new();
    /// assert_eq!(server.get_modbus_device(), "/dev/ttyS1".to_string());
    /// ```
    pub fn get_modbus_device(&self) -> String {
        self.modbus_device.to_string()
    }

    /// `set_modbus_device` - Setzt das Modbus Device
    ///
    /// # Examples
    /// ```
    /// use xmz_server::server::server::Server;
    ///
    /// let mut server = Server::new();
    /// assert_eq!(server.get_modbus_device(), "/dev/ttyS1".to_string());
    /// server.set_modbus_device("/dev/ttyUSB0".to_string());
    /// assert_eq!(server.get_modbus_device(), "/dev/ttyUSB0".to_string());
    /// ```
    /// TODO: -> Result<> Rueckgabewert und Custom Error einpflegen
    pub fn set_modbus_device(&mut self, device: String) {
        self.modbus_device = device;
    }

    /// Sensor Update Task
    ///
    /// Dieser Task checkt zu Begin ob das konfigurierte Modbus Interface `modbus_device` erreichbar ist.
    /// Wenn das Device nicht existiert, oder die Berechtigungen des Users nicht ausreichen wird ein Fehler
    /// ausgegeben.
    ///
    pub fn update_sensors(&mut self) -> Result<(), Error> {
        // Test ob das Serielle Interface existiert und die Berechtigungen für ein Zugriff ausreichen
        try!(fs::metadata(&self.modbus_device));
        // Modbus Kontext erzeugen
        let mut modbus_context = Modbus::new_rtu(self.modbus_device.as_ref(), self.modbus_baud, self.modbus_parity, self.modbus_data_bit, self.modbus_stop_bit);

        for modul in &mut self.modules {
            try!(modbus_context.set_slave(modul.get_modbus_slave_id()));
            // try!(modbus_context.set_debug(true));
            try!(modbus_context.rtu_set_rts(MODBUS_RTU_RTS_DOWN));
            try!(modbus_context.connect());
            let mut tab_reg: Vec<u16> = Vec::new();

            for sensor in &mut modul.sensors {
                tab_reg = modbus_context.read_registers(sensor.modbus_register_address as i32, 1);
                tab_reg.get(0).map(|var| sensor.adc_value = Some(*var));
                modbus_context.close();
            }
        }

        Ok(())
    }

    /// Handle Client Communikation via nanomsg
    ///
    pub fn handle_nanomsg_requests(&mut self) -> Result<(), Error> {
        let mut socket = try!(Socket::new(Protocol::Rep));
        let mut endpoint = try!(socket.bind("ipc:///tmp/xmz-server.ipc"));

        let _ = socket.set_receive_timeout(100);
        let mut request = String::new();

        match socket.read_to_string(&mut request) {
            Ok(_) => {
                let server_command = try!(ServerCommand::from_str(&request));
                let _ = try!(self.execute(server_command, &mut socket));
                request.clear();
            },
            Err(_) => {},
        }
        let _ = endpoint.shutdown();
        drop(socket);

        Ok(())
    }

    /// Führt ein Befehl ausgegeben
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn execute(&mut self, command: ServerCommand, socket: &mut Socket) -> Result<(), Error> {
        match command {
            // LED Befehle
            // led set 1
            // led get 1
            // led clear 1
            // led toggle 1
            ServerCommand::Led { subcommand, params, .. } => {

                match subcommand.as_ref() {
                    "list" => {
                        sende_fehler(socket, "Noch nicht implementiert".to_string());
                    },
                    "test" => {
                        self.leds.test();
                        sende_ok(socket);
                    },
                    "set" => {
                        let num = u64::from_str(&params.unwrap()).unwrap_or(0);
                        self.leds.set(num);
                        self.leds.shift_out();
                        sende_ok(socket);
                    },
                    "get" => {
                        let num = u64::from_str(&params.unwrap()).unwrap_or(0);
                        let result = self.leds.get(num);
                        sende(socket, result.to_string());
                    },
                    "clear" => {
                        let num = u64::from_str(&params.unwrap()).unwrap_or(0);
                        self.leds.clear(num);
                        self.leds.shift_out();
                        sende_ok(socket);
                    },
                    "toggle" => {
                        let num = u64::from_str(&params.unwrap()).unwrap_or(0);
                        self.leds.toggle(num);
                        self.leds.shift_out();
                        sende_ok(socket);
                    },
                    _ => {}
                }
            },
            // RELAIS Befehle
            // relais set 1
            // relais get 1
            // relais clear 1
            // relais toggle 1
            ServerCommand::Relais { subcommand, params, .. } => {

                match subcommand.as_ref() {
                    "list" => {
                        sende_fehler(socket, "Noch nicht implementiert".to_string());
                    },
                    "test" => {
                        self.relais.test();
                        sende_ok(socket);
                    },
                    "set" => {
                        let num = u64::from_str(&params.unwrap()).unwrap_or(0);
                        self.relais.set(num);
                        self.relais.shift_out();
                        sende_ok(socket);
                    },
                    "get" => {
                        let num = u64::from_str(&params.unwrap()).unwrap_or(0);
                        let result = self.relais.get(num);
                        sende(socket, result.to_string());
                    },
                    "clear" => {
                        let num = u64::from_str(&params.unwrap()).unwrap_or(0);
                        self.relais.clear(num);
                        self.relais.shift_out();
                        sende_ok(socket);
                    },
                    "toggle" => {
                        let num = u64::from_str(&params.unwrap()).unwrap_or(0);
                        self.relais.toggle(num);
                        self.relais.shift_out();
                        sende_ok(socket);
                    },
                    _ => {}
                }
            },
            // SERVER Befehle
            ServerCommand::Server { subcommand, config_entry, config_value, .. } => {
                match subcommand.as_ref() {
                    "set" => {
                        match config_entry.as_ref() {
                            "modbus_device" => {
                                // Checke ob das Device existiert
                                // config_value.map(|c| self.set_modbus_device(c); sende_ok(socket));
                                self.set_modbus_device(config_value.unwrap());
                                sende_ok(socket);
                            },
                            _ => {
                                sende_fehler(socket, "Unbekannter Konfigurationswert".to_string());
                            },
                        }
                    },
                    "get" => {
                        match config_entry.as_ref() {
                            "modbus_device" => {
                                let modbus_device = self.get_modbus_device();
                                sende(socket, modbus_device);
                            },
                            _ => {
                                sende_fehler(socket, "Unbekannter Konfigurationswert".to_string());
                            },
                        }
                    },
                    _ => {},
                }
            },
            // MODULE Befehle
            ServerCommand::Module { subcommand, config_entry, config_value, module_num, .. } => {
                match subcommand.as_ref() {
                    "new" => {
                        sende_fehler(socket, "Noch nicht implementiert".to_string());
                    },
                    // Serialized Module und Sensoren
                    "list" => {
                        match json::encode(&self.modules) {
                            Ok(serialized) => {
                                sende(socket, serialized);
                            },
                            Err(err) => {
                                sende_fehler(socket, err.to_string());
                            }
                        }
                    },
                    "show" => {
                        sende_fehler(socket, "Noch nicht implementiert".to_string());
                    },
                    "get" => {
                        sende_fehler(socket, "Noch nicht implementiert".to_string());
                    },
                    "set" => {
                        sende_fehler(socket, "Noch nicht implementiert".to_string());
                    },
                    _ => {},
                }
            }
            // _ => {},
        }
        Ok(())
    }
}

/// Nanomsg Helper Sende String
///
fn sende<T: AsRef<str>>(socket: &mut Socket, msg: T) {
    match socket.write_all(msg.as_ref().as_bytes()) {
        Ok(..) => {
            println!("SENDE: {}", msg.as_ref());
        }
        Err(err) => {
            println!("FEHLER: Konnte Nachricht: {} nicht senden: {}", msg.as_ref(), err);
        }
    }
}

/// Helper sende OK über den Socket
fn sende_ok(socket: &mut Socket) {
    match socket.write_all("OK".as_bytes()) {
        Ok(..) => {
            // println!("OK");
        }
        Err(err) => {
            println!("FEHLER: {}", err);
        }
    }
}

/// Helper sende Fehler und Fehlermeldung über den Socket
fn sende_fehler(socket: &mut Socket, msg: String) {
    match socket.write_all(format!("FEHLER: {}", msg).as_bytes()) {
        Ok(..) => { println!("FEHLER: {}", msg); }
        Err(err) => {
            println!("Konnte FEHLER nicht senden: {}", err);
        }
    }
}




#[cfg(test)]
mod test {
    use server::server::Server;
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
        assert_eq!(server.modules.get(0).unwrap().get_modbus_slave_id(), 1);
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
