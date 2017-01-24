//! Die Kombisensor Datenstruktur representiert eine Platine eines [CO-NO2-Kombisensor-Mod](https://github.com/Kliemann-Service-GmbH/CO-NO2-Kombisensor-Mod) der Firma RA-GAS
//!
use co_no2_kombisensor::sensor::{SI, Sensor, SensorType};
use nom::{IResult, le_u8, le_u16};

// use errors::*;

/// Platine des CO-NO2-Kombisensor-Mod
#[derive(Clone, Debug)]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Kombisensor {
    #[serde(default)]
    version: String,
    modbus_slave_id: u8,
    #[serde(default)]
    sensors: Vec<Sensor>,
    #[serde(default)]
    error_count: u64,
}

impl Default for Kombisensor {
    fn default() -> Self {
        Kombisensor {
            version: "0.0.0".to_string(),
            modbus_slave_id: 247,
            sensors: vec![
                Sensor::new_with_type(SensorType::NemotoNO2),
                Sensor::new_with_type(SensorType::NemotoCO),
            ],
            error_count: 0,
        }
    }
}

impl Kombisensor {
    /// Erzeugt eine neue Instanz
    ///
    pub fn new() -> Self {
        Kombisensor { ..Default::default() }
    }

    /// Liefert die Modbus Adresse des Kombisensors
    ///
    /// Jede Platine, eine Kombisensor mit mehreren Sensormeßzellen, hat eine Modbus Slave ID
    /// die so genannte Modbus Adresse.
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let kombisensor = Kombisensor::new();
    /// assert_eq!(kombisensor.get_modbus_slave_id(), 247);
    /// ```
    pub fn get_modbus_slave_id(&self) -> u8 {
        self.modbus_slave_id
    }

    /// Liefert eine Sensor Referenz in Option verpackt
    ///
    /// # Parameters
    /// `num`   - Nummer des gesuchten Sensors
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    /// ```
    pub fn get_sensor(&self, num: usize) -> Option<&Sensor> {
        self.sensors.get(num)
    }

    /// Liefert eine mutable Sensor Referenz in Option verpackt
    ///
    /// # Parameters
    /// `num`   - Nummer des gesuchten Sensors
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    /// ```
    pub fn get_sensor_mut(&mut self, num: usize) -> Option<&mut Sensor> {
        self.sensors.get_mut(num)
    }

    /// Liefert die Sensoren als Referenze zu einem Vector
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    /// ```
    pub fn get_sensors(&self) -> &Vec<Sensor> {
        self.sensors.as_ref()
    }

    /// Liefert die Sensoren in einem mutablen Vector Slice
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    /// ```
    pub fn get_sensors_mut(&mut self) -> &mut Vec<Sensor> {
        self.sensors.as_mut()
    }

    /// Liefert den Fehlerzähler zurück
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let kombisensor = Kombisensor::new();
    /// assert_eq!(kombisensor.get_error_count(), 0);
    /// ```
    pub fn get_error_count(&self) -> u64 {
        self.error_count
    }

    /// Erhöht den Fehlerzähler um Eins
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let mut kombisensor = Kombisensor::new();
    /// assert_eq!(kombisensor.get_error_count(), 0);
    /// kombisensor.inc_error_count();
    /// assert_eq!(kombisensor.get_error_count(), 1);
    /// ```
    pub fn inc_error_count(&mut self) {
        self.error_count += 1;
    }

    /// Setzt den Fehlerzähler auf Null zurück
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let mut kombisensor = Kombisensor::new();
    /// kombisensor.inc_error_count();
    /// assert_eq!(kombisensor.get_error_count(), 1);
    /// kombisensor.reset_error_count();
    /// assert_eq!(kombisensor.get_error_count(), 0);
    /// ```
    pub fn reset_error_count(&mut self) {
        self.error_count = 0;
    }

    /// Setzt die Modbus Slave Id
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    /// ```
    pub fn set_modbus_slave_id(&mut self, modbus_slave_id: u8) {
        self.modbus_slave_id = modbus_slave_id;
    }

    /// Setzt die Modbus Slave Id
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    /// ```
    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    /// Diese Funktion stellt erstmal sicher das jeder Kombisensor auch 2 Sensor Datenstructuren
    /// besitzt. Beim aller ersten Start des Systems z.B. besitzt jede Kombisensor Datenstruktur nur
    /// ein leeres Vec<Sensor>. Da die Sensor Werte meistens nicht in der Konfiguration zu finden sind.
    ///
    /// Kein Witz, die Parameter dieser Funktion sind wirklich so zahlreich.
    pub fn set_sensors_and_values(&mut self,
        sensor1_number :u16, sensor2_number :u16,
        sensor1_adc_value :u16, sensor2_adc_value :u16,
        sensor1_min_value :u16, sensor2_min_value :u16,
        sensor1_max_value :u16, sensor2_max_value :u16,
        sensor1_adc_value_at_nullgas :u16, sensor2_adc_value_at_nullgas :u16,
        sensor1_adc_value_at_messgas :u16, sensor2_adc_value_at_messgas :u16,
        sensor1_concentration_at_nullgas :u16, sensor2_concentration_at_nullgas :u16,
        sensor1_concentration_at_messgas :u16, sensor2_concentration_at_messgas :u16,
        sensor1_config :u16, sensor2_config :u16,)
    {
        while self.sensors.len() < 1 {
            self.sensors.push(Sensor::new_with_type(SensorType::NemotoNO2));
        }
        while self.sensors.len() < 2 {
            self.sensors.push(Sensor::new_with_type(SensorType::NemotoCO));
        }
        {
            let mut sensor1 = self.get_sensor_mut(0).unwrap();
            sensor1.set_number(sensor1_number);
            sensor1.set_adc_value(sensor1_adc_value);
            sensor1.set_min_value(sensor1_min_value);
            sensor1.set_max_value(sensor1_max_value);
            sensor1.set_adc_value_at_nullgas(sensor1_adc_value_at_nullgas);
            sensor1.set_adc_value_at_messgas(sensor1_adc_value_at_messgas);
            sensor1.set_concentration_at_nullgas(sensor1_concentration_at_nullgas);
            sensor1.set_concentration_at_messgas(sensor1_concentration_at_messgas);
            sensor1.set_config(sensor1_config);

        }
        {
            let mut sensor2 = self.get_sensor_mut(1).unwrap();
            sensor2.set_number(sensor2_number);
            sensor2.set_adc_value(sensor2_adc_value);
            sensor2.set_min_value(sensor2_min_value);
            sensor2.set_max_value(sensor2_max_value);
            sensor2.set_adc_value_at_nullgas(sensor2_adc_value_at_nullgas);
            sensor2.set_adc_value_at_messgas(sensor2_adc_value_at_messgas);
            sensor2.set_concentration_at_nullgas(sensor2_concentration_at_nullgas);
            sensor2.set_concentration_at_messgas(sensor2_concentration_at_messgas);
            sensor2.set_config(sensor2_config);
       }
    }

    pub fn parse<'a>(&mut self, input: &'a [u8]) -> IResult<&'a [u8], ()> {
        do_parse!(input,
            major: le_u16 >>
            minor: le_u16 >>
            patch: le_u16 >>
            slave_id: le_u8 >>
            _buffer: take!(13) >>
            sensor1_number: le_u16 >>
            sensor1_adc_value: le_u16 >>
            sensor1_min_value: le_u16 >>
            sensor1_max_value: le_u16 >>
            sensor1_adc_value_at_nullgas: le_u16 >>
            sensor1_adc_value_at_messgas: le_u16 >>
            sensor1_concentration_at_nullgas: le_u16 >>
            sensor1_concentration_at_messgas: le_u16 >>
            sensor1_config: le_u16 >>
            _sensor1_buffer: take!(2) >>
            sensor2_number: le_u16 >>
            sensor2_adc_value: le_u16 >>
            sensor2_min_value: le_u16 >>
            sensor2_max_value: le_u16 >>
            sensor2_adc_value_at_nullgas: le_u16 >>
            sensor2_adc_value_at_messgas: le_u16 >>
            sensor2_concentration_at_nullgas: le_u16 >>
            sensor2_concentration_at_messgas: le_u16 >>
            sensor2_config: le_u16 >>
            _sensor2_buffer: take!(2) >>
            (
                {
                    self.set_version( format!("{}.{}.{}", major, minor, patch) );
                    self.set_modbus_slave_id( slave_id );
                    self.set_sensors_and_values(
                        sensor1_number, sensor2_number,
                        sensor1_adc_value, sensor2_adc_value,
                        sensor1_min_value, sensor2_min_value,
                        sensor1_max_value, sensor2_max_value,
                        sensor1_adc_value_at_nullgas, sensor2_adc_value_at_nullgas,
                        sensor1_adc_value_at_messgas, sensor2_adc_value_at_messgas,
                        sensor1_concentration_at_nullgas, sensor2_concentration_at_nullgas,
                        sensor1_concentration_at_messgas, sensor2_concentration_at_messgas,
                        sensor1_config, sensor2_config,
                    );
                }
            )
        )
    }
}

/// Wandelt den gegebenen Vec<u16> in ein Slice aus bytes
///
/// Diese Funktion wird für die Kombisensor.parse() Parameter benätigt.
///
/// # Examples
/// ```
/// use xmz_server::*;
/// ```
pub fn to_bytes<'a>(slice_u16: Vec<u16>) -> &'a [u8] {
    use std::slice;
    use std::mem;
    let slice_u8: &[u8] = unsafe {
        slice::from_raw_parts(
            slice_u16.as_ptr() as *const u8,
            slice_u16.len() * mem::size_of::<u16>()
        )
    };

    slice_u8
}


#[cfg(test)]
mod tests {
    use super::*;

    // Helper
    fn get_input_helper() -> Vec<u16> {
        vec![
            0, 13, 10, 3, 0, 0, 0, 0, 0, 0,
            1, 922, 0, 30, 920, 564, 0, 20, 1, 0,
            2, 101, 0, 300, 112, 760, 0, 280, 1, 0
        ]
    }

    #[test]
    fn test_kombisensor_parser_set_version() {
        let input = get_input_helper();
        let bytes = to_bytes(input);
        let mut kombisensor = Kombisensor::new();
        assert_eq!(kombisensor.version, "0.0.0");
        kombisensor.parse(&bytes[..]);
        assert_eq!(kombisensor.version, "0.13.10");
    }

    #[test]
    fn test_kombisensor_parser_modbus_slave_id() {
        let input = get_input_helper();
        let bytes = to_bytes(input);
        let mut kombisensor = Kombisensor::new();
        assert_eq!(kombisensor.modbus_slave_id, 247);
        kombisensor.parse(&bytes[..]);
        assert_eq!(kombisensor.modbus_slave_id, 3);
    }

    #[test]
    fn test_kombisensor_parser_sensors() {
        let input = get_input_helper();
        let bytes = to_bytes(input);
        let mut kombisensor = Kombisensor::new();
        assert_eq!(kombisensor.get_sensors().len(), 0);
        kombisensor.parse(&bytes[..]);
        assert_eq!(kombisensor.get_sensors().len(), 2);
    }

    #[test]
    fn test_kombisensor_sensors1_nummer() {
        let input = get_input_helper();
        let bytes = to_bytes(input);
        let mut kombisensor = Kombisensor::new();
        kombisensor.parse(&bytes[..]);
        let sensor1_num = kombisensor.get_sensor(0).unwrap().get_number();
        assert_eq!(sensor1_num, 1);
    }

    #[test]
    fn test_sensor1_set_adc_value() {
        let input = get_input_helper();
        let bytes = to_bytes(input);
        let mut kombisensor = Kombisensor::new();
        kombisensor.parse(&bytes[..]);
        let sensor1_adc_value = kombisensor.get_sensor(0).unwrap().get_adc_value();
        assert_eq!(sensor1_adc_value, 922)
    }

    #[test]
    fn test_sensor1_set_min_value() {
        let input = get_input_helper();
        let bytes = to_bytes(input);
        let mut kombisensor = Kombisensor::new();
        kombisensor.parse(&bytes[..]);
        let sensor1_min_value = kombisensor.get_sensor(0).unwrap().get_min_value();
        assert_eq!(sensor1_min_value, 0)
    }

    #[test]
    fn test_sensor1_set_max_value() {
        let input = get_input_helper();
        let bytes = to_bytes(input);
        let mut kombisensor = Kombisensor::new();
        kombisensor.parse(&bytes[..]);
        let sensor1_max_value = kombisensor.get_sensor(0).unwrap().get_max_value();
        assert_eq!(sensor1_max_value, 30)
    }

    #[test]
    fn test_sensor1_set_adc_value_at_nullgas() {
        let input = get_input_helper();
        let bytes = to_bytes(input);
        let mut kombisensor = Kombisensor::new();
        kombisensor.parse(&bytes[..]);
        let sensor1_adc_value_at_nullgas = kombisensor.get_sensor(0).unwrap().get_adc_value_at_nullgas();
        assert_eq!(sensor1_adc_value_at_nullgas, 920)
    }

    #[test]
    fn test_sensor1_set_adc_value_at_messgas() {
        let input = get_input_helper();
        let bytes = to_bytes(input);
        let mut kombisensor = Kombisensor::new();
        kombisensor.parse(&bytes[..]);
        let sensor1_adc_value_at_messgas = kombisensor.get_sensor(0).unwrap().get_adc_value_at_messgas();
        assert_eq!(sensor1_adc_value_at_messgas, 564)
    }

    #[test]
    fn test_sensor1_set_concentration_at_nullgas() {
        let input = get_input_helper();
        let bytes = to_bytes(input);
        let mut kombisensor = Kombisensor::new();
        kombisensor.parse(&bytes[..]);
        let sensor1_concentration_at_nullgas = kombisensor.get_sensor(0).unwrap().get_concentration_at_nullgas();
        assert_eq!(sensor1_concentration_at_nullgas, 0)
    }

    #[test]
    fn test_sensor1_set_concentration_at_messgas() {
        let input = get_input_helper();
        let bytes = to_bytes(input);
        let mut kombisensor = Kombisensor::new();
        kombisensor.parse(&bytes[..]);
        let sensor1_concentration_at_messgas = kombisensor.get_sensor(0).unwrap().get_concentration_at_messgas();
        assert_eq!(sensor1_concentration_at_messgas, 20)
    }

    #[test]
    fn test_sensor1_set_config() {
        let input = get_input_helper();
        let bytes = to_bytes(input);
        let mut kombisensor = Kombisensor::new();
        kombisensor.parse(&bytes[..]);
        let sensor1_config = kombisensor.get_sensor(0).unwrap().get_config();
        assert_eq!(sensor1_config, 1)
    }
}
