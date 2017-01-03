use error::*;
use sensor::Sensor;
use serde_json;
use server::Server;


#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub server: Server,
    pub sensors: Vec<Sensor>,
}

#[allow(dead_code)]
impl Configuration {
    pub fn from_config(config: String) -> Configuration {
        match serde_json::from_str(&config) {
            Ok(c) => c,
            Err(_) => Configuration { server: Server::new(), sensors: vec![] }
        }
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
