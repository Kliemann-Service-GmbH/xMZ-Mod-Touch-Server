//! Modul zur Verwaltung der globale Server Konfigurationsdatei
//!
//!
use error::*;
use co_no2_kombisensor::Kombisensor;
use serde_json;
use server::Server;


/// Representiert die Globale Konfigurationsdatei des Servers
#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub server: Server,
    pub kombisensors: Vec<Kombisensor>,
}

/// Configuration des Servers
///
/// Die Konfiguration wird nur aus der JSON Datei generiert. **Deshalb ist eine `new()` Funktion
/// nicht nötig.**
/// Für ein Beispiel einer manuellen Initalisierung siehe `examples/configuration_to_json.rs`
impl Configuration {
    pub fn from_config(config: String) -> Result<Configuration> {
        let c = try!(serde_json::from_str(&config));

        Ok(c)
    }

    /// Configuration als JSON codierterts String Result
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::*;
    ///
    /// let configuration = Configuration {
    ///     server: Server::new(),
    ///     sensors: vec![Sensor::new(SensorType::NemotoNO2), Sensor::new(SensorType::NemotoCO), Sensor::new(SensorType::NemotoNO2), Sensor::new(SensorType::NemotoCO), ]
    /// };
    ///
    /// println!("{}", configuration.to_json().unwrap());
    /// ```
    pub fn to_json(&self) -> Result<String> {
        let s: String = try!(serde_json::to_string_pretty(self));

        Ok(s)
    }
}
