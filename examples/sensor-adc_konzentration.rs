#[macro_use] extern crate log;
extern crate env_logger;
extern crate xmz_server;
use xmz_server::sensor::{Sensor, SensorType};

fn main() {
    trace!("Initialisiere den Logger");
    env_logger::init().unwrap();

    let mut sensor = Sensor::new(SensorType::NemotoCO);
    sensor.list_all_concentrations();
}
