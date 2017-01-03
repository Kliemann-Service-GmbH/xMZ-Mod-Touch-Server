use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub enum SensorType {
    /// Nemoto NO2 Messzelle, EC NAP-550
    /// Datenblatt: https://www.nemoto.co.jp/nse/sensor-search/nap-550.html?lang=en
    NemotoNO2,
    /// Nemote CO Messzelle, EC NAP-505
    /// Datenblatt: https://www.nemoto.co.jp/nse/sensor-search/use/gas-alarm/nap-505.html?lang=en
    NemotoCO,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)] 
pub enum SI {
    ppm,
    Vol,
    UEG,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sensor {
    sensor_type: SensorType,
    /// ADC Wert    - wird vom Server Prozess über das Modbus Protokoll ausgelesen und aktualisiert
    /// SI Einheit des Sensors (ppm, % UEG, Vol %)
    si: SI,
    adc_value: Option<u16>,
    adc_value_at_nullgas: Option<u16>,
    adc_value_at_messgas: Option<u16>,
    concentration_nullgas: Option<u32>,
    concentration_messgas: Option<u32>,
    /// Adresse des Modbus Registers für den ADC Wert
    modbus_register_address: u32,
    /// Fehlerzähler
    error_count: u32,
}

impl fmt::Display for SensorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SensorType::NemotoNO2 => write!(f, "Nemoto™ NO2"),
            SensorType::NemotoCO => write!(f, "Nemoto™ CO"),
        }
    }
}

impl fmt::Display for SI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SI::ppm => write!(f, "ppm"),
            SI::UEG => write!(f, "% UEG"),
            SI::Vol => write!(f, "Vol %"),
        }
    }
}

impl Sensor {
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
                    si: SI::ppm,
                    adc_value_at_nullgas: Some(922),
                    adc_value_at_messgas: Some(622),
                    concentration_nullgas: Some(0),
                    concentration_messgas: Some(20),
                    modbus_register_address: 1,
                    error_count: 0,
                }
            }
            SensorType::NemotoCO => {
                Sensor {
                    sensor_type: SensorType::NemotoCO,
                    adc_value: None,
                    si: SI::ppm,
                    adc_value_at_nullgas: Some(107),
                    adc_value_at_messgas: Some(888),
                    concentration_nullgas: Some(0),
                    concentration_messgas: Some(280),
                    modbus_register_address: 11,
                    error_count: 0,
                }
            }
        }
    }
}
