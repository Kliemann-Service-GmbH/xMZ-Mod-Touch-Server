extern crate xmz_server;
use xmz_server::sensor::{Sensor, SensorType};

fn main() {
    let mut sensor = Sensor::new(SensorType::NemotoCO);
    sensor.list_all_concentrations();
}
