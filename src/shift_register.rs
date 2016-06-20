/// Representiert die verschiedenen Shift Register Typen
///
/// Zur Zeit gibt es 2 verschiedene Shift Register Typen
///
/// * LED       - Led Shift Register sind drei 8bit, serial in, paralel out, Shift Register, daisy chained
///             - Nur die ersten 24 Ausgänge sind verbunden
/// * RELAIS    - Die Relais Shift Register sind zwei 8bit, serial in, paralel out, Shift Register, daisy chained
///             - Nur die ersten 9 Ausgänge sind verbunden
#[derive(Debug, Eq, PartialEq)]
pub enum ShiftRegisterType {
    LED,
    RELAIS,
}

/// Datenstruktur der Shift Register Hardware
///
/// Das `data` Feld ist ein Buffer der den aktuellen Zustand der Shift Register wiederspiegelt.
/// Shift Register können nur geschrieben werden, desshalb benötigt man ein Speicherbereich um
/// zum Beispiel den aktuellen Zustand einzelner Bits abfragen zu können.
#[derive(Debug, Eq, PartialEq)]
pub struct ShiftRegister {
    register_type: ShiftRegisterType,
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
    /// use xmz_server::shift_register::{ShiftRegister,ShiftRegisterType};
    ///
    /// let led = ShiftRegister::new(ShiftRegisterType::LED);
    /// let relais = ShiftRegister::new(ShiftRegisterType::RELAIS);
    /// assert_eq!(led.data, 0b0);
    /// assert_eq!(relais.data, 0b0);
    /// ```
    pub fn new(register_type: ShiftRegisterType) -> Self {
        match register_type {
            ShiftRegisterType::LED => ShiftRegister {
                register_type: ShiftRegisterType::LED,
                data: 0,
            },
            ShiftRegisterType::RELAIS => ShiftRegister {
                register_type: ShiftRegisterType::RELAIS,
                data: 0,
            }
        }
    }

    /// Setzt das übergebene Bit im Shift Register `data` Buffer
    ///
    /// # Arguments
    /// * `num`     - Nummer des zu setzenden Bits **Diese Nummer ist Eins basiert!**
    ///
    /// Der Parameter ist nicht Null basiert. Das bedeutet `set(1)` setzt das erste Bit(0) im `data`
    /// Buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::shift_register::{ShiftRegister,ShiftRegisterType};
    ///
    /// let mut led = ShiftRegister::new(ShiftRegisterType::LED);
    /// assert_eq!(led.data, 0b0);
    /// led.set(3);
    /// assert_eq!(led.data, 0b100);
    /// ```
    /// More info: http://stackoverflow.com/questions/47981/how-do-you-set-clear-and-toggle-a-single-bit-in-c-c
    pub fn set(&mut self, num: u64) {
        self.data |= 1 << num -1;
    }

    /// Abfrage ob ein Bit gesetzt ist, `true` wenn ja, `false` wenn das bit nicht gesetzt ist
    ///
    /// # Arguments
    /// * `num`     - Nummer des abzufragenden Bits **Diese Nummer ist Eins basiert!**
    ///
    /// Der Parameter ist nicht Null basiert. Das bedeutet `get(1)` fragt das erste Bit(0) im `data`
    /// Buffer ab.
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::shift_register::{ShiftRegister,ShiftRegisterType};
    ///
    /// let mut led = ShiftRegister::new(ShiftRegisterType::LED);
    /// led.set(1);
    /// led.set(3);
    /// assert_eq!(led.get(1), true);
    /// assert_eq!(led.get(2), false);
    /// assert_eq!(led.get(3), true);
    /// ```
    /// More info: http://stackoverflow.com/questions/47981/how-do-you-set-clear-and-toggle-a-single-bit-in-c-c
    pub fn get(&self, num: u64) -> bool {
        match (self.data >> num -1) & 1 {
            0 => false,
            _ => true,
        }
    }

    /// Löscht das übergebene Bit
    ///
    /// # Arguments
    /// * `num`     - Nummer des zu löschenden Bits **Diese Nummer ist Eins basiert!**
    ///
    /// Der Parameter ist nicht Null basiert. Das bedeutet `clear(1)` löscht das erste Bit(0) im `data`
    /// Buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::shift_register::{ShiftRegister,ShiftRegisterType};
    ///
    /// let mut led = ShiftRegister::new(ShiftRegisterType::LED);
    /// assert_eq!(led.data, 0b0);
    ///
    /// led.set(3);
    /// assert_eq!(led.get(3), true);
    ///
    /// led.clear(3);
    /// assert_eq!(led.get(3), false);
    /// ```
    pub fn clear(&mut self, num: u64) {
        self.data &= 1 << num;
    }

    /// Schaltet das übergebene Bit um, war es Null dann wird es Eins und umgekehrt
    ///
    /// # Arguments
    /// * `num`     - Nummer des zu wechselnden Bits **Diese Nummer ist Eins basiert!**
    ///
    /// Der Parameter ist nicht Null basiert. Das bedeutet `toggle(1)` schaltet das erste Bit(0) im `data`
    /// Buffer um.
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::shift_register::{ShiftRegister,ShiftRegisterType};
    ///
    /// let mut led = ShiftRegister::new(ShiftRegisterType::LED);
    /// assert_eq!(led.data, 0b0);
    ///
    /// led.toggle(3);
    /// assert_eq!(led.get(3), true);
    /// led.toggle(3);
    /// assert_eq!(led.get(3), false);
    /// ```
    pub fn toggle(&mut self, num: u64) {
        self.data ^= 1 << num -1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_register_defaults() {
        let led = ShiftRegister::new(ShiftRegisterType::LED);
        assert_eq!(led.register_type, ShiftRegisterType::LED);
    }

    #[test]
    fn shift_register_creation() {
        let led = ShiftRegister::new(ShiftRegisterType::LED);
        assert_eq!(led.data, 0);
    }

    #[test]
    fn set_and_clear_all_bits() {
        let mut led = ShiftRegister::new(ShiftRegisterType::LED);

        for i in 1..64 {
            led.set(i);
            assert_eq!(led.get(i), true);

            led.clear(i);
            assert_eq!(led.get(i), false);
        }
    }

    #[test]
    fn toggle_all_bites_one_time() {
        let mut led = ShiftRegister::new(ShiftRegisterType::LED);

        for i in 1..64 {
            assert_eq!(led.get(i), false);
            led.toggle(i);
            assert_eq!(led.get(i), true);
            led.toggle(i);
            assert_eq!(led.get(i), false);
        }
    }
}
