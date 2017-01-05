use errors::*;
use sysfs_gpio::{Direction, Pin};


#[derive(Debug, Eq, PartialEq)]
pub enum ShiftRegisterType {
    LED,
    RELAIS,
    Simulation,
}

pub struct ShiftRegister {
    register_type: ShiftRegisterType,
    pub oe_pin: Option<Pin>,
    pub ds_pin: Option<Pin>,
    pub clock_pin: Option<Pin>,
    pub latch_pin: Option<Pin>,
    pub data: u64,
}

impl ShiftRegister {
    /// Erzeugt ein neuen Shift Register
    ///
    /// # Arguments
    /// * `register_type`     - Art des Shift Registers
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::*;
    ///
    /// let sim = ShiftRegister::new(ShiftRegisterType::Simulation);
    /// assert_eq!(sim.data, 0b0);
    /// ```
    pub fn new(register_type: ShiftRegisterType) -> Self {
        match register_type {
            ShiftRegisterType::LED => ShiftRegister {
                register_type: register_type,
                oe_pin: Some(Pin::new(276)),
                ds_pin: Some(Pin::new(38)),
                clock_pin: Some(Pin::new(44)),
                latch_pin: Some(Pin::new(40)),
                data: 0,
            },
            ShiftRegisterType::RELAIS => ShiftRegister {
                register_type: register_type,
                oe_pin: Some(Pin::new(277)),
                ds_pin: Some(Pin::new(45)),
                clock_pin: Some(Pin::new(39)),
                latch_pin: Some(Pin::new(37)),
                data: 0,
            },
            ShiftRegisterType::Simulation => ShiftRegister {
                register_type: register_type,
                oe_pin: None,
                ds_pin: None,
                clock_pin: None,
                latch_pin: None,
                data: 0,
            }
        }
    }
}
