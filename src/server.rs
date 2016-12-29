
use error::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    serial_interface: String,
    baud: i32,
}

impl Server {
    pub fn new() -> Server {
        Server {
            serial_interface: "/dev/ttyS1".to_string(),
            baud: 9600,
        }
    }
}
