//! Exceptions Ausnahmen/ Fehlerbehandlung
//!
//!
//! Playground URL: https://play.rust-lang.org/?gist=6c0100d86a96f116615f43389f7b8af6&version=nightly&backtrace=0
//! Gist URL: https://gist.github.com/6c0100d86a96f116615f43389f7b8af6
//!
#[derive(Debug)]
#[derive(Hash, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum ExceptionType {
    WartungsintervalReached,
    KombisensorOffline { num_zone: usize },
    KombisensorModbusError { num_zone: usize, num_kombisensor: usize },
    SensorDirectValue { num_zone: usize, sensor: usize },
}

#[derive(Debug)]
#[derive(Hash, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Exception {
    exception_type: ExceptionType,
}

impl Exception {
    /// Neue Ausnahme erstellen
    ///
    /// # Parameters
    ///
    /// * `exception_type`  - ExceptionType
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_mod_touch_server::{Exception, ExceptionType};
    /// let exception = Exception::new(ExceptionType::WartungsintervalReached);
    /// ```
    pub fn new(exception_type: ExceptionType) -> Self {
        Exception { exception_type: exception_type }
    }
}
