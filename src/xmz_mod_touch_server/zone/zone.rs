//! Zone der XMZModTouchServer Platform
//!
//! Eine Zone kann `n` Kombisensoren enthalten
//! 
use kombisensor::Kombisensor;


/// Eine `Zone` kann `n` [Kombisensoren](kombisensor/struct.Kombisensor.html) enthalten
///
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Zone {
    kombisensors: Vec<Kombisensor>,
}

impl Zone {
    /// Erstellt eine neue Zone
    ///
    /// # Return values
    ///
    /// Eine neue Zone
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Zone;
    ///
    /// let zone = Zone::new();
    /// ```
    pub fn new() -> Self {
        Zone {
            kombisensors: vec![
                Kombisensor::new(),
                Kombisensor::new(),
            ],
        }
    }

    /// Liefert eine Refernz auf einen Vector der die Kombisensoren der Zone entält
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Zone;
    ///
    /// let zone = Zone::new();
    /// assert_eq!(zone.get_kombisensors().len(), 2); // 2 Kombisensoren sind default
    /// ```
    pub fn get_kombisensors(&self) -> &Vec<Kombisensor> {
        &self.kombisensors
    }

    /// Liefert eine mutable Refernz auf einen Vector der die Kombisensoren der Zone entält
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Zone;
    ///
    /// let mut zone = Zone::new();
    /// assert_eq!(zone.get_kombisensors_mut().len(), 2); // 2 Kombisensoren sind default
    /// ```
    pub fn get_kombisensors_mut(&mut self) -> &mut Vec<Kombisensor> {
        &mut self.kombisensors
    }

    /// Finde Kombisensor by Id
    ///
    /// # Return values
    ///
    /// Liefert eine Referenz auf einen Kombisensor, wenn er vorhanden ist oder `None`, wenn der Kombisensor nicht Teil der Zone ist.
    ///
    /// # Parameters
    ///
    /// * `id`  - Id im Vector Kombisensoren (`kombisensors`) der Zone
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Zone;
    ///
    /// let zone = Zone::new();
    /// assert!(zone.get_kombisensor(0).is_some());
    /// ```
    pub fn get_kombisensor(&self, id: usize) -> Option<&Kombisensor> {
        self.kombisensors.get(id)
    }

    /// Finde mutablen Kombisensor by Id
    ///
    /// # Return values
    ///
    /// Liefert eine mutable Referenz auf einen Kombisensor, wenn er vorhanden ist oder `None`,
    /// wenn der Kombisensor nicht Teil der Zone ist.
    ///
    /// # Parameters
    ///
    /// * `id`  - Id im Vector Kombisensoren (`kombisensors`) der Zone
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Zone;
    ///
    /// let mut zone = Zone::new();
    /// assert!(zone.get_kombisensor_mut(0).is_some());
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
