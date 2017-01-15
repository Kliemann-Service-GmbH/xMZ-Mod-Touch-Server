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

    /// FIXME: Refactor das!
    /// Update der Sensor Datenstruktur via Modbus
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let server = Server::new();
    /// ```
    pub fn update_sensors(&mut self) -> Result<()> {
        use co_no2_kombisensor::kombisensor::to_bytes;

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
            // Nur Kombisensoren auslesen die nicht so viele Fehler haben
            // Der Error Counter soll nicht connective Sensoren erkennen helfen.
            if kombisensor.get_error_count() < 5 {
                // Modbus Adresse setzen
                try!(modbus_context.set_slave(kombisensor.get_modbus_slave_id() as i32)
                    .chain_err(|| format!("Modbus Address: {}, invalid", kombisensor.get_modbus_slave_id())));
                // Modbus Debug wenn das Programm im Debug Mode läuft
                if log_enabled!(LogLevel::Debug) {
                    try!(modbus_context.set_debug(true));
                }
                // Modbus Datenstruktur vorbereiten
                let mut tab_reg: Vec<u16> = Vec::new();
                // Mit der Seriellen Schnittstelle eine Verbindung aufbauen
                modbus_context.connect();
                // 30 Register über das Modbus Protokoll aus dem Kombisensor auslesen
                let tab_reg = modbus_context.read_registers(0 as i32, 30)
                    .map_err(|err| {
                        warn!("Kombisensor nicht erreichbar, Fehlerzähler um Eins erhöht.");
                        kombisensor.inc_error_count();
                    })
                    .map(|tab_reg| {
                        // Fehlerzähler wieder auf Null setzen
                        info!("Fehlerzähler wieder auf Null gesetzt.");
                        kombisensor.reset_error_count();
                        let bytes = to_bytes(tab_reg);
                        kombisensor.parse(&bytes[..]);
                    });
                modbus_context.close();
            } else {
                debug!("To many Errors while try reaching kombisensor: {}", kombisensor.get_modbus_slave_id());
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
