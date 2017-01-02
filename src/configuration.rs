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

    /// Configuration als JSON codierter String
    ///
    /// # Examples
    ///
    /// ```
    /// let configuration = Configuration {
    /// server: Server {
    /// serial_interface: "/dev/ttyS1".to_string(),
    /// baud: 9600,
    /// },
    /// sensors: vec![Sensor::new(), Sensor::new(), Sensor::new(), Sensor::new(), ]
    /// };
    /// println!("{}", configuration.as_str());
    /// ```
    pub fn as_str(&self) -> Result<String> {
        let s: String = try!(serde_json::to_string(self));

        Ok(s)
    }
}
