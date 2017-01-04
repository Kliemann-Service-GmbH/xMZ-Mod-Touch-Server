// use error::*;

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub serial_interface: String,
    pub baud: i32,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            serial_interface: "/dev/ttyS1".to_string(),
            baud: 9600,
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Server { ..Default::default() }
    }
}
