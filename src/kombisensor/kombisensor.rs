//! CO-NO2 Kombisensor mit Modbus Transceiver
//!
use exception::Exception;
use shift_register::ShiftRegister;
use std::collections::HashSet;
use kombisensor::Sensor;

/// Ein Kombisensor kann `n` Sensormesszellen enthalten, nomal sind 2 Messzellen (NO2 und CO)
///
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Kombisensor {
    firmware_version: String,
    modbus_address: u8,
    sensors: Vec<Sensor>,
}

impl Kombisensor {
    /// Erzeugt eine neue Kombisensor Instanz
    ///
    /// # Return values
    ///
    /// Diese Funktion liefert eine neue Kombisensor Instanz
    ///
    /// # Parameters
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    ///
    /// let kombisensor = Kombisensor::new();
    /// assert_eq!(kombisensor.get_sensors().len(), 2); // 2 Kombisensoren sind default
    /// ```
    pub fn new() -> Self {
        Kombisensor {
            firmware_version: "0.0.0".to_string(),
            modbus_address: 247,
            // TODO: Remove this two default sensors, if the config generator is working
            sensors: vec![
                Sensor::new(),
                Sensor::new(),
            ],
        }
    }

    /// Check Funktion der Kombisensor Instanz
    ///
    /// Hier werden die Sensoren durchlaufen, und deren `check` Funktion aufgerufen.
    ///
    /// # Parameters
    ///
    /// * `num_zone`    - Nummer der Zone, dieser Wert wird bei der Erstellung der Exception verwendet
    /// * `exceptions`  - mutable Refernenz zum Exceptions Hash Set des Servers. Siehe [`check()`](../struct.Zone.html#method.check) Funktion des Zone Moduls
    /// * `leds`        - mutable Refernenz zum LED's Shift Register des Servers
    /// * `relais`      - mutable Refernenz zum RELAIS's Shift Register des Servers
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    ///
    /// let kombisensor = Kombisensor::new();
    /// assert_eq!(kombisensor.get_sensors().len(), 2); // 2 Kombisensoren sind default
    /// ```
    pub fn check(&mut self,
                 num_zone: usize,
                 exceptions: &mut HashSet<Exception>,
                 leds: &mut ShiftRegister,
                 relais: &mut ShiftRegister) {
        debug!("\t\t\tcheck() Kombisensor ...");
        for (num, mut sensor) in &mut self.sensors.iter_mut().enumerate() {
            sensor.check(num_zone, num, exceptions, leds, relais);
        }
    }

    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // TODO: Write documentation
    /// assert!(false);
    /// ```
    pub fn update(&mut self,
                  num_zone: usize,
                  exceptions: &mut HashSet<Exception>,
                  leds: &mut ShiftRegister,
                  relais: &mut ShiftRegister) {
        debug!("\t\t\tupdate() Kombisensor ...");
        for (num, mut sensor) in &mut self.sensors.iter_mut().enumerate() {
            sensor.update(num_zone, num, exceptions, leds, relais);
        }
    }

    /// Get firmware version
    ///
    /// # Return values
    ///
    /// Liefert die aktuelle Firmware Version die aus dem Sensor ausgelesen wurde,
    /// eine Version "0.0.0" deutet darauf hin das der Kombisensor noch nie über Modbus ausgelesen wurde.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    ///
    /// let kombisensor = Kombisensor::new();
    /// assert_eq!(kombisensor.get_firmware_version(), "0.0.0");
    /// ```
    pub fn get_firmware_version(&self) -> String {
        self.firmware_version.clone()
    }

    /// Set firmware version
    ///
    /// # Parameters
    ///
    /// * `firmware_version`    - String mit der neuen Firmware Version
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    ///
    /// let mut kombisensor = Kombisensor::new();
    /// assert_eq!(kombisensor.get_firmware_version(), "0.0.0");
    /// kombisensor.set_firmware_version("9.9.99".to_string());
    /// assert_eq!(kombisensor.get_firmware_version(), "9.9.99");
    /// ```
    pub fn set_firmware_version(&mut self, firmware_version: String) {
        self.firmware_version = firmware_version
    }

    /// Get modbus_address
    ///
    /// # Return values
    ///
    /// Liefert die aktuelle Firmware Version die aus dem Sensor ausgelesen wurde,
    /// eine Version "0.0.0" deutet darauf hin das der Kombisensor noch nie über Modbus ausgelesen wurde.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    ///
    /// let kombisensor = Kombisensor::new();
    /// assert_eq!(kombisensor.get_modbus_address(), 247);
    /// ```
    pub fn get_modbus_address(&self) -> u8 {
        self.modbus_address
    }

    /// Set modbus_address
    ///
    /// # Parameters
    ///
    /// * `modbus_address`    - u8 mit der neuen Modbus Adresse
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    ///
    /// let mut kombisensor = Kombisensor::new();
    /// assert_eq!(kombisensor.get_modbus_address(), 247);
    /// kombisensor.set_modbus_address(1);
    /// assert_eq!(kombisensor.get_modbus_address(), 1);
    /// ```
    pub fn set_modbus_address(&mut self, modbus_address: u8) {
        self.modbus_address = modbus_address
    }

    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // TODO: Write documentation
    /// assert!(false);
    /// ```
    pub fn get_sensors(&self) -> &Vec<Sensor> {
        &self.sensors
    }

    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // TODO: Write documentation
    /// assert!(false);
    /// ```
    pub fn get_sensors_mut(&mut self) -> &mut Vec<Sensor> {
        &mut self.sensors
    }

    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // TODO: Write documentation
    /// assert!(false);
    /// ```
    pub fn get_sensor(&self, id: usize) -> Option<&Sensor> {
        self.sensors.get(id)
    }

    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // TODO: Write documentation
    /// assert!(false);
    /// ```
    pub fn get_sensor_mut(&mut self, id: usize) -> Option<&mut Sensor> {
        self.sensors.get_mut(id)
    }

    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // TODO: Write documentation
    /// assert!(false);
    /// ```
    pub fn is_online(&self) -> bool {
        false
    }
}

impl Default for Kombisensor {
    fn default() -> Self {
        Self::new()
    }
}
