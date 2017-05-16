use exception::Exception;
use kombisensor::Kombisensor;
use shift_register::ShiftRegister;
use std::collections::HashSet;


///! Eine `Zone` kann `n` [Kombisensoren](kombisensor/struct.Kombisensor.html) enthalten
///!
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Zone {
    kombisensors: Vec<Kombisensor>,
}

impl Zone {
    //! Erstellt eine neue Zone
    //!
    //! # Return values
    //!
    //! Eine neue Zone
    //!
    //! # Examples
    //!
    //! ```rust
    //! use xmz_mod_touch_server::Zone;
    //!
    //! let zone = Zone::new();
    //! assert_eq!(zone.get_kombisensors().len(), 2); // 2 Kombisensoren sind default
    //! ```
    pub fn new() -> Self {
        Zone {
            kombisensors: vec![
                Kombisensor::new(),
                Kombisensor::new(),
            ],
        }
    }

    /// Check Funktion der Zone
    ///
    /// Hier werden die Kombisensoren, der Zone, durchlaufen, und deren `check()` Funktion aufgerufen.
    ///
    /// # Parameters
    ///
    /// * `exceptions`  - mutable Refernenz zum Exceptions Hash Set des Servers. Siehe [`check()`](struct.XMZModTouchServer.html#method.check) Funktion des XMZModTouchServer Moduls
    /// * `leds`        - mutable Refernenz zum LED's Shift Register des Servers
    /// * `relais`      - mutable Refernenz zum RELAIS's Shift Register des Servers
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // TODO: Write documentation
    /// assert!(false);
    /// ```
    pub fn check(&mut self,
                 exceptions: &mut HashSet<Exception>,
                 leds: &mut ShiftRegister,
                 relais: &mut ShiftRegister) {
        debug!("\t\tcheck() Zone ...");
        for (num, mut kombisensor) in &mut self.kombisensors.iter_mut().enumerate() {
            kombisensor.check(num, exceptions, leds, relais);
        }
    }

    /// Update Funktion des Zone
    ///
    /// Hier werden die Zonen durchlaufen, und deren `update()` Funktion aufgerufen.
    ///
    /// # Parameters
    ///
    /// * `exceptions`  - mutable Refernenz zum Exceptions Hash Set des Servers. Siehe [`check()`](struct.XMZModTouchServer.html#method.check) Funktion des XMZModTouchServer Moduls
    /// * `leds`        - mutable Refernenz zum LED's Shift Register des Servers
    /// * `relais`      - mutable Refernenz zum RELAIS's Shift Register des Servers
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn update(&mut self,
                  exceptions: &mut HashSet<Exception>,
                  leds: &mut ShiftRegister,
                  relais: &mut ShiftRegister) {
        debug!("\t\tupdate() Zone ...");
        for (num, mut kombisensor) in &mut self.kombisensors.iter_mut().enumerate() {
            kombisensor.update(num, exceptions, leds, relais);
        }
    }

    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // TODO: Write documentation
    /// assert!(false);
    /// ```
    pub fn get_kombisensors(&self) -> &Vec<Kombisensor> {
        &self.kombisensors
    }

    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // TODO: Write documentation
    /// assert!(false);
    /// ```
    pub fn get_kombisensors_mut(&mut self) -> &mut Vec<Kombisensor> {
        &mut self.kombisensors
    }

    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // TODO: Write documentation
    /// assert!(false);
    /// ```
    pub fn get_kombisensor(&self, id: usize) -> Option<&Kombisensor> {
        self.kombisensors.get(id)
    }

    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // TODO: Write documentation
    /// assert!(false);
    /// ```
    pub fn get_kombisensor_mut(&mut self, id: usize) -> Option<&mut Kombisensor> {
        self.kombisensors.get_mut(id)
    }
}

impl Default for Zone {
    fn default() -> Self {
        Self::new()
    }
}
