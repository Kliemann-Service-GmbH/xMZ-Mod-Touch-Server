//! CO-NO2 Kombisensor mit Modbus Transceiver
//!
use errors::*;
use server::zone::kombisensor::sensor::{Sensor, SensorType};
use std::fmt;


#[derive(Clone)]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize, Debug)]
pub enum KombisensorType {
    Unknown,    // Default Werte, keine Sensoren, ich wollte hier nicht Default als Member Name verwenden
    RAGas,
    RAGasSimulation,
}

#[derive(Clone)]
#[derive(PartialEq, PartialOrd)]
#[derive(Serialize, Deserialize, Debug)]
pub enum KombisensorStatus {
    // alles Ok
    Normal,
    // Kabelbruch
    Kabelbruch,
}

/// Ein Kombisensor kann `n` Sensormesszellen enthalten, nomal sind 2 Messzellen (NO2 und CO)
///
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Kombisensor {
    // Typ des Kombisensors
    kombisensor_type: KombisensorType,
    firmware_version: String,
    modbus_device: String,
    modbus_address: u8,
    modbus_debug: bool,
    sensors: Vec<Sensor>,
    error_count: u64,
    status: KombisensorStatus,
}

impl Kombisensor {
    /// Erzeugt eine neue Kombisensor Instanz
    ///
    /// # Return values
    ///
    /// Diese Funktion liefert eine neue Kombisensor Instanz
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    ///
    /// let kombisensor = Kombisensor::new();
    /// assert_eq!(kombisensor.get_sensors().len(), 0);
    /// ```
    pub fn new() -> Self {
        Kombisensor {
            kombisensor_type: KombisensorType::Unknown,
            firmware_version: "0.0.0".to_string(),
            modbus_address: 247,
            modbus_device: "/dev/ttyUSB0".to_string(),
            modbus_debug: false,
            sensors: vec![],
            error_count: 0,
            status: KombisensorStatus::Normal,
        }
    }
    /// Erzeugt eine spezielle Kombisensor Instanz
    ///
    /// # Parameters
    ///
    /// * `kombisensor_type`    - Typ des Kombisensors
    ///
    /// # Return values
    ///
    /// Diese Funktion liefert eine neue Kombisensor Instanz vom gegebenen Typ
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::{Kombisensor, KombisensorType};
    /// let kombisensor = Kombisensor::new_with_type(KombisensorType::RAGas);
    ///
    /// assert_eq!(kombisensor.get_modbus_device(), "/dev/ttyS1".to_string());
    /// ```
    pub fn new_with_type(kombisensor_type: KombisensorType) -> Self {
        match kombisensor_type {
            KombisensorType::RAGas => {
                Kombisensor {
                    kombisensor_type: kombisensor_type,
                    modbus_device: "/dev/ttyS1".to_string(),
                    sensors: vec![
                        Sensor::new_with_type(SensorType::NemotoNO2),
                        Sensor::new_with_type(SensorType::NemotoCO),
                    ],
                    ..Default::default()
                }
            }
            KombisensorType::RAGasSimulation => {
                Kombisensor {
                    kombisensor_type: kombisensor_type,
                    modbus_device: "/dev/ttyUSB0".to_string(),
                    modbus_debug: true,
                    sensors: vec![
                        Sensor::new_with_type(SensorType::SimulationNO2),
                        Sensor::new_with_type(SensorType::SimulationCO),
                    ],
                    ..Default::default()
                }
            }
            _ => { Kombisensor::new() }
        }
    }

    /// Get firmware version
    ///
    /// # Return values
    ///
    /// Liefert die aktuelle Firmware Version die aus dem Sensor ausgelesen wurde,
    /// eine Version "0.0.0" deutet darauf hin das der Kombisensor noch nie über Modbus ausgelesen wurde.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    ///
    /// let kombisensor = Kombisensor::new();
    /// assert_eq!(kombisensor.get_firmware_version(), "0.0.0");
    /// ```
    pub fn get_firmware_version(&self) -> String {
        self.firmware_version.clone()
    }

    /// Liefert den Typ des Kombisensors
    ///
    /// # Return values
    ///
    /// Liefert den Typ des Kombisensors
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::{Kombisensor, KombisensorType};
    ///
    /// let kombisensor = Kombisensor::new_with_type(KombisensorType::RAGasSimulation);
    /// assert_eq!(kombisensor.get_kombisensor_type(), KombisensorType::RAGasSimulation);
    /// ```
    pub fn get_kombisensor_type(&self) -> KombisensorType {
        self.kombisensor_type.clone()
    }

    /// Set firmware version
    ///
    /// # Parameters
    ///
    /// * `firmware_version`    - String mit der neuen Firmware Version
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    ///
    /// let mut kombisensor = Kombisensor::new();
    ///
    /// assert_eq!(kombisensor.get_firmware_version(), "0.0.0");
    /// kombisensor.set_firmware_version("9.9.99".to_string());
    /// assert_eq!(kombisensor.get_firmware_version(), "9.9.99");
    /// ```
    pub fn set_firmware_version(&mut self, firmware_version: String) {
        self.firmware_version = firmware_version
    }

    /// Get modbus_address
    ///
    /// # Return values
    ///
    /// Liefert die aktuelle Firmware Version die aus dem Sensor ausgelesen wurde,
    /// eine Version "0.0.0" deutet darauf hin das der Kombisensor noch nie über Modbus ausgelesen wurde.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    ///
    /// let kombisensor = Kombisensor::new();
    /// assert_eq!(kombisensor.get_modbus_address(), 247);
    /// ```
    pub fn get_modbus_address(&self) -> u8 {
        self.modbus_address
    }

    /// Set modbus_address
    ///
    /// # Parameters
    ///
    /// * `modbus_address`    - u8 mit der neuen Modbus Adresse
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    ///
    /// let mut kombisensor = Kombisensor::new();
    /// assert_eq!(kombisensor.get_modbus_address(), 247);
    /// kombisensor.set_modbus_address(1);
    /// assert_eq!(kombisensor.get_modbus_address(), 1);
    /// ```
    pub fn set_modbus_address(&mut self, modbus_address: u8) {
        self.modbus_address = modbus_address
    }

    /// Get modbus_device
    ///
    /// # Return values
    ///
    /// Liefert die Modbus Device Adresse als String
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    ///
    /// let kombisensor = Kombisensor::new();
    /// assert_eq!(kombisensor.get_modbus_device(), "/dev/ttyUSB0".to_string());
    /// ```
    pub fn get_modbus_device(&self) -> String {
        self.modbus_device.clone()
    }

    /// Set modbus_device
    ///
    /// # Parameters
    ///
    /// * `modbus_device`    - String mit der neuen Modbus Device Adresse
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    /// let mut kombisensor = Kombisensor::new();
    ///
    /// kombisensor.set_modbus_device("/dev/ttyS1".to_string());
    /// assert_eq!(kombisensor.get_modbus_device(), "/dev/ttyS1".to_string());
    /// ```
    pub fn set_modbus_device(&mut self, modbus_device: String) {
        self.modbus_device = modbus_device
    }

    /// Get modbus_debug
    ///
    /// # Return values
    ///
    /// Liefert die Modbus Device Adresse als boolen
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    /// let kombisensor = Kombisensor::new();
    ///
    /// assert_eq!(kombisensor.get_modbus_debug(), false);
    /// ```
    pub fn get_modbus_debug(&self) -> bool {
        self.modbus_debug
    }

    /// Set modbus_debug
    ///
    /// # Parameters
    ///
    /// * `modbus_debug`    - String mit der neuen Modbus Device Adresse
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    /// let mut kombisensor = Kombisensor::new();
    ///
    /// kombisensor.set_modbus_debug(true);
    /// assert_eq!(kombisensor.get_modbus_debug(), true);
    /// ```
    pub fn set_modbus_debug(&mut self, modbus_debug: bool) {
        self.modbus_debug = modbus_debug
    }

    /// TODO: Referenz auf Vector durch &[T] ersetzen
    /// Liefert eine Referenz auf einen Vector mit den Sensoren
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    ///
    /// let kombisensor = Kombisensor::new();
    /// assert_eq!(kombisensor.get_sensors().len(), 0);
    /// ```
    pub fn get_sensors(&self) -> &Vec<Sensor> {
        &self.sensors
    }

    /// Liefert eine Referenz auf einen Vector mit den Sensoren
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    ///
    /// let kombisensor = Kombisensor::new();
    /// assert_eq!(kombisensor.get_sensors().len(), 0);
    /// ```
    pub fn add_sensor(&mut self, sensor: Sensor) {
        self.sensors.push(sensor);
    }

    /// Liefert eine mutable Referenz auf einen Vector mit den Sensoren
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    ///
    /// let mut kombisensor = Kombisensor::new();
    /// assert_eq!(kombisensor.get_sensors_mut().len(), 0);
    /// ```
    pub fn get_sensors_mut(&mut self) -> &mut Vec<Sensor> {
        &mut self.sensors
    }

    /// Sensor Messzellen des Kombisensors
    ///
    /// # Return values
    ///
    /// Liefert ein Option Type, mit den Sensoren
    ///
    /// # Parameters
    ///
    /// * `id`  - Id der Sensormesszelle
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::{Kombisensor, KombisensorType};
    ///
    /// let kombisensor = Kombisensor::new_with_type(KombisensorType::RAGasSimulation);
    /// assert!(kombisensor.get_sensor(0).is_some());
    /// ```
    pub fn get_sensor(&self, id: usize) -> Option<&Sensor> {
        self.sensors.get(id)
    }

    /// Optionale, mutable Referenz auf eine Sensor Messzelle des Kombisensors
    ///
    /// # Return values
    ///
    /// Liefert ein Option Type, mit der mut Referenz auf den Sensor
    ///
    /// # Parameters
    ///
    /// * `id`  - Id der Sensormesszelle
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::{Kombisensor, KombisensorType};
    ///
    /// let mut kombisensor = Kombisensor::new_with_type(KombisensorType::RAGasSimulation);
    /// assert!(kombisensor.get_sensor_mut(0).is_some());
    /// ```
    pub fn get_sensor_mut(&mut self, id: usize) -> Option<&mut Sensor> {
        self.sensors.get_mut(id)
    }

    /// Get Error Counter
    ///
    /// # Return values
    ///
    /// Liefert den aktuellen Stand des Fehlerzählers
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    ///
    /// let kombisensor = Kombisensor::new();
    /// assert_eq!(kombisensor.get_error_count(), 0);
    /// ```
    pub fn get_error_count(&self) -> u64 {
        self.error_count
    }

    /// Erhöht den Fehlerzähler (Error Counter)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    ///
    /// let mut kombisensor = Kombisensor::new();
    ///
    /// assert_eq!(kombisensor.get_error_count(), 0);
    /// kombisensor.inc_error_count();
    /// assert_eq!(kombisensor.get_error_count(), 1);
    /// ```
    pub fn inc_error_count(&mut self) {
        self.error_count += 1
    }

    /// Reset den Fehlerzähler (Error Counter)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    /// let mut kombisensor = Kombisensor::new();
    /// kombisensor.inc_error_count();
    /// assert_eq!(kombisensor.get_error_count(), 1);
    ///
    /// kombisensor.reset_error_count();
    /// assert_eq!(kombisensor.get_error_count(), 0);
    /// ```
    pub fn reset_error_count(&mut self) {
        self.error_count = 0
    }

    /// Fragt die Daten des Kombisensors via Modbus ab
    ///
    /// # Return values
    ///
    /// Die Funktion liefert ein Result
    ///
    fn update_via_modbus(&mut self) -> Result<()> {
        if self.status != KombisensorStatus::Normal { bail!("Kabelbruch") };

        use libmodbus_rs::{Modbus, ModbusRTU, ModbusClient, MODBUS_RTU_MAX_ADU_LENGTH, SerialMode, RequestToSendMode};

        let mut modbus = Modbus::new_rtu(&self.modbus_device, 9600, 'N', 8, 1)?;
        modbus.set_slave(self.modbus_address)?;

        // Debug Modus einschalten wenn gewünscht
        modbus.set_debug(self.modbus_debug)?;

        if self.kombisensor_type == KombisensorType::RAGas {
            // debug!("modbus.rtu_set_serial_mode(SerialMode::MODBUS_RTU_RS485)");
            // modbus.rtu_set_serial_mode(SerialMode::MODBUS_RTU_RS485)?;
            debug!("modbus.rtu_set_rts(RequestToSendMode::MODBUS_RTU_RTS_DOWN)");
            modbus.rtu_set_rts(RequestToSendMode::MODBUS_RTU_RTS_DOWN)?;
        }

        // modbus.connect().map_err(|_| self.inc_error_count() );
        modbus.connect()?;

        let mut response_register = vec![0u16; MODBUS_RTU_MAX_ADU_LENGTH as usize];
        modbus.read_registers(0, 30, &mut response_register)?;

        if response_register.len() < 28 { bail!("Modbus Data invalid: {:?}", response_register) }

        // Parse die empfangenen Daten
        // TODO: unwrap() entfernen!
        let firmware_version_major = *response_register.get(0).unwrap();
        let firmware_version_minor = *response_register.get(1).unwrap();
        let firmware_version_patch = *response_register.get(2).unwrap();
        // let modbus_address = *response_register.get(3).unwrap();
        let sensor1_num = *response_register.get(10).unwrap();
        let sensor1_adc_value = *response_register.get(11).unwrap();
        let sensor1_min_value = *response_register.get(12).unwrap();
        let sensor1_max_value = *response_register.get(13).unwrap();
        let sensor1_adc_value_at_nullgas = *response_register.get(14).unwrap();
        let sensor1_adc_value_at_messgas = *response_register.get(15).unwrap();
        let sensor1_concentration_at_nullgas = *response_register.get(16).unwrap();
        let sensor1_concentration_at_messgas = *response_register.get(17).unwrap();
        let sensor1_config = *response_register.get(18).unwrap();
        let sensor2_num = *response_register.get(20).unwrap();
        let sensor2_adc_value = *response_register.get(21).unwrap();
        let sensor2_min_value = *response_register.get(22).unwrap();
        let sensor2_max_value = *response_register.get(23).unwrap();
        let sensor2_adc_value_at_nullgas = *response_register.get(24).unwrap();
        let sensor2_adc_value_at_messgas = *response_register.get(25).unwrap();
        let sensor2_concentration_at_nullgas = *response_register.get(26).unwrap();
        let sensor2_concentration_at_messgas = *response_register.get(27).unwrap();
        let sensor2_config = *response_register.get(28).unwrap();

        self.set_firmware_version(format!("{}.{}.{}", firmware_version_major, firmware_version_minor, firmware_version_patch));

        // Run through both sensors and update the mebers
        if let Some(sensor1) = self.get_sensor_mut(0) {
            sensor1.set_adc_value(sensor1_adc_value);
            sensor1.set_adc_value(sensor1_adc_value);
            sensor1.set_min_value(sensor1_min_value);
            sensor1.set_max_value(sensor1_max_value);
            sensor1.set_adc_value_at_nullgas(sensor1_adc_value_at_nullgas);
            sensor1.set_adc_value_at_messgas(sensor1_adc_value_at_messgas);
            sensor1.set_concentration_at_nullgas(sensor1_concentration_at_nullgas);
            sensor1.set_concentration_at_messgas(sensor1_concentration_at_messgas);
            sensor1.set_config(sensor1_config);
        }

        if let Some(sensor2) = self.get_sensor_mut(1) {
            sensor2.set_adc_value(sensor2_adc_value);
            sensor2.set_adc_value(sensor2_adc_value);
            sensor2.set_min_value(sensor2_min_value);
            sensor2.set_max_value(sensor2_max_value);
            sensor2.set_adc_value_at_nullgas(sensor2_adc_value_at_nullgas);
            sensor2.set_adc_value_at_messgas(sensor2_adc_value_at_messgas);
            sensor2.set_concentration_at_nullgas(sensor2_concentration_at_nullgas);
            sensor2.set_concentration_at_messgas(sensor2_concentration_at_messgas);
            sensor2.set_config(sensor2_config);
        }


        Ok(())
    }

    /// Gibt den Status des Kombisensors wieder
    pub fn get_status(&self) -> KombisensorStatus {
        self.status.clone()
    }

    // Update Status des Kombisensors
    //
    // Diese Funktion wird in der public `update()` Funktion aufgerufen.
    fn update_status(&mut self) {
        if self.error_count >= 5 {
            self.status = KombisensorStatus::Kabelbruch;
        } else {
            self.status = KombisensorStatus::Normal;
        }

    }


    /// Update Funktion des Kombisensors
    ///
    /// Diese Funktion fast die einzelnen Update Funktionen des Kombisensors zusammen
    pub fn update(&mut self) {
        match self.update_via_modbus() {
            Ok(_) => {}
            Err(_) => self.error_count += 1,
        }

        self.update_status();
    }

}

impl fmt::Display for KombisensorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            KombisensorType::Unknown => write!(f, "Unbekannter Kombisensor"),
            KombisensorType::RAGas => write!(f, "RA-GAS CO/ NO2 Kombisensor"),
            KombisensorType::RAGasSimulation => write!(f, "RA-GAS CO/ NO2 Kombisensor (Sim)"),
        }
    }
}

impl Default for Kombisensor {
    fn default() -> Self {
        Self::new()
    }
}
