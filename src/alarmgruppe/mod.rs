use errors::*;


pub enum ShiftRegisterType {
    LED,
    RELAIS,
}

pub struct ShiftRegister {
    register_type: ShiftRegisterType,
    // pub oe_pin: Pin,
    // pub ds_pin: Pin,
    // pub clock_pin: Pin,
    // pub latch_pin: Pin,
    pub data: u64,
}

impl ShiftRegister {
    pub fn new(register_type: ShiftRegisterType) -> Self {
        match register_type {
            ShiftRegisterType::LED => ShiftRegister {
                register_type: register_type,
                data: 0,
            },
            ShiftRegisterType::RELAIS => ShiftRegister {
                register_type: register_type,
                data: 0,
            }
        }
    }
}


#[derive(Debug, Eq, PartialEq)]
pub enum AlarmgruppenTyp {
    Stoerung,
    Zone,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Alarmgruppe {
    gruppen_typ: AlarmgruppenTyp,
}

impl Alarmgruppe {
    /// Erzeugt eine neue Alarmgruppe
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let stoerung = Alarmgruppe::new(AlarmgruppenTyp::Stoerung);
    /// ```
    ///
    /// ```
    /// use xmz_server::*;
    ///
    /// let zone1 = Alarmgruppe::new(AlarmgruppenTyp::Stoerung);
    /// ```
    ///
    pub fn new(gruppen_typ: AlarmgruppenTyp) -> Self {
        match gruppen_typ {
            AlarmgruppenTyp::Stoerung => { Alarmgruppe { gruppen_typ: gruppen_typ } }
            AlarmgruppenTyp::Zone => { Alarmgruppe { gruppen_typ: gruppen_typ } }
        }
    }

}


mod tests {
    use super::*;

    #[test]
    fn relais() {
        let relais = ShiftRegister::new(ShiftRegisterType::RELAIS);
    }

    #[test]
    fn leds() {
        let led = ShiftRegister::new(ShiftRegisterType::LED);
    }

    #[test]
    fn stoerung() {
        let stoerung = Alarmgruppe::new(AlarmgruppenTyp::Stoerung);
    }

    #[test]
    fn zone1() {
        let zone1 = Alarmgruppe::new(AlarmgruppenTyp::Zone);
    }
}
