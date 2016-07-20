use libmodbus_rs::*;
use libmodbus_rs::modbus::{Modbus};
use module::{Module, ModuleType};
use nanomsg_device::NanomsgDevice;
use nanomsg::{Socket, Protocol};
use server::server_command::{ServerCommand};
use server::server_error::{ServerError};
use server::zone::{Zone, ZoneType};
use shift_register::{ShiftRegister, ShiftRegisterType};
use std::fs;
use std::io::{Read};
use std::io::{Write};
use std::str::FromStr;


pub struct Server<'a> {
    pub leds: ShiftRegister,
    pub relais: ShiftRegister,
    // Sensor Module des Servers
    pub modules: Vec<Module<'a>>,
    // Alarm/ Störzonen des Servers
    pub zones: Vec<Zone>,
    modbus_device: String,
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
            modbus_device: "/dev/ttyS1".to_string(),
            // modbus_device: "/dev/ttyUSB0".to_string(),
            modbus_baud: 9600,
            modbus_parity: 'N',
            modbus_data_bit: 8,
            modbus_stop_bit: 1,
        }
    }

    /// Wichtige Grundeinstellungen, wie das leeren der ShiftRegister Speicher
    pub fn init(&mut self) -> Result<(), ServerError> {
        self.leds.reset();
        self.relais.reset();

        self.leds.test();

        self.default_configuration();

        let _device = try!(NanomsgDevice::create());
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

    pub fn handle_nanomsg_requests(&mut self) -> Result<(), ServerError> {
        let mut socket = try!(Socket::new(Protocol::Rep));
        let _ = socket.set_send_timeout(1000);
        let mut endpoint = try!(socket.connect("ipc:///tmp/xmz-server.ipc"));
        let mut request = String::new();
        let _ = try!(socket.read_to_string(&mut request));
        let server_command = try!(ServerCommand::from_str(&request));
        let _ = try!(self.execute(server_command, &mut socket));
        request.clear();
        let _ = endpoint.shutdown();

        Ok(())
    }

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
    pub fn update_sensors(&mut self) {
        match fs::metadata(&self.modbus_device){
            Ok(_) => {
                // Modbus Kontext erzeugen
                let mut modbus_context = Modbus::new_rtu(self.modbus_device.as_ref(), self.modbus_baud, self.modbus_parity, self.modbus_data_bit, self.modbus_stop_bit);
                for modul in &mut self.modules {
                    // Modbus Slave ID festlegen
                    match modbus_context.set_slave(modul.get_modbus_slave_id()) {
                        Ok(_) => {
                            // let _ = modbus_context.set_debug(true);
                            match modbus_context.rtu_set_rts(MODBUS_RTU_RTS_DOWN) {
                                Ok(_) => {
                                    let mut tab_reg: Vec<u16> = Vec::new();

                                    for sensor in &mut modul.sensors {
                                        match modbus_context.connect() {
                                            Ok(_) => {
                                                tab_reg = modbus_context.read_registers(sensor.modbus_register_address as i32, 1);
                                                tab_reg.get(0).map(|var| sensor.adc_value = Some(*var));
                                                modbus_context.close();
                                            }
                                            Err(err) => {
                                                println!("Modbus Connect ist fehlgeschlagen: {}", err);
                                            }
                                        }
                                    }
                                }
                                Err(err) => { println!("Konnte MODBUS_RTU_RTS_DOWN nicht setzen: {}", err); }
                            }
                        }
                        Err(err) => { println!("Modbus Context konnte nicht erzeugt werden: {}", err); }
                    }
                };
                modbus_context.free();
            },
            Err(err) => { println!("Modbus Device: '{}' ist nicht verfügbar: {}", self.modbus_device, err); }
        }
    }

    /// Führt ein Befehl ausgegeben
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn execute(&mut self, command: ServerCommand, socket: &mut Socket) -> Result<(), ServerError> {
        match command {
            // LED Befehle
            ServerCommand::Led { subcommand, params, .. } => {
                match subcommand.as_ref() {
                    "set" => {
                        self.leds.set(u64::from_str(&params).unwrap());
                        self.leds.shift_out();
                        sende_ok(socket);
                    },
                    "get" => {
                        self.leds.get(u64::from_str(&params).unwrap());
                        sende_ok(socket);
                    },
                    "clear" => {
                        self.leds.clear(u64::from_str(&params).unwrap());
                        self.leds.shift_out();
                        sende_ok(socket);
                    },
                    "toggle" => {
                        self.leds.toggle(u64::from_str(&params).unwrap());
                        self.leds.shift_out();
                        sende_ok(socket);
                    },
                    _ => {}
                }
            },
            // RELAIS Befehle
            ServerCommand::Relais { subcommand, params, .. } => {
                match subcommand.as_ref() {
                    "set" => {
                        self.relais.set(u64::from_str(&params).unwrap());
                        self.relais.shift_out();
                        sende_ok(socket);
                    },
                    "get" => {
                        self.relais.get(u64::from_str(&params).unwrap());
                        sende_ok(socket);
                    },
                    "clear" => {
                        self.relais.clear(u64::from_str(&params).unwrap());
                        self.relais.shift_out();
                        sende_ok(socket);
                    },
                    "toggle" => {
                        self.relais.toggle(u64::from_str(&params).unwrap());
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
                        let modbus_device = self.get_modbus_device();
                        sende(socket, modbus_device);
                    },
                    _ => {},
                }
            },
            // MODULE Befehle
            ServerCommand::Module { subcommand, config_entry, config_value, module_num, .. } => {
                match subcommand.as_ref() {
                    "set" => {
                        // 3. Parameter `config_entry`
                        match config_entry {
                            Some(config_entry) => {
                                match config_entry.as_ref() {
                                    "modbus_slave_id" => {
                                        // 4. Parameter `config_value`
                                        match config_value {
                                            Some(config_value) => {
                                                match i32::from_str(config_value.as_ref()) {
                                                    Ok(config_value) => {
                                                        // 5. Parameter `module_num`
                                                        module_num.map(|num| {
                                                            println!("Neue Modulnummer {}", num);
                                                            sende(socket, format!("Module: {} neu ID: {}", num, config_value));
                                                        });
                                                        //     Some(module_num) => {
                                                        //         match i32::from_str(module_num.as_ref()) {
                                                        //             Ok(module_num) => {
                                                        //
                                                        //                 // self.modules.get(module_num as usize).map(|mut module| {
                                                        //                 //     match module.set_modbus_slave_id(config_value) {
                                                        //                 //         Ok(_) => {}
                                                        //                 //         Err(err) => { sende_fehler(socket, format!("'{}' ist keine gültige Modbus Slave ID: {}", config_value, err)) }
                                                        //                 //     }
                                                        //                 // });
                                                        //
                                                        //             }
                                                        //             None => { sende_fehler(socket, format!("Module '{}' nicht vorhanden", module_num)) }
                                                        //         },
                                                        //         Err(err) => { sende_fehler(socket, format!("'{}' ist keine 32Bit Integer: {}", config_value, err)); }
                                                        //     }
                                                        //     None => {},
                                                        // }
                                                    },
                                                    Err(err) => { sende_fehler(socket, format!("'{}' ist keine 32Bit Integer: {}", config_value, err)); }
                                                }
                                            },
                                            None => {},
                                        }
                                    },
                                    _ => { sende_fehler(socket, format!("Unbekannter Konfigurationswert: {}", config_entry)) }
                                }
                            },
                            None => {},
                        }
                    },
                    "list" => {},
                    "add" => {},
                    "delete" => {},
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
fn sende(socket: &mut Socket, msg: String) {
    match socket.write_all(msg.as_bytes()) {
        Ok(..) => { println!("SENDE: {}", msg); }
        Err(err) => { println!("FEHLER: Konnte folgende Nachricht nicht senden: {}", msg); }
    }
}

/// Helper sende OK über den Socket
fn sende_ok(socket: &mut Socket) {
    match socket.write_all("OK".as_bytes()) {
        Ok(..) => { println!("OK"); }
        Err(err) => {
            println!("FEHLER: Konnte nicht OK senden");
        }
    }
}

/// Helper sende Fehler und Fehlermeldung über den Socket
fn sende_fehler(socket: &mut Socket, msg: String) {
    match socket.write_all(format!("FEHLER: {}", msg).as_bytes()) {
        Ok(..) => { println!("FEHLER: {}", msg); }
        Err(err) => {
            println!("Konnte FEHLER nicht senden");
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
