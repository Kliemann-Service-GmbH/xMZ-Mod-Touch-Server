use errors::*;
use shift_register::{ShiftRegister, ShiftRegisterType};
use co_no2_kombisensor::{Kombisensor};
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
            kombisensors: vec![],
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

    /// Default Konfiguration des Servers
    fn default_configuration(&mut self) -> Result<()> {
        try!(self.relais.set(1));
        try!(self.leds.set(1));
        try!(self.leds.set(3));

        Ok(())
    }

}
