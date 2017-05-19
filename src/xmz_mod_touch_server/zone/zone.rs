use kombisensor::Kombisensor;


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
