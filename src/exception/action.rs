use errors::*;
use shift_register::{ShiftRegister, ShiftRegisterType};


pub struct Action<'a, O, P>
    where
        O: 'a,
        P: Copy,
{
    object: &'a O,
    function: fn(&O, P) -> Result<()>,
    param: P,
}

impl<'a, O, P> Action<'a, O, P>
    where
        O: 'a,
        P: Copy,
{
    /// Erstellt eine neue Action
    ///
    /// # Parameters
    ///
    /// * `object`      - Object dessen Funktion aufgerufen werden soll
    /// * `function`    - Funktion die aufgerufen werden soll
    /// * `param`       - Parameter der Funktion
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::{Action, ShiftRegister, ShiftRegisterType};
    ///
    /// let leds = ShiftRegister::new(ShiftRegisterType::Simulation);
    /// let action = Action::new(&leds, ShiftRegister::set, 2);
    /// ```
    pub fn new(object: &'a O, function: fn(&O, P) -> Result<()>, param: P) -> Action<O, P> {
        Action {
            object,
            function,
            param,
        }
    }

    /// Führt die Funktion der Aktion aus
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::{Action, ShiftRegister, ShiftRegisterType};
    /// let leds = ShiftRegister::new(ShiftRegisterType::Simulation);
    /// let action = Action::new(&leds, ShiftRegister::set, 2);
    ///
    /// assert!(action.run().is_ok());
    /// ```
    pub fn run(&self) -> Result<()> {
        (self.function)(self.object, self.param)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_action() {
        let leds = ShiftRegister::new(ShiftRegisterType::Simulation);

        let action = Action::new(&leds, ShiftRegister::set, 2);
        assert!(action.run().is_ok());
    }

    #[test]
    fn vec_box_actions() {
        let leds = ShiftRegister::new(ShiftRegisterType::Simulation);
        let relais = ShiftRegister::new(ShiftRegisterType::Simulation);

        let action1 = Action::new(&leds, ShiftRegister::set, 2);
        let action2 = Action::new(&relais, ShiftRegister::clear, 1);

        let actions: Vec<Box<Action<ShiftRegister, u64>>> = vec![Box::new(action1), Box::new(action2)];

        // Startbedingungen
        relais.set(1);
        assert_eq!(leds.get_data().unwrap(), 0b0);
        assert_eq!(relais.get_data().unwrap(), 0b1);

        // action1.run();
        // action2.run();
        for action in actions {
            action.run();
        }

        // Nachdem die Actionen ausgeführt wurden, Endbedingen prüfen
        assert_eq!(leds.get_data().unwrap(), 0b10);
        assert_eq!(relais.get_data().unwrap(), 0b0);
    }
}
