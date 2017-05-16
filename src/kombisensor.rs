//! CO-NO2 Kombisensor mit Modbus Transceiver
//!
use exception::Exception;
use sensor::Sensor;
use shift_register::ShiftRegister;
use std::collections::HashSet;


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Kombisensor {
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
