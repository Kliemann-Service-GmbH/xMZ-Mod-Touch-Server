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
    fn from_config(&mut self, config_file: &'static str) {
        let _deserialized_configuration: Result<Configuration> = match serde_json::from_str(config_file) {
            Ok(deserialized_configuration) => {
                println!("{:#?}", deserialized_configuration);
                Ok(deserialized_configuration)
            },
            Err(err) => {
                println!("error: {}", err);
                Err(XMZError::Serde(err))
            },
        };

    }

    pub fn as_str(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
