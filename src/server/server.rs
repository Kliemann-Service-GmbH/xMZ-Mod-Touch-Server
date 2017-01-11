use co_no2_kombisensor::{Kombisensor};
use errors::*;
use libmodbus_rs::modbus::Modbus;
use log::LogLevel;
use shift_register::{ShiftRegister, ShiftRegisterType};
use std::fs;
use zone::{Zone, ZoneType};


#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMode {
    /// Normalbetrieb
    Normal,
    /// Wartungsmodus, Sensoren werden ausgelesen, jedoch keine Alarmauswertung
    Wartung,
    /// Es erfolgt keine echte Modbus Komunikation mit den Sensoren
    Simmulation,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    leds: ShiftRegister,
    relais: ShiftRegister,
    kombisensors: Vec<Kombisensor>,
    zones: Vec<Zone>,
    modbus_serial_device: String,
    modbus_baud: i32,
    modbus_parity: char,
    modbus_data_bit: i32,
    modbus_stop_bit: i32,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            leds: ShiftRegister::new(ShiftRegisterType::LED),
            relais: ShiftRegister::new(ShiftRegisterType::RELAIS),
            kombisensors: vec![
                Kombisensor::new(),
            ],
            zones: vec![
                Zone::new(ZoneType::STOERUNG),
                Zone::new(ZoneType::SCHWELLWERT),
            ],
            modbus_serial_device: "/dev/ttyS1".to_string(),
            modbus_baud: 9600,
            modbus_parity: 'N',
            modbus_data_bit: 8,
            modbus_stop_bit: 1,
        }
    }
}

impl Server {
    /// Erzeugt eine neue Instanz
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let server = Server::new();
    /// ```
    pub fn new() -> Self {
        Server { ..Default::default() }
    }

    /// Liefert eine Referenz zu den Kombisensoren des Servers
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let server = Server::new();
    /// ```
    pub fn get_kombisensors(&self) -> &Vec<Kombisensor> {
        self.kombisensors.as_ref()
    }

    /// Liefert eine **mutable** Referenz zu den Kombisensoren des Servers
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let server = Server::new();
    /// ```
    pub fn get_kombisensors_mut(&mut self) -> &mut Vec<Kombisensor> {
        self.kombisensors.as_mut()
    }

    /// Grundeinstellungen der Hardware
    ///
    /// Dazu gehören unter anderem das leeren der ShiftRegister Speicher,
    /// ein Lampentest der LEDs sowie das Schalten der Default Konfiguration.
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let server = Server::new();
    /// ```
    pub fn init(&mut self) -> Result<()> {
        // Relais und LEDs aus beim Hardware Start
        try!(self.leds.reset());
        try!(self.relais.reset());
        // Lampentest, alle LED leuchten 1s auf
        try!(self.leds.test());

        try!(self.default_configuration());

        Ok(())
    }

    /// Update der Sensor Datenstruktur via Modbus
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let server = Server::new();
    /// ```
    pub fn update_sensors(&mut self) -> Result<()> {
        // Test ob das Serielle Interface existiert
        // und ob die Berechtigungen für ein Zugriff ausreichen
        try!(fs::metadata(&self.modbus_serial_device)
            .chain_err(|| "Server's Modbus Serial Interface not found"));

        // Modbus Kontext erzeugen
        let mut modbus_context = Modbus::new_rtu(self.modbus_serial_device.as_ref(),
                                    self.modbus_baud,
                                    self.modbus_parity,
                                    self.modbus_data_bit,
                                    self.modbus_stop_bit);

        for kombisensor in self.get_kombisensors_mut().iter_mut() {
            // Modbus Adresse setzen
            try!(modbus_context.set_slave(kombisensor.get_modbus_slave_id() as i32));
            // Modbus Debug wenn das Programm im Debug Mode läuft
            if log_enabled!(LogLevel::Debug) {
                try!(modbus_context.set_debug(true));
            }
            // try!(modbus_context.rtu_set_rts(MODBUS_RTU_RTS_DOWN));
            let mut tab_reg: Vec<u16> = Vec::new();

            for sensor in kombisensor.get_sensors_mut().iter_mut() {
                if sensor.get_error_count() <= 5 {
                    match modbus_context.connect() {
                        Ok(_) => {
                            info!("Fehlerzähler des Sensors wird wieder auf Null gesetzt.");
                            sensor.error_count_reset();
                            try!(modbus_context.read_registers(0 as i32, 30).map(|tab_reg| {
                                // tab_reg.get(0).map(|var| sensor.set_adc_value(Some(*var)));
                                println!("{:?}", tab_reg);
                            }));
                            modbus_context.close();
                        }
                        Err(err) => {
                            debug!("modbus_connect() fehlgeschlagen, erhöhe Sensor.error_count: {} um eins", sensor.get_error_count());
                            sensor.error_count_inc();
                        }
                    }
                } else {
                    debug!("Der Fehlerzähler des Sensors (error_count) ist gleich 5. Es wird kein weiterer `modbus_connect()` versucht.");
                }
            }
        }

        Ok(())
    }

    /// Default Konfiguration des Servers
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let server = Server::new();
    /// ```
    fn default_configuration(&mut self) -> Result<()> {
        try!(self.relais.set(1));
        try!(self.leds.set(1)); // Power

        Ok(())
    }
}
