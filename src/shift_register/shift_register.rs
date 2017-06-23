use errors::*;
use rand::Rng;
use std::sync::{RwLock};
use std::thread;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};


/// Verwendungszweck des ShiftRegister Arrays
///
/// Je nach Typ werden andere Hardware Pins angesprochen.
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum ShiftRegisterType {
    LED,
    Relais,
    Simulation,
}

/// Representation der Shiftregister Hardware
///
/// Diese Struct hält neben dem ShiftRegisterTypen,
/// die Pin Nummern der Hardware Pins (in Options so das Testumgebungen ohne Hardware simuliert werden kann).
/// **In den Options werden absichtlich nicht direkt die Pin::new() Instanzen gespeichert, da diese nicht serialisiert
/// werden können. Das Speichern der Server Konfig wäre somit nicht möglich.**
///
/// Der Data Member dieser Stuctur ist in ein RwLock gekapselt, so das der Wert auch
/// bei immutablen Referenzen auf den ShiftRegister geändert werden kann.
///
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ShiftRegister {
    register_type: ShiftRegisterType,
    oe_pin: Option<u64>,
    ds_pin: Option<u64>,
    clock_pin: Option<u64>,
    latch_pin: Option<u64>,
    // Interior Mutability wird benötigt, um die ShiftRegister nicht als &mut Referenzen
    // durch die gesammte Anwendung schleifen zu müssen.
    data: RwLock<u64>,
}

impl Default for ShiftRegister {
    fn default() -> Self {
        ShiftRegister {
            register_type: ShiftRegisterType::Simulation,
            oe_pin: None,
            ds_pin: None,
            clock_pin: None,
            latch_pin: None,
            data: RwLock::new(0),
        }
    }
}

impl ShiftRegister {
    /// Erzeugt ein neuen Shift Register
    ///
    /// # Return values
    ///
    /// Diese Funktion erzeugt eine ShiftRegister Datenstruktur. In dieser wird der aktuelle Zustand der ShiftRegister gespeichert `data`.
    /// Zudem enthält sie die Implemetation div. Helper funktionen die den ShiftRegister verwalten können.
    ///
    /// # Parameters
    ///
    /// * `register_type`     - Art des Shift Registers
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_mod_touch_server::{ShiftRegister, ShiftRegisterType};
    ///
    /// let sim = ShiftRegister::new(ShiftRegisterType::Simulation);
    /// assert_eq!(sim.get_data().unwrap(), 0b0);
    /// ```
    pub fn new(register_type: ShiftRegisterType) -> Self {
        match register_type {
            ShiftRegisterType::LED => ShiftRegister {
                register_type: register_type,
                oe_pin: Some(276),
                ds_pin: Some(38),
                clock_pin: Some(44),
                latch_pin: Some(40),
                ..Default::default()
            },
            ShiftRegisterType::Relais => ShiftRegister {
                register_type: register_type,
                oe_pin: Some(277),
                ds_pin: Some(45),
                clock_pin: Some(39),
                latch_pin: Some(37),
                ..Default::default()
            },
            _ => ShiftRegister { // der Catch all Arm fängt auch `ShiftRegisterType::Simulation`
                register_type: register_type,
                ..Default::default()
            }
        }
    }

    /// Setzt das übergebene Bit im Shift Register `data` Buffer
    ///
    /// # Parameters
    /// * `num`     - Nummer des zu setzenden Bits **Diese Nummer ist Eins basiert!**
    ///
    /// Der Parameter ist nicht Null basiert. Das bedeutet `set(1)` setzt das erste Bit(0) im `data`
    /// Buffer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::{ShiftRegister, ShiftRegisterType};
    ///
    /// let sim = ShiftRegister::new(ShiftRegisterType::Simulation);
    /// assert_eq!(sim.get_data().unwrap(), 0b0);
    /// sim.set(3).unwrap();
    /// assert_eq!(sim.get_data().unwrap(), 0b100);
    /// ```
    /// More info: http://stackoverflow.com/questions/47981/how-do-you-set-clear-and-toggle-a-single-bit-in-c-c
    pub fn set(&self, num: u64) -> Result<()> {
        if let Ok(mut data) = self.data.write() {
            *data |= 1 << (num -1);
        } else {
            bail!("Could not write lock data member")
        } // RwLock der `data` wird wieder frei gegeben, wichtig sonnst funktioniert `self.shift_out()` nicht

        self.shift_out()?;

        Ok(())
    }

    /// Abfrage ob ein Bit gesetzt ist, `true` wenn ja, `false` wenn das bit nicht gesetzt ist
    ///
    /// # Parameters
    ///
    /// * `num`     - Nummer des abzufragenden Bits **Diese Nummer ist Eins basiert!**
    ///
    /// Der Parameter ist nicht Null basiert. D.h. `get(1)` fragt das erste Bit(0) im `data`
    /// Buffer ab.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::{ShiftRegister, ShiftRegisterType};
    ///
    /// let sim = ShiftRegister::new(ShiftRegisterType::Simulation);
    /// sim.set(1).unwrap();
    /// sim.set(3).unwrap();
    /// assert_eq!(sim.get(1).unwrap(), true);
    /// assert_eq!(sim.get(2).unwrap(), false);
    /// assert_eq!(sim.get(3).unwrap(), true);
    /// ```
    /// More info: http://stackoverflow.com/questions/47981/how-do-you-set-clear-and-toggle-a-single-bit-in-c-c
    pub fn get(&self, num: u64) -> Result<bool> {
        if let Ok(data) = self.data.read() {
            match (*data >> (num -1)) & 1 {
                0 => Ok(false),
                _ => Ok(true),
            }
        } else {
            bail!("Could not read lock data member");
        }
    }

    /// Löscht das übergebene Bit
    ///
    /// # Parameters
    ///
    /// * `num`     - Nummer des zu löschenden Bits **Diese Nummer ist Eins basiert!**
    ///
    /// Der Parameter ist nicht Null basiert. D.h. `clear(1)` löscht das erste Bit(0) im `data`
    /// Buffer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::{ShiftRegister, ShiftRegisterType};
    ///
    /// let sim = ShiftRegister::new(ShiftRegisterType::Simulation);
    /// assert_eq!(sim.get_data().unwrap(), 0b0);
    ///
    /// sim.set(1).unwrap();
    /// sim.set(3).unwrap();
    /// assert_eq!(sim.get(1).unwrap(), true);
    /// assert_eq!(sim.get(3).unwrap(), true);
    ///
    /// sim.clear(3).unwrap();
    /// assert_eq!(sim.get(1).unwrap(), true);
    /// assert_eq!(sim.get(3).unwrap(), false);
    /// ```
    pub fn clear(&self, num: u64) -> Result<()> {
        if let Ok(mut data) = self.data.write() {
            *data &= !(1 << (num - 1));
        } else {
            bail!("Could not write lock data member")
        } // RwLock der `data` wird wieder frei gegeben, wichtig sonnst funktioniert `self.shift_out()` nicht

        self.shift_out()?;

        Ok(())
    }

    /// Schaltet das übergebene Bit um, war es Null dann wird es Eins und umgekehrt
    ///
    /// # Parameters
    ///
    /// * `num`     - Nummer des zu wechselnden Bits **Diese Nummer ist Eins basiert!**
    ///
    /// Der Parameter ist nicht Null basiert. D.h. `toggle(1)` schaltet das erste Bit(0) im `data`
    /// Buffer um.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::{ShiftRegister, ShiftRegisterType};
    ///
    /// let sim = ShiftRegister::new(ShiftRegisterType::Simulation);
    /// assert_eq!(sim.get_data().unwrap(), 0b0);
    ///
    /// sim.toggle(3).unwrap();
    /// assert_eq!(sim.get(3).unwrap(), true);
    /// sim.toggle(3).unwrap();
    /// assert_eq!(sim.get(3).unwrap(), false);
    /// ```
    pub fn toggle(&self, num: u64) -> Result<()> {
        if let Ok(mut data) = self.data.write() {
            *data ^= 1 << (num -1);
        } else {
            bail!("Could not write lock data member")
        } // RwLock der `data` wird wieder frei gegeben, wichtig sonnst funktioniert `self.shift_out()` nicht

        self.shift_out()?;

        Ok(())
    }

    /// Reset nullt den Datenspeicher und gleicht ihn mit der Hardware ab.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::{ShiftRegister, ShiftRegisterType};
    ///
    /// let sim = ShiftRegister::new(ShiftRegisterType::Simulation);
    /// assert_eq!(sim.get(1).unwrap(), false);
    /// sim.set(1).unwrap();
    /// assert_eq!(sim.get(1).unwrap(), true);
    /// sim.reset().unwrap();
    /// assert_eq!(sim.get(1).unwrap(), false);
    /// ```
    pub fn reset(&self) -> Result<()> {
        if let Ok(mut data) = self.data.write() {
            *data = 0;
        } else {
            bail!("Could not write lock data member")
        } // RwLock der `data` wird wieder frei gegeben, wichtig sonnst funktioniert `self.shift_out()` nicht

        self.shift_out()?;

        Ok(())
    }


    /// Lampentest testet alle Outputs
    ///
    /// Diese Funktion schaltet alle Ausgänge high, wartet eine Sekunde und schaltet danach alle
    /// Ausgänge wieder aus.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::{ShiftRegister, ShiftRegisterType};
    ///
    /// let sim = ShiftRegister::new(ShiftRegisterType::Simulation);
    ///
    /// sim.set(1).unwrap();
    /// sim.clear(10).unwrap();
    /// sim.test().unwrap();
    /// assert_eq!(sim.get(1).unwrap(), true);
    /// assert_eq!(sim.get(10).unwrap(), false);
    /// ```
    pub fn test(&self) -> Result<()> {
        let old_state;

        if let Ok(mut data) = self.data.write() {
            // Alten Stand speichern
            old_state = *data;
            // Buffer komplett mit Einsen füllen
            *data = u64::max_value();
        } else {
            bail!("Could not write lock data member")
        } // RwLock der `data` wird wieder frei gegeben, wichtig sonnst funktioniert `self.shift_out()` nicht

        self.shift_out()?;

        // 1Sec warten
        thread::sleep(Duration::new(1, 0));

        self.reset()?;

        if let Ok(mut data) = self.data.write() {
            // alten Stand wieder herstellen
            *data = old_state;
        } else {
            bail!("Could not write lock data member")
        } // RwLock der `data` wird wieder frei gegeben, wichtig sonnst funktioniert `self.shift_out()` nicht

        self.shift_out()?;

        Ok(())
    }

    /// Random Lampentest testet einige, wirklich vorhanden, Outputs, zufällig
    ///
    /// Diese Funktion schaltet alle Ausgänge high, wartet eine Sekunde und schaltet danach alle
    /// Ausgänge wieder aus.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::{ShiftRegister, ShiftRegisterType};
    ///
    /// let sim = ShiftRegister::new(ShiftRegisterType::Simulation);
    ///
    /// sim.test_random().unwrap();
    /// ```
    pub fn test_random(&self) -> Result<()> {
        let old_state;

        if let Ok(mut data) = self.data.write() {
            // Alten Stand speichern
            old_state = *data;
            // Buffer mit Zufallsdaten füllen
            *data = ::rand::thread_rng().gen_range(1, u64::max_value());
        } else {
            bail!("Could not write lock data member")
        } // RwLock der `data` wird wieder frei gegeben, wichtig sonnst funktioniert `self.shift_out()` nicht

        self.shift_out()?;

        // 1Sec warten
        thread::sleep(Duration::new(1, 0));

        self.reset()?;

        if let Ok(mut data) = self.data.write() {
            // alten Stand wieder herstellen
            *data = old_state;
        } else {
            bail!("Could not write lock data member")
        } // RwLock der `data` wird wieder frei gegeben, wichtig sonnst funktioniert `self.shift_out()` nicht

        self.shift_out()?;

        Ok(())
    }

    pub fn get_data(&self) -> Result<u64> {
        match self.data.read() {
            Ok(data) => Ok(data.clone()),
            Err(_) => bail!("Could not read lock data member"),
        }
    }

    /// Exportiert die Pins in das sysfs des Linux Kernels
    ///
    fn export_pins(&self) -> Result<()> {
        if let Some(oe_pin) = self.oe_pin { Pin::new(oe_pin).export()? };
        if let Some(ds_pin) = self.ds_pin { Pin::new(ds_pin).export()? };
        if let Some(clock_pin) = self.clock_pin { Pin::new(clock_pin).export()? };
        if let Some(latch_pin) = self.latch_pin { Pin::new(latch_pin).export()? };

        Ok(())
    }

    /// Schaltet die Pins in den OUTPUT Pin Modus
    ///
    fn set_pin_direction_output(&self) -> Result<()> {
        if let Some(oe_pin) = self.oe_pin { Pin::new(oe_pin).set_direction(Direction::Out)? };
        if let Some(oe_pin) = self.oe_pin { Pin::new(oe_pin).set_value(0)? }; // !OE pin low == Shift register enabled.
        if let Some(ds_pin) = self.ds_pin { Pin::new(ds_pin).set_direction(Direction::Out)? };
        if let Some(ds_pin) = self.ds_pin { Pin::new(ds_pin).set_value(0)? };
        if let Some(clock_pin) = self.clock_pin { Pin::new(clock_pin).set_direction(Direction::Out)? };
        if let Some(clock_pin) = self.clock_pin { Pin::new(clock_pin).set_value(0)? };
        if let Some(latch_pin) = self.latch_pin { Pin::new(latch_pin).set_direction(Direction::Out)? };
        if let Some(latch_pin) = self.latch_pin { Pin::new(latch_pin).set_value(0)? };

        Ok(())
    }

    /// Toggelt den Latch Pin pin high->low,
    ///
    fn latch_out(&self) -> Result<()> {
        if let Some(latch_pin) = self.latch_pin { Pin::new(latch_pin).set_value(1)? };
        if let Some(latch_pin) = self.latch_pin { Pin::new(latch_pin).set_value(0)? };

        Ok(())
    }

    /// Toogelt den Clock Pin high->low
    ///
    fn clock_in(&self) -> Result<()> {
        if let Some(clock_pin) = self.clock_pin { Pin::new(clock_pin).set_value(1)? };
        if let Some(clock_pin) = self.clock_pin { Pin::new(clock_pin).set_value(0)? };

        Ok(())
    }

    /// Schiebt die kompletten Daten in die Schiebe Register und schaltet anschließend
    /// die Ausgänge dieser Schiebe Register (latch out)
    ///
    fn shift_out(&self) -> Result<()> {
        // Wenn export_pins erfolgreich ist werden die Daten eingeclocked, ansonsten passiert nix
        self.export_pins()?;
        self.set_pin_direction_output()?;

        // Daten einclocken
        for i in (0..64).rev() {
            // RwLock von `data`
            if let Ok(data) = self.data.read() {
                match (*data >> i) & 1 {
                    1 => { if let Some(ds_pin) = self.ds_pin { Pin::new(ds_pin).set_value(1)? } },
                    _ => { if let Some(ds_pin) = self.ds_pin { Pin::new(ds_pin).set_value(0)? } },
                }
            } // RwLock der `data` wird wieder frei gegeben
            self.clock_in()?;
        }
        self.latch_out()?;

        Ok(())
    }
}
