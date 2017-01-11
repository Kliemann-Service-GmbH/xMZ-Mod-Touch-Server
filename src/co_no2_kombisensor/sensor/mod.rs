//! Dieses Modul representiert eine Messzelle, eines [CO-NO2-Kombisensor-Mod](https://github.com/Kliemann-Service-GmbH/CO-NO2-Kombisensor-Mod) der Firma RA-GAS
//! `Firmware Version: 0.13.10`
use std::fmt;
// use errors::*;

/// Typ der Messzelle
#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub enum SensorType {
    /// Nemoto NO2 Messzelle, EC NAP-550
    /// Datenblatt: https://www.nemoto.co.jp/nse/sensor-search/nap-550.html?lang=en
    NemotoNO2,
    /// Nemote CO Messzelle, EC NAP-505
    /// Datenblatt: https://www.nemoto.co.jp/nse/sensor-search/use/gas-alarm/nap-505.html?lang=en
    NemotoCO,
    /// Sensor Type für Simmulation eines NO2 Sensors und Testläufe
    SimmulationNO2,
    /// Sensor Type für Simmulation eines CO Sensors und Testläufe
    SimmulationCO,
}

/// SI Einheit des zu messenden Mediums
#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum SI {
    none,
    ppm,
    Vol,
    UEG,
}

/// Representation der Firmware Daten einer Messzelle
#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Sensor {
    number: u16,
    /// ADC Wert    - wird vom Server Prozess über das Modbus Protokoll ausgelesen und aktualisiert
    adc_value: u16,
    min_value: u16,
    max_value: u16,
    adc_value_at_nullgas: u16,
    adc_value_at_messgas: u16,
    concentration_at_nullgas: u32,
    concentration_at_messgas: u32,
    sensor_type: SensorType,
    /// SI Einheit des Sensors (ppm, % UEG, Vol %)
    si: SI,
    config: u16,
    /// Fehlerzähler, zZt. nicht in Firmware vorhanden
    error_count: u32,
}

impl Default for Sensor {
    fn default() -> Self {
        Sensor {
            number: 0,
            adc_value: 0,
            min_value: 0,
            max_value: 0,
            adc_value_at_nullgas: 0,
            adc_value_at_messgas: 0,
            concentration_at_nullgas: 0,
            concentration_at_messgas: 0,
            sensor_type: SensorType::NemotoNO2,
            si: SI::ppm,
            config: 0,
            error_count: 0,
        }
    }
}

impl Sensor {
    /// Erzeugt eine neue Sensor Instanz
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let sensor = Sensor::new();
    /// ```
    pub fn new() -> Self {
        Sensor { ..Default::default() }
    }

    /// Erzeugt eine neue Sensor Instanz, eines bestimmten Sensor Typs
    ///
    /// # Attributes
    /// * `sensor_type`     - `SensorType` Type der Messzelle
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let sensor = Sensor::new_with_type(SensorType::SimmulationNO2);
    /// ```
    pub fn new_with_type(sensor_type: SensorType) -> Self {
        Sensor { sensor_type: sensor_type, ..Default::default() }
    }

    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let mut sensor = Sensor::new();
    /// assert_eq!(sensor.get_adc_value(), 0);
    /// sensor.set_adc_value(100);
    /// assert_eq!(sensor.get_adc_value(), 100);
    /// ```
    pub fn set_adc_value(&mut self, value: u16) {
        self.adc_value = value
    }

    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let sensor = Sensor::new();
    /// ```
    pub fn get_number(&self) -> u16 {
        self.number
    }

    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let sensor = Sensor::new();
    /// ```
    pub fn get_adc_value(&self) -> u16 {
        self.adc_value
    }

    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let sensor = Sensor::new();
    /// ```
    pub fn get_min_value(&self) -> u16 {
        self.min_value
    }

    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let sensor = Sensor::new();
    /// ```
    pub fn get_max_value(&self) -> u16 {
        self.max_value
    }

    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let sensor = Sensor::new();
    /// ```
    pub fn get_adc_value_at_nullgas(&self) -> u16 {
        self.adc_value_at_nullgas
    }

    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let sensor = Sensor::new();
    /// ```
    pub fn get_adc_value_at_messgas(&self) -> u16 {
        self.adc_value_at_messgas
    }

    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let sensor = Sensor::new();
    /// ```
    pub fn get_concentration_at_nullgas(&self) -> u32 {
        self.concentration_at_nullgas
    }

    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let sensor = Sensor::new();
    /// ```
    pub fn get_concentration_at_messgas(&self) -> u32 {
        self.concentration_at_messgas
    }

    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let sensor = Sensor::new();
    /// ```
    pub fn get_sensor_type(&self) -> SensorType {
        self.sensor_type.clone()
    }

    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let sensor = Sensor::new();
    /// ```
    pub fn get_si(&self) -> SI {
        self.si.clone()
    }

    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let sensor = Sensor::new();
    /// ```
    pub fn get_config(&self) -> u16 {
        self.config
    }

    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let sensor = Sensor::new();
    /// ```
    pub fn get_error_count(&self) -> u32 {
        self.error_count
    }

    /// Liefert den berechneten milli Volt Wert
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let sensor = Sensor::new();
    /// ```
    pub fn get_mv(&self) -> u16 {
        (5000 / 1024) * self.adc_value as u16
    }

    /// Berechnet die Gaskonzentration mit einer linearen Funktion
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xmz_server::*;
    ///
    /// let sensor = Sensor::new_with_type(SensorType::SimmulationNO2);
    /// assert_eq!(sensor.get_concentration(), 0.0);
    /// ```
    pub fn get_concentration(&self) -> f64 {
        let adc_value = self.adc_value;
        let adc_value_at_nullgas = self.adc_value_at_nullgas;
        // Damit wir in der Formel nicht durch Null teilen, wird der Wert adc_value_at_messgas auf 1 gesetzt, sollte er Null sein
        let adc_value_at_messgas = if self.adc_value_at_messgas == 0 { 1 } else { self.adc_value_at_messgas };
        let concentration_at_nullgas = self.concentration_at_nullgas;
        let concentration_at_messgas = self.concentration_at_messgas;

        let concentration = (concentration_at_messgas as f64 - concentration_at_nullgas as f64) /
        (adc_value_at_messgas as f64 - adc_value_at_nullgas as f64) *
        (adc_value as f64 - adc_value_at_nullgas as f64) + concentration_at_nullgas as f64;

        // Ist die Konzentration kleiner Null, wird Null ausgegeben, ansonnsten die berechnete Konzentration
        if concentration < 0.0 { 0.0 } else { concentration }
    }

    /// Liefert true oder false je nachdem ob der Sensor aktiv Ist
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::*;
    ///
    /// let sensor = Sensor::new_with_type(SensorType::SimmulationNO2);
    /// assert_eq!(sensor.is_enabled(), false);
    /// ```
    #[allow(dead_code)]
    pub fn is_enabled(&self) -> bool {
        match (self.config >> 0) & 1 {
            0 => false,
            _ => true,
        }
    }
}

/// String Format des Sensor Typen
///
impl fmt::Display for SensorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SensorType::NemotoNO2 => write!(f, "Nemoto™ NO2"),
            SensorType::NemotoCO => write!(f, "Nemoto™ CO"),
            SensorType::SimmulationNO2 => write!(f, "Simmulation NO2"),
            SensorType::SimmulationCO => write!(f, "Simmulation CO"),
        }
    }
}

/// String Format der SI Einheit
///
impl fmt::Display for SI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SI::none => write!(f, ""),
            SI::ppm => write!(f, "ppm"),
            SI::UEG => write!(f, "% UEG"),
            SI::Vol => write!(f, "Vol %"),
        }
    }
}
