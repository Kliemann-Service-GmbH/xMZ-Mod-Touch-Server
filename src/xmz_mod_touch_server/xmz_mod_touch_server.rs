//! Serverteil der xMZ-Mod-Touch-Server Platform
//!
//! Hier werden alle Komponenten des Servers verwaltet.
//!
use chrono;
use chrono::prelude::*;
use configuration::Configuration;
use errors::*;
use exception::{Exception, ExceptionType};
use serde_json;
use shift_register::{ShiftRegister, ShiftRegisterType};
use std::collections::HashSet;
use xmz_mod_touch_server::Zone;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;


pub const SERVER_MAX_UPTIME_SEC: i64 = 5;

/// Der XMZModTouchServer kann `n` [Zonen](struct.Zone.html) enthalten
///
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct XMZModTouchServer {
    version: String,
    // `create_time` wird nur ein mal beim erstellen der Konfiguration gesetzt
    create_time: chrono::DateTime<UTC>,
    // Wird jedes mal wenn der Serverprozess gestartet wurde, gesetzt
    start_time: chrono::DateTime<UTC>,
    // Ausnahmen
    pub exceptions: HashSet<Exception>,
    zones: Vec<Zone>,
    leds: Mutex<ShiftRegister>,
    relais: Mutex<ShiftRegister>,
}

impl XMZModTouchServer {
    /// Erzeugt eine neue XMZModTouchServer Instanz
    ///
    /// # Return values
    ///
    /// Diese Funktion liefert eine neue XMZModTouchServer Instanz
    ///
    /// # Parameters
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::XMZModTouchServer;
    ///
    /// let xmz_mod_touch_server = XMZModTouchServer::new();
    /// assert_eq!(xmz_mod_touch_server.get_version(), env!("CARGO_PKG_VERSION").to_string());
    /// ```
    pub fn new() -> Self {
        XMZModTouchServer {
            version: env!("CARGO_PKG_VERSION").to_string(),
            create_time: chrono::UTC::now(),
            start_time: chrono::UTC::now(),
            exceptions: HashSet::new(),
            zones: vec![],
            leds: Mutex::new(ShiftRegister::new(ShiftRegisterType::LED)),
            relais: Mutex::new(ShiftRegister::new(ShiftRegisterType::Relais)),
        }
    }

    /// Serverinstanz aus Konfigurationsdatei erstellen
    ///
    /// # Return values
    ///
    /// Diese Funktion liefert ein Result. Das Result enthält die Server Instanz, oder ein Error,
    /// wenn die Konfiguration nicht ausgelesen werden konnte.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xmz_mod_touch_server::XMZModTouchServer;
    ///
    /// let xmz_mod_touch_server = XMZModTouchServer::new_from_config();
    /// ```
    pub fn new_from_config() -> Result<XMZModTouchServer> {
        let mut xmz_mod_touch_server: XMZModTouchServer = match serde_json::from_str(&Configuration::get_config()?) {
            Ok(xmz_mod_touch_server) => xmz_mod_touch_server,
            _ => panic!("Konnte Konfigurationsdatei nicht lesen. Server konnte nicht erstellt werden."),
        };

        // Update start_time to now
        xmz_mod_touch_server.reset_start_time();

        Ok(xmz_mod_touch_server)
    }


    /// Check Funktion des XMZModTouchServer
    ///
    /// Hier werden die Zonen durchlaufen, und deren `check()` Funktion aufgerufen.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::XMZModTouchServer;
    ///
    /// let mut xmz_mod_touch_server = XMZModTouchServer::new();
    /// xmz_mod_touch_server.check();
    /// ```
    pub fn check(&mut self) {
        debug!("\tCheck XMZModTouchServer ...");
        for (num_zone, zone) in self.get_zones().iter().enumerate() {
            // debug!("\t\Check Zone {} ...", num_zone);
            for (num_kombisensor, kombisensor) in zone.get_kombisensors().iter().enumerate() {
                // debug!("\t\t\Check Kombisensor {} ...", num_kombisensor);
                for (num_sensor, sensor) in kombisensor.get_sensors().iter().enumerate() {
                    // debug!("\t\t\t\Check Sensor {} ...", num_sensor);
                    // Begin checks sensor ...
                    if sensor.get_concentration() >= sensor.alarm3_direct_value as f64 {
                        println!("sensor.get_concentration() {} >= sensor.alarm3_direct_value {}", sensor.get_concentration(), sensor.alarm3_direct_value);
                        if let Ok(mut leds) = self.leds.lock() {
                            leds.set(5);
                            leds.set(6);
                            leds.set(7);
                        }
                        if let Ok(mut relais) = self.relais.lock() {
                            relais.set(2);
                            relais.set(3);
                            relais.set(4);
                        }
                    }
                }
            }
        }
    }

    /// Update Funktion des XMZModTouchServer
    ///
    /// Hier werden die Zonen durchlaufen, und deren `update()` Funktion aufgerufen.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::XMZModTouchServer;
    ///
    /// let mut xmz_mod_touch_server = XMZModTouchServer::new();
    /// xmz_mod_touch_server.update();
    /// ```
    pub fn update(&mut self) {
        debug!("\tUpdate XMZModTouchServer ...");
        for (num_zone, mut zone) in &mut self.get_zones_mut().iter_mut().enumerate() {
            debug!("\t\tUpdate Zone {} ...", num_zone);
            for (num_kombisensor, mut kombisensor) in &mut zone.get_kombisensors_mut().iter_mut().enumerate() {
                debug!("\t\t\tUpdate Kombisensor {} via Modbus ...", num_kombisensor);
                match kombisensor.get_from_modbus() {
                    Err(e) => {
                        // println!("Zone: {} Kombisensor: {} Error: {:?}", &num_zone, &num_kombisensor, e);
                        // self.add_exception(Exception::new(ExceptionType::KombisensorModbusError{ num_zone, num_kombisensor }));
                    }
                    _ => {
                    }
                }

                for (num_sensor, mut sensor) in &mut kombisensor.get_sensors_mut().iter_mut().enumerate() {
                    // Aktualisiert den Tuppel Vector. In dem Tuppel werden die Daten der letzten 15Minuten gehalten,
                    // mit diesen wird der Mittelwert gebildet
                    debug!("\t\t\tUpdate Sensor {} average ...", num_sensor);
                    sensor.update_adc_values_average();
                }
            }
        }
    }

    /// `basic_configuration` - Grundkonfiguration/ Grundeistellungen der LEDs und Relais
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xmz_mod_touch_server::XMZModTouchServer;
    ///
    /// let mut xmz_mod_touch_server = XMZModTouchServer::new();
    /// xmz_mod_touch_server.basic_configuration().unwrap();
    /// ```
    pub fn basic_configuration(&mut self) -> Result<()> {
        loop {
            if let Ok(mut leds) = self.leds.lock() {
                debug!("Basic configuration server LEDs");
                leds.reset();
                // Power LED an
                leds.set(1)?;
                break;
            }
        }

        loop {
            if let Ok(mut relais) = self.relais.lock() {
                debug!("Basic configuration server RELAIS");
                relais.reset();
                // Relais Störung anziehen (normal closed)
                relais.set(1)?;
                break;
            }
        }

        Ok(())
    }

    /// Liefert die Versionsnummer des XMZModTouchServer's
    ///
    /// Die Versionsnummer entspricht der Crate Versionsnummer, wird aus dieser automatisch gebildet.
    ///
    /// # Return values
    ///
    /// Diese Funktion liefert eine neue XMZModTouchServer Instanz
    ///
    /// # Parameters
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::XMZModTouchServer;
    ///
    /// let xmz_mod_touch_server = XMZModTouchServer::new();
    /// assert_eq!(xmz_mod_touch_server.get_version(), env!("CARGO_PKG_VERSION").to_string());
    /// ```
    pub fn get_version(&self) -> String {
        self.version.clone()
    }

    /// Liefert eine Refernz auf die Exception des Servers
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::XMZModTouchServer;
    ///
    /// let mut xmz_mod_touch_server = XMZModTouchServer::new();
    /// assert_eq!(xmz_mod_touch_server.get_exceptions().len(), 0);
    /// ```
    pub fn get_exceptions(&self) -> &HashSet<Exception> {
        &self.exceptions
    }

    /// Finde eine Exception
    ///
    /// # Return values
    ///
    /// Liefert ein Option Type der entweder eine Refernz auf die Exception des Servers oder `None` enthält,
    /// wenn eine Exception mit dieser Id nicht existiert
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use xmz_mod_touch_server::XMZModTouchServer;
    /// let mut xmz_mod_touch_server = XMZModTouchServer::new();
    /// xmz_mod_touch_server.add_exception(Exception::new(ExceptionType::WartungsintervalReached));
    ///
    /// assert!(xmz_mod_touch_server.get_exception(0).is_some());
    /// ```
    pub fn get_exception(&self, id: usize) -> Option<&Exception> {
        unimplemented!()
    }

    /// Fügt eine Exception hinzu
    ///
    /// # Parameters
    ///
    /// * `exception`   - Exception die hinzugefügt werden soll
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::{XMZModTouchServer, Exception, ExceptionType};
    ///
    /// let mut xmz_mod_touch_server = XMZModTouchServer::new();
    /// assert_eq!(xmz_mod_touch_server.get_exceptions().len(), 0);
    ///
    /// xmz_mod_touch_server.add_exception(Exception::new(ExceptionType::WartungsintervalReached));
    /// assert_eq!(xmz_mod_touch_server.get_exceptions().len(), 1);
    ///
    /// // if the exception is alreddy present, dont insert again
    /// xmz_mod_touch_server.add_exception(Exception::new(ExceptionType::WartungsintervalReached));
    /// assert_eq!(xmz_mod_touch_server.get_exceptions().len(), 1);
    /// ```
    pub fn add_exception(&mut self, exception: Exception) {
        if !self.exceptions.contains(&exception) {
            self.exceptions.insert(exception);
        }
    }


    /// Zonen des Servers
    ///
    /// # Return values
    ///
    /// Liefert eine Refernz auf die Zonen des Servers
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::XMZModTouchServer;
    ///
    /// let xmz_mod_touch_server = XMZModTouchServer::new();
    /// assert_eq!(xmz_mod_touch_server.get_zones().len(), 0); // Eine Zone default
    /// ```
    pub fn get_zones(&self) -> &Vec<Zone> {
        &self.zones
    }

    /// Mutable Refernz auf die Zonen des Servers
    ///
    /// # Return values
    ///
    /// Liefert eine mutable Refernz auf die Zonen des Servers
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::XMZModTouchServer;
    ///
    /// let mut xmz_mod_touch_server = XMZModTouchServer::new();
    /// assert_eq!(xmz_mod_touch_server.get_zones_mut().len(), 0); // Eine Zone default
    /// ```
    pub fn get_zones_mut(&mut self) -> &mut Vec<Zone> {
        &mut self.zones
    }

    /// Finde Zone
    ///
    /// # Return values
    ///
    /// Liefert ein `Option` Typen, der eine Refernz auf die gesucht Zone oder `None` enthält
    ///
    /// # Parameters
    ///
    /// * `id`  - Id der gesuchten Zone
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::XMZModTouchServer;
    ///
    /// let xmz_mod_touch_server = XMZModTouchServer::new();
    /// assert!(xmz_mod_touch_server.get_zone(0).is_none());
    /// ```
    pub fn get_zone(&self, id: usize) -> Option<&Zone> {
        self.zones.get(id)
    }

    /// Finde mut Referenz auf Zone
    ///
    /// # Return values
    ///
    /// Liefert ein `Option` Typen, der eine mutable Refernz auf die gesucht Zone oder `None` enthält
    ///
    /// # Parameters
    ///
    /// * `id`  - Id der gesuchten Zone
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::XMZModTouchServer;
    ///
    /// let mut xmz_mod_touch_server = XMZModTouchServer::new();
    /// assert!(xmz_mod_touch_server.get_zone_mut(0).is_none());
    /// ```
    pub fn get_zone_mut(&mut self, id: usize) -> Option<&mut Zone> {
        self.zones.get_mut(id)
    }

    /// Erzeugt eine neu Zone
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::XMZModTouchServer;
    /// let mut xmz_mod_touch_server = XMZModTouchServer::new();
    /// assert!(xmz_mod_touch_server.get_zone(0).is_none());
    ///
    /// xmz_mod_touch_server.add_zone();
    /// assert!(xmz_mod_touch_server.get_zone(0).is_some());
    /// ```
    pub fn add_zone(&mut self) {
        self.zones.push(Zone::new());
    }

    /// Atomic Reference Counted Mutex einer Referenz auf die LED's ShiftRegister
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::XMZModTouchServer;
    ///
    /// let xmz_mod_touch_server = XMZModTouchServer::new();
    /// let server_leds = xmz_mod_touch_server.get_leds().lock().unwrap();
    /// assert_eq!(server_leds.data, 0);
    /// ```
    pub fn get_leds(&self) -> &Mutex<ShiftRegister> {
        &self.leds
    }

    /// Atomic Reference Counted Mutex einer Mutable Referenz auf die LED's ShiftRegister
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::XMZModTouchServer;
    ///
    /// let mut xmz_mod_touch_server = XMZModTouchServer::new();
    /// let mut server_leds = xmz_mod_touch_server.get_leds().lock().unwrap();
    /// assert_eq!(server_leds.data, 0);
    /// ```
    pub fn get_leds_mut(&mut self) -> &mut Mutex<ShiftRegister> {
        &mut self.leds
    }

    /// Atomic Reference Counted Mutex einer Referenz auf die Relais ShiftRegister
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::XMZModTouchServer;
    ///
    /// let xmz_mod_touch_server = XMZModTouchServer::new();
    /// let server_relais = xmz_mod_touch_server.get_relais().lock().unwrap();
    /// assert_eq!(server_relais.data, 0);
    /// ```
    pub fn get_relais(&self) -> &Mutex<ShiftRegister> {
        &self.relais
    }

    /// Atomic Reference Counted Mutex einer Mutable Referenz auf die Relais ShiftRegister
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::XMZModTouchServer;
    ///
    /// let mut xmz_mod_touch_server = XMZModTouchServer::new();
    /// let mut server_relais = xmz_mod_touch_server.get_relais().lock().unwrap();
    /// assert_eq!(server_relais.data, 0);
    /// ```
    pub fn get_relais_mut(&mut self) -> &mut Mutex<ShiftRegister> {
        &mut self.relais
    }


    /// Uptime des Servers
    ///
    /// # Return values
    ///
    /// * `uptime`  - Die Uptime des Servers
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::XMZModTouchServer;
    ///
    /// let xmz_mod_touch_server = XMZModTouchServer::new();
    /// ::std::thread::sleep(::std::time::Duration::from_millis(10));
    /// assert!(xmz_mod_touch_server.uptime().num_milliseconds() >= 10);
    /// ```
    pub fn uptime(&self) -> chrono::Duration {
        // Wartungsintervall erreicht?
        chrono::UTC::now().signed_duration_since(self.start_time)
    }


    // Macht was sie meint
    //
    // Nachdem die Konfiguration mit `XMZModTouchServer::new_from_config()` wieder eingelesen wurde
    // muss der `start_time` Member auf die aktuelle Systemzeit gesetzt werden.
    //
    fn reset_start_time(&mut self) {
        self.start_time = UTC::now();
    }

}

impl Default for XMZModTouchServer {
    fn default() -> Self {
        Self::new()
    }
}
