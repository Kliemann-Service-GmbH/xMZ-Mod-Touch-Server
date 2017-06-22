//! Zone der XMZModTouchServer Platform
//!
//! Eine Zone kann `n` Kombisensoren enthalten
//!
use server::zone::kombisensor::Kombisensor;


// TODO: Check if Clone is needed, other structs too
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
#[derive(Serialize, Deserialize)]
pub enum ZoneStatus {
    Normal,
    AP1,
    AP2,
    DIW,
}

/// Eine `Zone` kann `n` [Kombisensoren](kombisensor/struct.Kombisensor.html) enthalten
///
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Zone {
    status: ZoneStatus,
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
            status: ZoneStatus::Normal,
            kombisensors: vec![],
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
    /// assert_eq!(zone.get_kombisensors().len(), 0);
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
    /// assert_eq!(zone.get_kombisensors_mut().len(), 0);
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
    /// assert!(zone.get_kombisensor(0).is_none());
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
    /// assert!(zone.get_kombisensor_mut(0).is_none());
    /// ```
    pub fn get_kombisensor_mut(&mut self, id: usize) -> Option<&mut Kombisensor> {
        self.kombisensors.get_mut(id)
    }

    /// Fügt ein gegebenen Kombisensor der Zone hinzu
    ///
    /// # Parameters
    ///
    /// * `kombisensor`   - [`Kombisensor`](struct.Kombisensor.html)
    ///
    /// # Example
    ///
    /// ```rust
    /// use xmz_mod_touch_server::{Kombisensor, Zone};
    /// let mut zone = Zone::new();
    /// assert!(zone.get_kombisensor(0).is_none());
    ///
    /// zone.add_kombisensor(Kombisensor::new());
    /// assert!(zone.get_kombisensor(0).is_some());
    /// ```
    pub fn add_kombisensor(&mut self, kombisensor: Kombisensor) {
        self.kombisensors.push(kombisensor);
    }


    /// Liefert den Status der Zone
    ///
    pub fn get_status(&self) -> ZoneStatus {
        self.status.clone()
    }

    pub fn set_status(&mut self, status: ZoneStatus) {
        // TODO: teste `status` wie in den Struct Buildern
        self.status = status
    }


    // Update Funktion der Zone
    //
    pub fn update(&mut self) {
        // Begin Status Auswertung

        // hightes_state wird erhöht wenn ein Sensor ein erhöhten Messwert liefert.
        // Am Ende der Sensorenauswertung wird der Zonene Status auf den hightest_status
        // gesetzt. Ist kein Sensor auffällig, dann ist es einfach ZoneStatus::Normal
        let mut hightes_state = ZoneStatus::Normal;

        // duchlaufe alle Kombisensoren der Zone
        for kombisensor in self.get_kombisensors() {
            match kombisensor.get_sensors().iter().find(|&s| s.alarmpunkt1_reached() == true) {
                Some(_) => {
                    if hightes_state < ZoneStatus::AP1 { hightes_state = ZoneStatus::AP1; }
                }
                None => {}
            }
            match kombisensor.get_sensors().iter().find(|&s| s.alarmpunkt2_reached() == true) {
                Some(_) => {
                    if hightes_state < ZoneStatus::AP2 { hightes_state = ZoneStatus::AP2; }
                }
                None => {}
            }
            match kombisensor.get_sensors().iter().find(|&s| s.direct_value_reached() == true) {
                Some(_) => {
                    if hightes_state < ZoneStatus::DIW { hightes_state = ZoneStatus::DIW; }
                }
                None => {}
            }

            // Alle Sensoren durchlaufen
            // ...
        }
        // Alle Kombisensoren durchlaufen
        // ...
        self.status = hightes_state;
    }
}


impl Default for Zone {
    fn default() -> Self {
        Self::new()
    }
}
