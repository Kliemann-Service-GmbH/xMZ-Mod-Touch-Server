use co_no2_kombisensor::{Kombisensor};
use errors::*;
use libmodbus_rs::modbus::Modbus;
use log::LogLevel;
use shift_register::{ShiftRegister, ShiftRegisterType};
use std::fs;
use zone::{Zone, ZoneType};


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
    /// Erzeugt eine neue Server Instanz
    ///
    pub fn new() -> Self {
        Server { ..Default::default() }
    }

    pub fn get_kombisensors(&self) -> &Vec<Kombisensor> {
        self.kombisensors.as_ref()
    }
    pub fn get_kombisensors_mut(&mut self) -> &mut Vec<Kombisensor> {
        self.kombisensors.as_mut()
    }

    /// Grundeinstellungen der Hardware
    ///
    /// Dazu gehören unter anderem das leeren der ShiftRegister Speicher,
    /// ein Lampentest der LEDs sowie das Schalten der Default Konfiguration.
    pub fn init(&mut self) -> Result<()> {
        try!(self.leds.reset());
        try!(self.relais.reset());
        // Lampentest
        try!(self.leds.test());

        try!(self.default_configuration());

        Ok(())
    }

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
            try!(modbus_context.set_slave(kombisensor.get_modbus_slave_id() as i32));
            // Modbus Debug wenn das Programm im Debug Mode läuft
            if log_enabled!(LogLevel::Debug) {
                try!(modbus_context.set_debug(true));
            }
            // try!(modbus_context.rtu_set_rts(MODBUS_RTU_RTS_DOWN));
            let mut _tab_reg: Vec<u16> = Vec::new();

            for sensor in kombisensor.get_sensors_mut().iter_mut() {
                sensor.set_adc_value(10);
            }
        }

        Ok(())
    }

    /// Default Konfiguration des Servers
    fn default_configuration(&mut self) -> Result<()> {
        try!(self.relais.set(1));
        try!(self.leds.set(1)); // Power

        Ok(())
    }

}
