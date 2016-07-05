use server::zone::Zone;
use std::result;

/// Mögliche Fehler die auftreten können
#[derive(Debug, Eq, PartialEq)]
pub enum SensorError {
    InvalidValue,
    NoAdcValue,
    NoAdcValueAtNullgas,
    NoAdcValueAtMessgas,
    NoConcentrationNullgas,
    NoConcentrationMessgas,
}

// Rust Type Alias
pub type Result<T> = result::Result<T, SensorError>;

#[derive(Debug, Eq, PartialEq)]
pub enum SensorType {
    /// Nemoto NO² Messzelle, EC NAP-550 https://www.nemoto.co.jp/nse/sensor-search/nap-550.html?lang=en
    NemotoNO2,
    /// Nemote CO Messzelle, EC NAP-505 https://www.nemoto.co.jp/nse/sensor-search/use/gas-alarm/nap-505.html?lang=en
    NemotoCO,
}

#[derive(Debug, Eq, PartialEq)]
enum Alarmauswertung {
    On,
    Off,
    Simulation,
}

pub struct Sensor<'a> {
    /// Sensor Typ
    pub sensor_type: SensorType,
    /// ADC Wert    - wird vom Server Prozess über das Modbus Protokoll ausgelesen und aktualisiert
    pub adc_value: Option<u16>,
    /// SI Einheit des Sensors (ppm, %UEG, Vol %)
    pub si: &'a str,
    adc_value_at_nullgas: Option<u32>,
    adc_value_at_messgas: Option<u32>,
    concentration_nullgas: Option<u32>,
    concentration_messgas: Option<u32>,
    /// Adresse des Modbus Registers für den ADC Wert
    pub modbus_register_address: u32,
    alarmauswertung: Alarmauswertung,
    zones: Vec<&'a Zone>,
}

impl<'a> Sensor<'a> {
    /// Erzeugt eine neue Sensor Instanz
    ///
    /// # Attributes
    /// * `sensor_type`     - `SensorType` Type der Messzelle
    ///
    pub fn new(sensor_type: SensorType) -> Self {
        match sensor_type {
            SensorType::NemotoNO2 => {
                Sensor {
                    sensor_type: SensorType::NemotoNO2,
                    adc_value: None,
                    si: "ppm",
                    adc_value_at_nullgas: Some(922),  // TODO: Read in sensor calibration data
                    adc_value_at_messgas: Some(622),  // TODO: Read in sensor calibration data
                    concentration_nullgas: Some(0),  // TODO: Read in sensor calibration data
                    concentration_messgas: Some(20),  // TODO: Read in sensor calibration data
                    modbus_register_address: 1,
                    alarmauswertung: Alarmauswertung::Simulation,
                    zones: vec!(),
                }
            },
            SensorType::NemotoCO => {
                Sensor {
                    sensor_type: SensorType::NemotoCO,
                    adc_value: None,
                    si: "ppm",
                    adc_value_at_nullgas: Some(107),  // TODO: Read in sensor calibration data
                    adc_value_at_messgas: Some(888),  // TODO: Read in sensor calibration data
                    concentration_nullgas: Some(0),  // TODO: Read in sensor calibration data
                    concentration_messgas: Some(280),  // TODO: Read in sensor calibration data
                    modbus_register_address: 11,
                    alarmauswertung: Alarmauswertung::Simulation,
                    zones: vec!(),
                }
            },
        }
    }

    /// `concentration` - Liefert den aktuell, berechneten Wert zurück
    ///
    /// Der Wert wird mit einer linearen Funktion aus den Calibrationsdaten `adc_value_at_nullgas`,
    /// `adc_value_at_messgas`, `concentration_nullgas`, `concentration_messgas` und dem akuellen
    /// Analog/ Digital Wert `adc_value` berechnet.
    ///
    /// Die Funktion liefert ein `Result<u32, SensorError>` zurück.
    /// Im Erfolgsfall ein u32 ansonnsten ein SensorError.
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::sensor::{Sensor, SensorError, SensorType};
    ///
    /// let mut sensor = Sensor::new(SensorType::NemotoNO2);
    /// assert_eq!(sensor.concentration(), Err(SensorError::NoAdcValue));
    /// ```
    pub fn concentration(&self) -> Result<f32> {
        let x = match self.adc_value {
            None => {return Err(SensorError::NoAdcValue); }
            Some(value) => {value}
        };
        let y2 = match self.concentration_messgas{
            None => {return Err(SensorError::NoConcentrationMessgas); }
            Some(value) => {value}
        };
        let y1 = match self.concentration_nullgas{
            None => {return Err(SensorError::NoConcentrationNullgas); }
            Some(value) => {value}
        };
        let x2 = match self.adc_value_at_messgas{
            None => {return Err(SensorError::NoAdcValueAtMessgas); }
            Some(value) => {value}
        };
        let x1 = match self.adc_value_at_nullgas{
            None => {return Err(SensorError::NoAdcValueAtNullgas); }
            Some(value) => {value}
        };

        let result: f32 = (y2 as f32 - y1 as f32) / (x2 as f32 - x1 as f32) * (x as f32 - x1 as f32) + y1 as f32;

        Ok(result)
    }

    /// Liefert den ADC Wert für eine gegebene Konzentration
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::sensor::{Sensor, SensorType};
    ///
    /// let mut sensor = Sensor::new(SensorType::NemotoNO2);
    /// assert_eq!(sensor.adc_from_concentration(20.0).unwrap(), 60.133335);
    /// ```
    pub fn adc_from_concentration(&self, concentration: f32) -> Result<f32> {
        let x = concentration;
        let y2 = match self.concentration_messgas{
            None => {return Err(SensorError::NoConcentrationMessgas); }
            Some(value) => {value}
        };
        let y1 = match self.concentration_nullgas{
            None => {return Err(SensorError::NoConcentrationNullgas); }
            Some(value) => {value}
        };
        let x2 = match self.adc_value_at_messgas{
            None => {return Err(SensorError::NoAdcValueAtMessgas); }
            Some(value) => {value}
        };
        let x1 = match self.adc_value_at_nullgas{
            None => {return Err(SensorError::NoAdcValueAtNullgas); }
            Some(value) => {value}
        };

        let result: f32 = (y2 as f32 - y1 as f32) / (x2 as f32 - x1 as f32) * (x as f32 - x1 as f32) + y1 as f32;

        Ok(result)
    }

    /// Listet alle berechneten Konzentrationen für alle möglichen ADC Werte auf.
    ///
    /// Basis sind die default Kalibrationsdaten der Sensoren, siehe `new()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::sensor::{Sensor, SensorType};
    ///
    /// let mut sensor = Sensor::new(SensorType::NemotoNO2);
    /// sensor.list_all_concentrations();
    /// ```
    ///
    /// ```
    /// use xmz_server::sensor::{Sensor, SensorType};
    ///
    /// let mut sensor = Sensor::new(SensorType::NemotoCO);
    /// sensor.list_all_concentrations();
    /// ```
    pub fn list_all_concentrations(&mut self) {
        for i in 0..1024 {
            self.adc_value = Some(i);
            println!("ADC Wert: [{}] entspricht einer Konzentration von: {} {}", i, self.concentration().unwrap(), self.si);
        }
    }
}


#[cfg(test)]
mod test {
    use server::Server;
    use server::zone::Zone;
    use sensor::{Alarmauswertung, Sensor, SensorType, SensorError};

    // Helper Funktion die ein NO2 Sensor zurück Liefert
    fn default_no2_sensor<'a>() -> Sensor<'a> {
        let mut sensor = Sensor::new(SensorType::NemotoNO2);
        sensor.adc_value = Some(772);
        sensor.adc_value_at_nullgas = Some(922);
        sensor.concentration_nullgas = Some(0);
        sensor.adc_value_at_messgas = Some(622);
        sensor.concentration_messgas = Some(20);
        sensor
    }

    #[test]
    fn modbus_register_adresse_nemoto_no2() {
        let sensor = Sensor::new(SensorType::NemotoNO2);
        assert_eq!(sensor.modbus_register_address, 1);
    }

    #[test]
    fn modbus_register_address_nemoto_co() {
        let sensor = Sensor::new(SensorType::NemotoCO);
        assert_eq!(sensor.modbus_register_address, 11);
    }

    #[test]
    fn alarmauswertung() {
        let sensor1 = Sensor::new(SensorType::NemotoCO);
        let sensor2 = Sensor::new(SensorType::NemotoNO2);
        assert_eq!(sensor1.alarmauswertung, Alarmauswertung::Simulation);
        assert_eq!(sensor2.alarmauswertung, Alarmauswertung::Simulation);
    }

    #[test]
    fn sensor_ohne_zone() {
        let sensor = Sensor::new(SensorType::NemotoNO2);
        assert_eq!(sensor.zones.len(), 0);
    }

    #[test]
    fn sensor_mit_einer_zone() {
        let server = Server::new();
        let mut sensor = Sensor::new(SensorType::NemotoNO2);
        sensor.zones.push(&server.zones[0]);
        assert_eq!(sensor.zones.len(), 1);
    }

    #[test]
    fn sensor_mit_mehr_als_einer_zone() {
        let server = Server::new();
        let mut sensor = Sensor::new(SensorType::NemotoNO2);
        sensor.zones.push(&server.zones[0]);
        sensor.zones.push(&server.zones[1]);
        assert_eq!(sensor.zones.len(), 2);
    }

    #[test]
    fn sensor_mit_einer_zone_kann_alarmpunkt_setzen() {
        let server = Server::new();
        let mut sensor = Sensor::new(SensorType::NemotoNO2);
        sensor.zones.push(&server.zones[0]);
        assert_eq!(sensor.zones[0].alarmpunkt(0).unwrap(), false);
        sensor.zones[0].alarmpunkt_set(0, true);
        assert_eq!(sensor.zones[0].alarmpunkt(0).unwrap(), true);
    }

    #[test]
    fn sensor_mit_mehr_als_einer_zone_kann_alarmpunkt_setzen() {
        let server = Server::new();
        let mut sensor = Sensor::new(SensorType::NemotoNO2);
        sensor.zones.push(&server.zones[0]);
        sensor.zones.push(&server.zones[1]);
        assert_eq!(sensor.zones[0].alarmpunkt(0).unwrap(), false);
        assert_eq!(sensor.zones[1].alarmpunkt(0).unwrap(), false);
        sensor.zones[0].alarmpunkt_set(0, true);
        sensor.zones[1].alarmpunkt_set(3, true);
        assert_eq!(sensor.zones[0].alarmpunkt(0).unwrap(), true);
        assert_eq!(sensor.zones[1].alarmpunkt(3).unwrap(), true);
    }


    // ADC
    #[test]
    fn concentration_should_fail_with_no_adc_value() {
        let mut sensor = default_no2_sensor();
        sensor.adc_value = None;
        assert_eq!(sensor.concentration(), Err(SensorError::NoAdcValue));
    }

    #[test]
    fn concentration_should_fail_with_no_adc_value_at_nullgas() {
        let mut sensor = default_no2_sensor();
        sensor.adc_value_at_nullgas = None;
        assert_eq!(sensor.concentration(), Err(SensorError::NoAdcValueAtNullgas));
    }

    #[test]
    fn concentration_should_fail_with_no_concentration_nullgas() {
        let mut sensor = default_no2_sensor();
        sensor.concentration_nullgas = None;
        assert_eq!(sensor.concentration(), Err(SensorError::NoConcentrationNullgas));
    }

    #[test]
    fn concentration_should_fail_with_no_concentration_messgas() {
        let mut sensor = default_no2_sensor();
        sensor.concentration_messgas = None;
        assert_eq!(sensor.concentration(), Err(SensorError::NoConcentrationMessgas));
    }

    #[test]
    fn concentration_no2() {
        let mut sensor = default_no2_sensor();
        assert_eq!(sensor.concentration().unwrap(), 10.000001);
    }

    #[test]
    fn concentration_co() {
        let mut sensor = Sensor::new(SensorType::NemotoCO);
        sensor.adc_value = Some(333);
        sensor.adc_value_at_nullgas = Some(114);
        sensor.concentration_nullgas = Some(0);
        sensor.adc_value_at_messgas = Some(875);
        sensor.concentration_messgas = Some(280);
        assert_eq!(sensor.concentration().unwrap(), 80.578186);
    }

}
