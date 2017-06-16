/// Ausnahme Typ
///
#[derive(Debug)]
#[derive(Hash, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum ExceptionType {
    WartungsintervalReached,
    KombisensorOffline { num_zone: usize },
    KombisensorModbusError { num_zone: usize, num_kombisensor: usize },
    SensorAP3DirectValue { num_zone: usize, num_sensor: usize },
    SensorAP2Average15min { num_zone: usize, num_sensor: usize },
    SensorAP1Average15min { num_zone: usize, num_sensor: usize },
}

/// Ausnahme (Fehler der auftreten kann)
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
