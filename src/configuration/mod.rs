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
    #[serde(default)]
    pub server: Server,
    #[serde(default)]
    pub kombisensors: Vec<Kombisensor>,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            server: Server::new(),
            kombisensors: vec![],
        }
    }
}

/// Configuration des Servers
///
/// Die Konfiguration wird nur aus der JSON Datei generiert. **Deshalb ist eine `new()` Funktion
/// nicht nötig.**
/// Für ein Beispiel einer manuellen Initalisierung siehe `examples/configuration_to_json.rs`
impl Configuration {
    /// Erzeugt eine Configuration Datenstructur aus einem JSON Codierten String
    ///
    /// Der String wird im Normalfall aus eine .json Datei gebildet.
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
    ///     kombisensors: vec![Kombisensor::new(), Kombisensor::new()]
    /// };
    ///
    /// println!("{}", configuration.to_json().unwrap());
    /// ```
    pub fn to_json(&self) -> Result<String> {
        let s: String = try!(serde_json::to_string_pretty(self));

        Ok(s)
    }

    // Getter

    /// Liefert die Server Konfiguration
    pub fn get_server(&self) -> Server {
        self.server.clone()
    }

    /// Liefert die Kombisensoren
    pub fn get_kombisensors(&self) -> Vec<Kombisensor> {
        self.kombisensors.clone()
    }

}
