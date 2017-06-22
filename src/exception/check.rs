use errors::*;
use shift_register::{ShiftRegister, ShiftRegisterType};


pub struct Check<'a, O>
    where   O: 'a,
{
    object: &'a O,
    function: fn(&O) -> bool,
}

impl<'a, O> Check<'a, O>
    where   O: 'a,
{
    /// Erstellt ein neuen Check
    ///
    /// # Parameters
    ///
    /// * `object`      - Object dessen Funktion aufgerufen werden soll
    /// * `function`    - Funktion die aufgerufen werden soll
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::{Check, Server};
    /// let server = Server::new();
    ///
    /// let check = Check::new(&server, Server::wartungsintervall_reached);
    /// ```
    pub fn new(object: &'a O, function: fn(&O) -> bool) -> Check<O> {
        Check {
            object,
            function,
        }
    }

    /// Test aufrufen
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::{Check, Server};
    /// let server = Server::new();
    /// let check = Check::new(&server, Server::wartungsintervall_reached);
    ///
    /// assert!(!check.test());
    /// ```
    pub fn test(&self) -> bool {
        (self.function)(self.object)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_check() {
        use server::Server;
        let server = Server::new();

        let check = Check::new(&server, Server::wartungsintervall_reached);
    }

//     #[test]
//     fn vec_box_checks() {
//         let leds = ShiftRegister::new(ShiftRegisterType::Simulation);
//         let relais = ShiftRegister::new(ShiftRegisterType::Simulation);
//
//         let check1 = Check::new(&leds, ShiftRegister::set, 2);
//         let check2 = Check::new(&relais, ShiftRegister::clear, 1);
//
//         let checks: Vec<Box<Check<ShiftRegister, u64>>> = vec![Box::new(check1), Box::new(check2)];
//
//         // Startbedingungen
//         relais.set(1);
//         assert_eq!(leds.get_data().unwrap(), 0b0);
//         assert_eq!(relais.get_data().unwrap(), 0b1);
//
//         // check1.run();
//         // check2.run();
//         for check in checks {
//             check.run();
//         }
//
//         // Nachdem die Checken ausgeführt wurden, Endbedingen prüfen
//         assert_eq!(leds.get_data().unwrap(), 0b10);
//         assert_eq!(relais.get_data().unwrap(), 0b0);
//     }
}
