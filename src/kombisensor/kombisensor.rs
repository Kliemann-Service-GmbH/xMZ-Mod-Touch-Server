//! CO-NO2 Kombisensor mit Modbus Transceiver
//!
use errors::*;
use kombisensor::Sensor;


/// Ein Kombisensor kann `n` Sensormesszellen enthalten, nomal sind 2 Messzellen (NO2 und CO)
///
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Kombisensor {
    firmware_version: String,
    modbus_address: u8,
    sensors: Vec<Sensor>,
    error_count: u64,
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
    /// assert_eq!(kombisensor.get_sensors().len(), 2); // 2 Kombisensoren sind default
    /// ```
    pub fn new() -> Self {
        Kombisensor {
            firmware_version: "0.0.0".to_string(),
            modbus_address: 247,
            // TODO: Remove this two default sensors, if the config generator is working
            sensors: vec![
                Sensor::new(),
                Sensor::new(),
            ],
            error_count: 0,
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

    /// Liefert eine Referenz auf einen Vector mit den Sensoren
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    ///
    /// let kombisensor = Kombisensor::new();
    /// assert_eq!(kombisensor.get_sensors().len(), 2); // 2 Kombisensoren sind default
    /// ```
    pub fn get_sensors(&self) -> &Vec<Sensor> {
        &self.sensors
    }

    /// Liefert eine mutable Referenz auf einen Vector mit den Sensoren
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    ///
    /// let mut kombisensor = Kombisensor::new();
    /// assert_eq!(kombisensor.get_sensors_mut().len(), 2); // 2 Kombisensoren sind default
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
    /// use xmz_mod_touch_server::Kombisensor;
    ///
    /// let kombisensor = Kombisensor::new();
    /// assert!(kombisensor.get_sensor(0).is_some());
    /// ```
    pub fn get_sensor(&self, id: usize) -> Option<&Sensor> {
        self.sensors.get(id)
    }

    /// Mutable Refernz auf Sensor Messzellen des Kombisensors
    ///
    /// # Return values
    ///
    /// Liefert ein Option Type, mit den mut Sensoren
    ///
    /// # Parameters
    ///
    /// * `id`  - Id der Sensormesszelle
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::Kombisensor;
    ///
    /// let mut kombisensor = Kombisensor::new();
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

    /// Erhöht den  Error Counter
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

    /// Reset den Error Counter
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
    /// # Examples
    ///
    /// ```rust,ignore
    /// assert!(false);
    /// ```
    pub fn get_from_modbus(&self) -> Result<()> {
        // use libmodbus_rs::{Modbus, ModbusRTU, ModbusClient, MODBUS_RTU_MAX_ADU_LENGTH};
        //
        // let device: String = "/dev/ttyUSB0".to_string();
        // let slave_id: u8 = 247;
        //
        // let mut modbus = Modbus::new_rtu(&device, 9600, 'N', 8, 1)?;
        // modbus.set_slave(slave_id)?;
        //
        // // modbus.set_debug(true);
        // modbus.connect()?;
        //
        // let mut response_register = vec![0u16; MODBUS_RTU_MAX_ADU_LENGTH as usize];
        // modbus.read_registers(0, 30, &mut response_register)?;
        //
        // Ok(response_register)

        Ok(())
    }


}

impl Default for Kombisensor {
    fn default() -> Self {
        Self::new()
    }
}
