//! Serverteil der xMZ-Mod-Touch-Server Platform
//!
//! Hier werden alle Komponenten des Servers verwaltet.
//!
use chrono;
use chrono::prelude::*;
use server::configuration::Configuration;
use errors::*;
use exception::{Exception, ExceptionType};
use serde_json;
use shift_register::{ShiftRegister, ShiftRegisterType};
use std::cell::RefCell;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use server::zone::{Zone, ZoneStatus};
use server::zone::kombisensor::{Kombisensor, KombisensorStatus};

/// Der Server kann `n` [Zonen](struct.Zone.html) enthalten
///
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Server {
    version: String,
    // `create_time` wird nur ein mal beim erstellen der Konfiguration gesetzt
    create_time: chrono::DateTime<Utc>,
    // Wird jedes mal wenn der Serverprozess gestartet wurde, gesetzt
    start_time: chrono::DateTime<Utc>,
    // Anzahl Tage der max. Laufzeit. Wird diese Anzahl erreicht wird der Wartungsintervall Alarm ausgelöst
    wartungsintervall_days: i64,
    // Ausnahmen
    exceptions: Mutex<HashSet<Exception>>,
    zones: Vec<Zone>,
    leds: ShiftRegister,
    relais: ShiftRegister,
}

impl Server {
    /// Erzeugt eine neue Server Instanz
    ///
    /// # Return values
    ///
    /// Diese Funktion liefert eine neue Server Instanz
    ///
    /// # Parameters
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Server;
    ///
    /// let xmz_mod_touch_server = Server::new();
    /// assert_eq!(xmz_mod_touch_server.get_version(), env!("CARGO_PKG_VERSION").to_string());
    /// ```
    pub fn new() -> Self {
        Server {
            version: env!("CARGO_PKG_VERSION").to_string(),
            create_time: chrono::Utc::now(),
            start_time: chrono::Utc::now(),
            wartungsintervall_days: 365,
            exceptions: Mutex::new(HashSet::new()),
            zones: vec![],
            leds: ShiftRegister::new(ShiftRegisterType::LED),
            relais: ShiftRegister::new(ShiftRegisterType::Relais),
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
    /// use xmz_mod_touch_server::Server;
    ///
    /// let xmz_mod_touch_server = Server::new_from_config();
    /// ```
    pub fn new_from_config() -> Result<Server> {
        let mut xmz_mod_touch_server: Server = match serde_json::from_str(&Configuration::get_config()?) {
            Ok(xmz_mod_touch_server) => xmz_mod_touch_server,
            _ => panic!("Konnte Konfigurationsdatei nicht lesen. Server konnte nicht erstellt werden."),
        };

        // Update start_time to now
        xmz_mod_touch_server.reset_start_time();

        Ok(xmz_mod_touch_server)
    }


    /// Check Funktion des Server
    ///
    /// Hier werden die Zonen durchlaufen, und deren `check()` Funktion aufgerufen.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Server;
    ///
    /// let xmz_mod_touch_server = Server::new();
    /// xmz_mod_touch_server.check();
    /// ```
    pub fn check(&self) {
        debug!("Check Server ...");
        if self.wartungsintervall_reached() {
            self.leds.set(2); self.leds.set(3);
        } else {
            self.leds.clear(2); self.leds.clear(3);
        }

        for (num_zone, zone) in self.get_zones().iter().enumerate() {
            debug!("\tCheck Zone {} ...", num_zone);
            match zone.get_status() {
                ZoneStatus::DIW => {
                    self.leds.set(5); self.leds.set(6); self.leds.set(7);
                    self.relais.set(2); self.relais.set(3); self.relais.set(4);
                }
                ZoneStatus::AP2 => {
                    self.leds.set(5); self.leds.set(6); self.leds.clear(7);
                    self.relais.set(2); self.relais.set(3); self.relais.clear(4);
                }
                ZoneStatus::AP1 => {
                    self.leds.set(5); self.leds.clear(6); self.leds.clear(7);
                    self.relais.set(2); self.relais.clear(3); self.relais.clear(4);
                }
                _ => {}
            }

            for (num_kombisensor, kombisensor) in zone.get_kombisensors().iter().enumerate() {
                debug!("\t\tCheck Kombisensor {} ...", num_kombisensor);
                match kombisensor.get_status() {
                    KombisensorStatus::Kabelbruch => {
                        self.leds.set(2);
                    }
                    _ => {
                        self.leds.clear(2);
                    }
                }

                for (num_sensor, sensor) in kombisensor.get_sensors().iter().enumerate() {
                    debug!("\t\t\tCheck Sensor {} ...", num_sensor);
                }
            }
        }
    }

    /// Update Funktion des Server
    ///
    /// Hier werden die Zonen durchlaufen, und deren `update()` Funktion aufgerufen.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Server;
    ///
    /// let mut xmz_mod_touch_server = Server::new();
    /// xmz_mod_touch_server.update();
    /// ```
    pub fn update(&mut self) {
        debug!("Check Server ...");
        for (num_zone, mut zone) in &mut self.get_zones_mut().iter_mut().enumerate() {
            debug!("\tCheck Zone {} ...", num_zone);
            zone.update();

            for (num_kombisensor, mut kombisensor) in &mut zone.get_kombisensors_mut().iter_mut().enumerate() {
                debug!("\t\tCheck Kombisensor {} ...", num_kombisensor);
                kombisensor.update();

                for (num_sensor, mut sensor) in &mut kombisensor.get_sensors_mut().iter_mut().enumerate() {
                    debug!("\t\t\tCheck Sensor {} ...", num_sensor);
                    sensor.update();
                }
            }
        }
    }

    /// `basic_configuration` - Grundkonfiguration/ Grundeistellungen der LEDs und Relais
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xmz_mod_touch_server::Server;
    ///
    /// let mut xmz_mod_touch_server = Server::new();
    /// xmz_mod_touch_server.basic_configuration().unwrap();
    /// ```
    pub fn basic_configuration(&mut self) -> Result<()> {
        self.leds.reset()?;
        // Power LED an
        self.leds.set(1)?;

        self.relais.reset()?;
        // Relais Störung anziehen (normal closed)
        self.relais.set(1)?;

        Ok(())
    }

    /// Liefert die Versionsnummer des Server's
    ///
    /// Die Versionsnummer entspricht der Crate Versionsnummer, wird aus dieser automatisch gebildet.
    ///
    /// # Return values
    ///
    /// Diese Funktion liefert eine neue Server Instanz
    ///
    /// # Parameters
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Server;
    ///
    /// let xmz_mod_touch_server = Server::new();
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
    /// use xmz_mod_touch_server::Server;
    ///
    /// let mut xmz_mod_touch_server = Server::new();
    /// xmz_mod_touch_server.get_exceptions();
    /// ```
    pub fn get_exceptions(&self) -> &Mutex<HashSet<Exception>> {
        &self.exceptions
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
    /// use xmz_mod_touch_server::Server;
    ///
    /// let xmz_mod_touch_server = Server::new();
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
    /// use xmz_mod_touch_server::Server;
    ///
    /// let mut xmz_mod_touch_server = Server::new();
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
    /// use xmz_mod_touch_server::Server;
    ///
    /// let xmz_mod_touch_server = Server::new();
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
    /// use xmz_mod_touch_server::Server;
    ///
    /// let mut xmz_mod_touch_server = Server::new();
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
    /// use xmz_mod_touch_server::Server;
    /// let mut xmz_mod_touch_server = Server::new();
    /// assert!(xmz_mod_touch_server.get_zone(0).is_none());
    ///
    /// xmz_mod_touch_server.add_zone();
    /// assert!(xmz_mod_touch_server.get_zone(0).is_some());
    /// ```
    pub fn add_zone(&mut self) {
        self.zones.push(Zone::new());
    }

    /// Liefert Maximal Tage bis Wartungsintervall Alarm ausgelöst wird
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Server;
    /// let xmz_mod_touch_server = Server::new();
    ///
    /// assert_eq!(xmz_mod_touch_server.get_max_wartungsintervall_days(), 365);
    /// ```
    pub fn get_max_wartungsintervall_days(&self) -> i64 {
        self.wartungsintervall_days
    }

    /// Setzt die Maximal Tage bis Wartungsintervall Alarm ausgelöst wird
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Server;
    /// let mut xmz_mod_touch_server = Server::new();
    /// assert_eq!(xmz_mod_touch_server.get_max_wartungsintervall_days(), 365);
    ///
    /// xmz_mod_touch_server.set_max_wartungsintervall_days(0);
    /// assert_eq!(xmz_mod_touch_server.get_max_wartungsintervall_days(), 0);
    /// ```
    pub fn set_max_wartungsintervall_days(&mut self, interval: i64) {
        self.wartungsintervall_days = interval;
    }

    /// Uptime des Servers
    ///
    /// Wieviel Zeit ist seit dem letzten Neustart des Servers vergangen. **Bitte nicht mit der [`runtime`](struct.Server.html#method.runtime) des Servers verwechseln!**
    ///
    /// # Return values
    ///
    /// * `uptime`  - Die Uptime des Servers
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Server;
    ///
    /// let xmz_mod_touch_server = Server::new();
    /// ::std::thread::sleep(::std::time::Duration::from_millis(10));
    /// assert!(xmz_mod_touch_server.uptime().num_milliseconds() >= 10);
    /// ```
    pub fn uptime(&self) -> chrono::Duration {
        chrono::Utc::now().signed_duration_since(self.start_time)
    }

    /// Runtime des Servers
    ///
    /// Komplette Laufzeit des Servers. Diese Funktion wertet den `create_time` Member aus. Dieser Zeitstempel wird bei der
    /// Erstellung der Konfigurationsdatei, oder bei einer Wartung auf das aktuelle Datum gesetzt.
    ///
    /// # Return values
    ///
    /// * `uptime`  - Die Uptime des Servers
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Server;
    /// let xmz_mod_touch_server = Server::new();
    ///
    /// let runtime = xmz_mod_touch_server.runtime();
    /// ```
    pub fn runtime(&self) -> chrono::Duration {
        chrono::Utc::now().signed_duration_since(self.create_time)
    }

    // Prüfung Wartungsintervall erreicht
    //
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Server;
    /// let mut xmz_mod_touch_server = Server::new();
    /// assert_eq!(xmz_mod_touch_server.wartungsintervall_reached(), false);
    ///
    /// xmz_mod_touch_server.set_max_wartungsintervall_days(0);
    /// assert_eq!(xmz_mod_touch_server.wartungsintervall_reached(), true);
    /// ```
    pub fn wartungsintervall_reached(&self) -> bool {
        self.runtime().num_days() >= self.get_max_wartungsintervall_days()
    }

    // Macht was sie meint
    //
    // Nachdem die Konfiguration mit `Server::new_from_config()` wieder eingelesen wurde
    // muss der `start_time` Member auf die aktuelle Systemzeit gesetzt werden.
    //
    fn reset_start_time(&mut self) {
        self.start_time = Utc::now();
    }

}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}
