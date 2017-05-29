//! Sensormesszelle für CO oder NO2 Gas
//!
//! Dieses Modul representiert eine Messzelle, eines [CO-NO2-Kombisensor-Mod](https://github.com/Kliemann-Service-GmbH/CO-NO2-Kombisensor-Mod) der Firma RA-GAS
//! `Firmware Version: 0.14.0`
//!
use ::chrono::{DateTime, UTC};
use exception::{Exception, ExceptionType};
use shift_register::ShiftRegister;
use std::collections::HashSet;
use std::fmt;


// Nur Messwerte der letzten 15Minuten behalten
// Die Konstante wird in Sekunden angegeben
pub const AVERAGE_15MIN_SEC: i64 = 15 * 60;
// pub const AVERAGE_15MIN_SEC: i64 = 10;

/// Typ der Messzelle
#[derive(Clone)]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize, Debug)]
pub enum SensorType {
    /// Nemoto NO2 Messzelle, EC NAP-550
    /// Datenblatt: https://www.nemoto.co.jp/nse/sensor-search/nap-550.html?lang=en
    NemotoNO2,
    /// Nemote CO Messzelle, EC NAP-505
    /// Datenblatt: https://www.nemoto.co.jp/nse/sensor-search/use/gas-alarm/nap-505.html?lang=en
    NemotoCO,
    /// Simulation, mit festen Werten, einer NO2 Messzelle
    SimulationNO2Fix,
    /// Simulation, mit festen Werten, einer CO Messzelle
    SimulationCOFix,
    /// Simulation einer NO2 Messzelle
    SimulationNO2,
    /// Simulation einer CO Messzelle
    SimulationCO,
}

/// SI Einheit des zu messenden Mediums
#[derive(Clone)]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum SI {
    none,
    ppm,
    Vol,
    UEG,
}

/// Representation der Firmware Daten einer Messzelle
///
/// Firmware Version: 0.14.0
#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Sensor {
    /// ADC Wert    - wird vom Server Prozess über das Modbus Protokoll ausgelesen und aktualisiert
    adc_value: u16,
    min_value: u16,
    max_value: u16,
    adc_value_at_nullgas: u16,
    adc_value_at_messgas: u16,
    concentration_at_nullgas: u16,
    concentration_at_messgas: u16,
    // Typ der Messzelle
    pub sensor_type: SensorType,
    /// SI Einheit des Sensors (ppm, % UEG, Vol %)
    si: SI,
    config: u16,
    /// Fehlerzähler, zZt. nicht in Firmware vorhanden
    error_count: u64,
    /// 15min Average
    adc_value_average_15min: f64,
    pub alarm1_average_15min: u32,
    pub alarm2_average_15min: u32,
    pub alarm3_direct_value: u32,
    #[serde(skip_deserializing, skip_serializing)]
    adc_values_average: Vec<(u16, DateTime<UTC>)>,
}

impl Sensor {
    /// Erzeugt eine neue Sensor Instanz
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::Sensor;
    ///
    /// let sensor = Sensor::new();
    /// ```
    pub fn new() -> Self {
        Sensor {
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
            adc_value_average_15min: 0.0,
            alarm1_average_15min: 0,
            alarm2_average_15min: 0,
            alarm3_direct_value: 0,
            adc_values_average: vec![],
        }
    }

    /// Erzeugt eine neue Sensor Instanz vom gegebenen Typ
    ///
    /// # Parameters
    ///
    /// * `sensor_type` - Sensor Typ
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::{Sensor, SensorType};
    ///
    /// let sensor_sim_no2_fix = Sensor::new_with_type(SensorType::SimulationNO2Fix);
    /// sensor_sim_no2_fix.get_concentration();
    /// ```
    pub fn new_with_type(sensor_type: SensorType) -> Self {
        match sensor_type {
            SensorType::NemotoNO2 => {
                Sensor {
                    max_value: 30,
                    adc_value_at_nullgas: 920,
                    adc_value_at_messgas: 564,
                    concentration_at_messgas: 20,       // 20ppm Messgas
                    alarm1_average_15min: 3, // laut DIN EN 50545-1 Alarm1 (15min Mittelwert) bei 3ppm für NO2
                    alarm2_average_15min: 6, // laut DIN EN 50545-1 Alarm2 (15min Mittelwert) bei 6ppm für NO2
                    alarm3_direct_value: 15, // laut DIN EN 50545-1 Alarm3 (Direktwert) bei 15ppm für NO2
                    sensor_type: sensor_type,
                    ..Default::default() }
            }
            SensorType::SimulationNO2Fix => {
                Sensor {
                    adc_value: 564,
                    max_value: 30,
                    adc_value_at_nullgas: 920,
                    adc_value_at_messgas: 564,
                    concentration_at_messgas: 20,       // 20ppm Messgas
                    alarm1_average_15min: 3, // laut DIN EN 50545-1 Alarm1 (15min Mittelwert) bei 3ppm für NO2
                    alarm2_average_15min: 6, // laut DIN EN 50545-1 Alarm2 (15min Mittelwert) bei 6ppm für NO2
                    alarm3_direct_value: 15, // laut DIN EN 50545-1 Alarm3 (Direktwert) bei 15ppm für NO2
                    sensor_type: sensor_type,
                    ..Default::default() }
            }
            SensorType::SimulationNO2 => {
                Sensor {
                    max_value: 30,
                    adc_value_at_nullgas: 920,
                    adc_value_at_messgas: 564,
                    concentration_at_messgas: 20,       // 20ppm Messgas
                    alarm1_average_15min: 3, // laut DIN EN 50545-1 Alarm1 (15min Mittelwert) bei 3ppm für NO2
                    alarm2_average_15min: 6, // laut DIN EN 50545-1 Alarm2 (15min Mittelwert) bei 6ppm für NO2
                    alarm3_direct_value: 15, // laut DIN EN 50545-1 Alarm3 (Direktwert) bei 15ppm für NO2
                    sensor_type: sensor_type,
                    ..Default::default() }
            }
            SensorType::NemotoCO => {
                Sensor {
                    max_value: 300,
                    adc_value_at_nullgas: 112,
                    adc_value_at_messgas: 760,
                    concentration_at_messgas: 270,       // 280ppm Messgas
                    alarm1_average_15min: 30, // laut DIN EN 50545-1 Alarm1 (15min Mittelwert) bei 3ppm für CO
                    alarm2_average_15min: 60, // laut DIN EN 50545-1 Alarm2 (15min Mittelwert) bei 6ppm für CO
                    alarm3_direct_value: 150, // laut DIN EN 50545-1 Alarm3 (Direktwert) bei 15ppm für CO
                    sensor_type: sensor_type,
                    ..Default::default() }
            }
            SensorType::SimulationCOFix => {
                Sensor {
                    adc_value: 760,
                    max_value: 300,
                    adc_value_at_nullgas: 112,
                    adc_value_at_messgas: 760,
                    concentration_at_messgas: 270,       // 280ppm Messgas
                    alarm1_average_15min: 30, // laut DIN EN 50545-1 Alarm1 (15min Mittelwert) bei 3ppm für CO
                    alarm2_average_15min: 60, // laut DIN EN 50545-1 Alarm2 (15min Mittelwert) bei 6ppm für CO
                    alarm3_direct_value: 150, // laut DIN EN 50545-1 Alarm3 (Direktwert) bei 15ppm für CO
                    sensor_type: sensor_type,
                    ..Default::default() }
            }
            SensorType::SimulationCO => {
                Sensor {
                    max_value: 300,
                    adc_value_at_nullgas: 112,
                    adc_value_at_messgas: 760,
                    concentration_at_messgas: 270,       // 280ppm Messgas
                    alarm1_average_15min: 30, // laut DIN EN 50545-1 Alarm1 (15min Mittelwert) bei 3ppm für CO
                    alarm2_average_15min: 60, // laut DIN EN 50545-1 Alarm2 (15min Mittelwert) bei 6ppm für CO
                    alarm3_direct_value: 150, // laut DIN EN 50545-1 Alarm3 (Direktwert) bei 15ppm für CO
                    sensor_type: sensor_type,
                    ..Default::default() }
            }
            _ => {
                Sensor { sensor_type: sensor_type, ..Default::default() }
            }
        }
    }

    // Public Attributes

    /// Liefert den aktuell ausgelsenen ADC Wert zurück
    ///
    /// Ist der Wert 0 dann deutete das darauf hin das der Sensor noch nicht über Modbus ausgelesen wurde.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::{Sensor, SensorType};
    /// let sensor = Sensor::new();
    ///
    /// assert_eq!(sensor.get_adc_value(), 0);
    /// ```
    pub fn get_adc_value(&self) -> u16 {
        self.adc_value
    }

    /// Liefert den minimal Wert der für den Sensor konfiguriert wurde
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::Sensor;
    /// let sensor = Sensor::new();
    ///
    /// assert_eq!(sensor.get_min_value(), 0);
    /// ```
    pub fn get_min_value(&self) -> u16 {
        self.min_value
    }

    /// Liefert den maximal Wert der für den Sensor konfiguriert wurde
    ///
    /// Ist der Wert 0 dann deutete das darauf hin das der Sensor noch nicht über Modbus ausgelesen wurde.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::Sensor;
    /// let sensor = Sensor::new();
    ///
    /// assert_eq!(sensor.get_max_value(), 0);
    /// ```
    pub fn get_max_value(&self) -> u16 {
        self.max_value
    }

    /// Liefert den ADC Wert der bei der Kalibration, mit Nullgas, im Sensor gespeichert wurde
    ///
    /// Ist der Wert 0 dann deutete das darauf hin das der Sensor noch nicht über Modbus ausgelesen wurde.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::Sensor;
    /// let sensor = Sensor::new();
    ///
    /// assert_eq!(sensor.get_adc_value_at_nullgas(), 0);
    /// ```
    pub fn get_adc_value_at_nullgas(&self) -> u16 {
        self.adc_value_at_nullgas
    }

    /// Liefert den ADC Wert der bei der Kalibration, mit Messgas, im Sensor gespeichert wurde
    ///
    /// Ist der Wert 0 dann deutete das darauf hin das der Sensor noch nicht über Modbus ausgelesen wurde.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::Sensor;
    /// let sensor = Sensor::new();
    ///
    /// assert_eq!(sensor.get_adc_value_at_messgas(), 0);
    /// ```
    pub fn get_adc_value_at_messgas(&self) -> u16 {
        self.adc_value_at_messgas
    }

    /// Liefert die Konzentration des Gases, die bei der Kalibration, mit Nullgas, im Sensor gespeichert wurde
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::Sensor;
    /// let sensor = Sensor::new();
    ///
    /// assert_eq!(sensor.get_concentration_at_nullgas(), 0);
    /// ```
    pub fn get_concentration_at_nullgas(&self) -> u16 {
        self.concentration_at_nullgas
    }

    /// Liefert die Konzentration des Gases, die bei der Kalibration, mit Messgas, im Sensor gespeichert wurde
    ///
    /// Ist der Wert 0 dann deutete das darauf hin das der Sensor noch nicht über Modbus ausgelesen wurde.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::Sensor;
    /// let sensor = Sensor::new();
    ///
    /// assert_eq!(sensor.get_concentration_at_messgas(), 0);
    /// ```
    pub fn get_concentration_at_messgas(&self) -> u16 {
        self.concentration_at_messgas
    }

    /// Liefert Config Register des Sensors
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::{Sensor, SensorType};
    /// let sensor = Sensor::new_with_type(SensorType::SimulationNO2);
    ///
    /// assert_eq!(sensor.get_config(), 0);
    /// ```
    pub fn get_config(&self) -> u16 {
        self.config
    }

    /// Liefert den Typen des Sensors
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::{Sensor, SensorType};
    /// let sensor = Sensor::new();
    ///
    /// assert_eq!(sensor.get_sensor_type(), SensorType::NemotoNO2);
    /// ```
    pub fn get_sensor_type(&self) -> SensorType {
        self.sensor_type.clone()
    }

    /// Liefert die SI Einheit des Sensors
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::{Sensor, SI};
    /// let sensor = Sensor::new();
    ///
    /// assert_eq!(sensor.get_si(), SI::ppm);
    /// ```
    pub fn get_si(&self) -> SI {
        self.si.clone()
    }



    /// Liefert den Stand des Fehlerzählers
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::Sensor;
    /// let sensor = Sensor::new();
    ///
    /// assert_eq!(sensor.get_error_count(), 0);
    /// ```
    pub fn get_error_count(&self) -> u64 {
        self.error_count
    }

    /// Inkrementiert den Fehlerzähler des Sensors
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::Sensor;
    /// let mut sensor = Sensor::new();
    ///
    /// assert_eq!(sensor.get_error_count(), 0);
    /// sensor.inc_error_count();
    /// assert_eq!(sensor.get_error_count(), 1);
    /// ```
    pub fn inc_error_count(&mut self) {
        self.error_count += 1;
    }

    /// Setzt den Fehlerzähler des Sensors wieder auf Nullgas
    ///
    ///  # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::Sensor;
    ///
    /// let mut sensor = Sensor::new();
    /// assert_eq!(sensor.get_error_count(), 0);
    /// sensor.inc_error_count();
    /// assert_eq!(sensor.get_error_count(), 1);
    ///
    /// sensor.reset_error_count();
    /// assert_eq!(sensor.get_error_count(), 0);
    /// ```
    pub fn reset_error_count(&mut self) {
        self.error_count = 0;
    }

    /// Direktwert überschritten?
    ///
    /// Liefert ein boolen `true` wenn der konfigurierte Direktwert überschritten wurden
    ///
    ///
    ///  # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::{Sensor, SensorType};
    /// let sensor = Sensor::new_with_type(SensorType::SimulationNO2Fix);
    ///
    /// assert_eq!(sensor.direct_value_reached(), true)
    /// ```
    pub fn direct_value_reached(&self) -> bool {
        self.get_concentration() >= self.alarm3_direct_value as f64
    }

    /// Direktwert
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::{Sensor, SensorType};
    ///
    /// let sensor = Sensor::new_with_type(SensorType::SimulationNO2Fix);
    /// sensor.get_concentration();
    /// ```
    pub fn get_concentration(&self) -> f64 {
        self.concentration_from(self.adc_value as f64)
    }

    /// Mittelwert 15 Minuten
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::{Sensor, SensorType};
    ///
    /// let sensor = Sensor::new_with_type(SensorType::SimulationNO2Fix);
    /// sensor.get_concentration_average_15min();
    /// ```
    pub fn get_concentration_average_15min(&self) -> f64 {
        self.concentration_from(self.adc_value_average_15min)
    }


    /// Berechnet die Gaskonzentration mit einer linearen Funktion
    ///
    /// Diese Funktion ist eine Helper Funktion. Sie wird von `get_concentration()` und `get_concentration_average_15min()`
    /// verwendet.
    ///
    fn concentration_from(&self, adc_value: f64) -> f64 {
        // adc_value_at_messgas wird für den NO2 speziell behandelt
        // Damit wir in der Formel nicht durch Null teilen, wird der Wert adc_value_at_messgas auf 1 gesetzt, sollte er Null sein
        let adc_value_at_messgas = if self.adc_value_at_messgas == 0 { 1 } else { self.adc_value_at_messgas };

        let concentration = (self.concentration_at_messgas as f64 - self.concentration_at_nullgas as f64) /
        (adc_value_at_messgas as f64 - self.adc_value_at_nullgas as f64) *
        (adc_value as f64 - self.adc_value_at_nullgas as f64) + self.concentration_at_nullgas as f64;

        // Ist die Konzentration kleiner Null, wird Null ausgegeben, ansonnsten die berechnete Konzentration
        if concentration < 0.0 { 0.0 } else { concentration }
    }


    /// Liefert den berechneten milli Volt Wert
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::{Sensor, SensorType};
    ///
    /// let sensor = Sensor::new_with_type(SensorType::SimulationNO2Fix);
    /// sensor.get_mv();
    /// ```
    pub fn get_mv(&self) -> u16 {
        (5000 / 1024) * self.adc_value as u16
    }

    /// Liefert true oder false je nachdem ob der Sensor aktiv ist
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::{Sensor, SensorType};
    ///
    /// let sensor = Sensor::new_with_type(SensorType::SimulationNO2);
    /// assert_eq!(sensor.is_enabled(), false);
    /// ```
    pub fn is_enabled(&self) -> bool {
        match (self.config >> 0) & 1 {
            0 => false,
            _ => true,
        }
    }

    /// Indirekter check ob der Sensor "online" ist
    ///
    /// Das heist diese Funktion soll prüfen ob der Sensor wenigstens ein mal erfolgreich über
    /// das Bus System ausgelesen worden ist.
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::{Sensor, SensorType};
    ///
    /// let sensor = Sensor::new_with_type(SensorType::SimulationNO2);
    /// assert_eq!(sensor.is_online(), false);
    /// ```
    pub fn is_online(&self) -> bool {
        self.adc_value > 0 && self.adc_values_average.len() > 0
    }



    // Setter

    /// Setzt den ADC Wert manuell von Hand
    ///
    /// Diese Funktion sollte nur zu Test oder Simulationszwecken nötig sein
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::{Sensor, SensorType};
    /// let mut sensor_sim_no2_fix = Sensor::new_with_type(SensorType::SimulationNO2Fix);
    ///
    /// sensor_sim_no2_fix.set_adc_value(386);
    /// assert_eq!(sensor_sim_no2_fix.get_concentration(), 30.0);
    /// ```
    pub fn set_adc_value(&mut self, adc_value: u16) {
        self.adc_value = adc_value;
    }

    /// Setzt den minimal Wert der für den Sensor konfiguriert wurde
    ///
    /// # Parameters
    ///
    /// * `min_value`   - Minimal Wert
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::Sensor;
    /// let mut sensor = Sensor::new();
    ///
    /// sensor.set_min_value(1);
    /// assert_eq!(sensor.get_min_value(), 1);
    /// ```
    pub fn set_min_value(&mut self, min_value: u16) {
        self.min_value = min_value;
    }

    /// Setzt den maximal Wert der für den Sensor konfiguriert wurde
    ///
    /// # Parameters
    ///
    /// * `max_value`   - Maximal Wert
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::Sensor;
    /// let mut sensor = Sensor::new();
    ///
    /// sensor.set_max_value(100);
    /// assert_eq!(sensor.get_max_value(), 100);
    /// ```
    pub fn set_max_value(&mut self, max_value: u16) {
        self.max_value = max_value;
    }

    /// Setzt den ADC Wert der bei der Kalibration, mit Nullgas, im Sensor gespeichert wurde
    ///
    /// # Parameters
    ///
    /// * `adc_value_at_nullgas`    - ADC Wert bie der Nullgas Kalibration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::Sensor;
    /// let mut sensor = Sensor::new();
    ///
    /// sensor.set_adc_value_at_nullgas(100);
    /// assert_eq!(sensor.get_adc_value_at_nullgas(), 100);
    /// ```
    pub fn set_adc_value_at_nullgas(&mut self, adc_value_at_nullgas: u16) {
        self.adc_value_at_nullgas = adc_value_at_nullgas;
    }

    /// Setzt den ADC Wert der bei der Kalibration, mit Messgas, im Sensor gespeichert wurde
    ///
    /// # Parameters
    ///
    /// * `adc_value_at_messgas`    - ADC Wert bie der Messgas Kalibration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::Sensor;
    /// let mut sensor = Sensor::new();
    ///
    /// sensor.set_adc_value_at_messgas(100);
    /// assert_eq!(sensor.get_adc_value_at_messgas(), 100);
    /// ```
    pub fn set_adc_value_at_messgas(&mut self, adc_value_at_messgas: u16) {
        self.adc_value_at_messgas = adc_value_at_messgas;
    }

    /// Setzt die Konzentration des Gases, die bei der Kalibration, mit Nullgas, im Sensor gespeichert wurde
    ///
    /// # Parameters
    ///
    /// * `concentration_at_nullgas`    - Konzentration bei Nullgas
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::Sensor;
    /// let mut sensor = Sensor::new();
    ///
    /// sensor.set_concentration_at_nullgas(100);
    /// assert_eq!(sensor.get_concentration_at_nullgas(), 100);
    /// ```
    pub fn set_concentration_at_nullgas(&mut self, concentration_at_nullgas: u16) {
        self.concentration_at_nullgas = concentration_at_nullgas;
    }

    /// Setzt die Konzentration des Gases, die bei der Kalibration, mit Messgas, im Sensor gespeichert wurde
    ///
    /// # Parameters
    ///
    /// * `concentration_at_messgas`    - Konzentration bei Messgas
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::Sensor;
    /// let mut sensor = Sensor::new();
    ///
    /// sensor.set_concentration_at_messgas(100);
    /// assert_eq!(sensor.get_concentration_at_messgas(), 100);
    /// ```
    pub fn set_concentration_at_messgas(&mut self, concentration_at_messgas: u16) {
        self.concentration_at_messgas = concentration_at_messgas;
    }

    /// Setzt Config Register des Sensors
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::kombisensor::{Sensor, SensorType};
    /// let mut sensor = Sensor::new_with_type(SensorType::SimulationNO2);
    ///
    /// sensor.set_config(1);
    /// assert_eq!(sensor.get_config(), 1);
    /// ```
    pub fn set_config(&mut self, config: u16) {
        self.config = config;
    }

    /// Berechnet den Mittelwert
    pub fn update_adc_values_average(&mut self) {
        // Nur wenn die Messzelle aktiv ist wird der Mittelwert berechnet
        if !self.is_enabled() { return; }

        // Update tuppel with the current (adc_value, timestamp)
        self.adc_values_average.push((self.adc_value, UTC::now()));

        //  [`position()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.position)
        // Searches for an element in an iterator, returning its index. We use the index then to [`split_off()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.split_off)
        //
        if let Some(index) = self.adc_values_average.iter().position(|&(_, timestamp)| UTC::now().signed_duration_since(timestamp).num_seconds() < AVERAGE_15MIN_SEC ) {
            // Mit `split_off()` kann man nun den Vector teilen, es bleiben nur noch die (Messerte, Zeitstempel) der letzten AVERAGE_15MIN_SEC übrig.
            // **Dieser Rest wird nun wieder als adc_values_average übernommen, alle anderen Werte werden verworfen.**
            //
            self.adc_values_average = self.adc_values_average.split_off(index);
        }

        // // DEBUG
        // for (num, adc_values_average) in self.adc_values_average.clone().iter().enumerate() {
        //     println!("{:?}, {:?}", num, adc_values_average);
        // };

        let num_adc_values_average = self.adc_values_average.len();
        debug!("num adc_values_average: {}", num_adc_values_average);

        let mut sum_adc_values_average: u64 = 0;
        for &(value, _) in self.adc_values_average.iter(){
            sum_adc_values_average += value as u64;
        }

        self.adc_value_average_15min = sum_adc_values_average as f64 / num_adc_values_average as f64;
    }

}

impl fmt::Display for SensorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SensorType::NemotoNO2 => write!(f, "Nemoto™ NO2"),
            SensorType::NemotoCO => write!(f, "Nemoto™ CO"),
            SensorType::SimulationNO2Fix => write!(f, "Simulation NO2 (Fix)"),
            SensorType::SimulationCOFix => write!(f, "Simulation CO (Fix)"),
            SensorType::SimulationNO2 => write!(f, "Simulation NO2"),
            SensorType::SimulationCO => write!(f, "Simulation CO"),
        }
    }
}

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

impl Default for Sensor {
    fn default() -> Self {
        Self::new()
    }
}
