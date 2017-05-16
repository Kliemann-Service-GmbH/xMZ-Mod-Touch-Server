use ::chrono::{DateTime, UTC};
use std::collections::HashSet;
use exception::{Exception, ExceptionType};
use shift_register::{ShiftRegister, ShiftRegisterType};
use xmz_mod_touch_server::Zone;


pub const SERVER_MAX_UPTIME_SEC: i64 = 5;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct XMZModTouchServer {
    version: String,
    start_time: DateTime<UTC>,
    pub exceptions: HashSet<Exception>,
    zones: Vec<Zone>,
    pub leds: ShiftRegister,
    pub relais: ShiftRegister,
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
            start_time: UTC::now(),
            exceptions: HashSet::new(),
            zones: vec![
                Zone::new(),
            ],
            leds: ShiftRegister::new(ShiftRegisterType::LED),
            relais: ShiftRegister::new(ShiftRegisterType::Relais),
        }
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


    /// `basic_configuration` - Grundkonfiguration/ Grundeistellungen der LEDs und Relais
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::XMZModTouchServer;
    ///
    /// let mut xmz_mod_touch_server = XMZModTouchServer::new();
    /// xmz_mod_touch_server.basic_configuration();
    /// ```
    pub fn basic_configuration(&mut self) {
        // Grundzustand definieren
        self.leds.reset();
        self.relais.reset();
        // Power LED an
        self.leds.set(1);
        // Relais Störung anziehen (normal closed)
        self.relais.set(1);
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
        debug!("\tcheck() XMZModTouchServer ...");
        self.check_uptime();
        for (_num, mut zone) in &mut self.zones.iter_mut().enumerate() {
            zone.check(&mut self.exceptions, &mut self.leds, &mut self.relais);
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
        debug!("\tupdate() XMZModTouchServer ...");
        for (_num, mut zone) in &mut self.zones.iter_mut().enumerate() {
            zone.update(&mut self.exceptions, &mut self.leds, &mut self.relais);
        }
    }

    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // TODO: Write documentation
    /// assert!(false);
    /// ```
    pub fn get_exceptions(&self) -> &HashSet<Exception> {
        &self.exceptions
    }

    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // TODO: Write documentation
    /// assert!(false);
    /// ```
    pub fn get_exception(&self, _id: usize) -> Option<&Exception> {
        None
    }

    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // TODO: Write documentation
    /// assert!(false);
    /// ```
    pub fn get_zones(&self) -> &Vec<Zone> {
        &self.zones
    }

    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // TODO: Write documentation
    /// assert!(false);
    /// ```
    pub fn get_zones_mut(&mut self) -> &mut Vec<Zone> {
        &mut self.zones
    }

    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // TODO: Write documentation
    /// assert!(false);
    /// ```
    pub fn get_zone(&self, id: usize) -> Option<&Zone> {
        self.zones.get(id)
    }

    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // TODO: Write documentation
    /// assert!(false);
    /// ```
    pub fn get_zone_mut(&mut self, id: usize) -> Option<&mut Zone> {
        self.zones.get_mut(id)
    }

    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // TODO: Write documentation
    /// assert!(false);
    /// ```
    fn check_uptime(&mut self) {
        // Wartungsintervall erreicht?
        if ::chrono::UTC::now().signed_duration_since(self.start_time) >
           ::chrono::Duration::seconds(SERVER_MAX_UPTIME_SEC) {
           self.leds.set(2).ok();
           self.leds.set(3).ok();
           self.relais.clear(1).ok();
           self.add_exception(Exception::new(ExceptionType::WartungsintervalReached));
        }
    }

    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // TODO: Write documentation
    /// assert!(false);
    /// ```
    fn add_exception(&mut self, exception: Exception) {
        if !self.exceptions.contains(&exception) {
            self.exceptions.insert(exception);
        }
    }
}

impl Default for XMZModTouchServer {
    fn default() -> Self {
        Self::new()
    }
}
