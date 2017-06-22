extern crate xmz_mod_touch_server;

use xmz_mod_touch_server::{Sensor, SensorType};

#[test]
fn basic() {
    let sensor = Sensor::new_with_type(SensorType::SimulationNO2Fix);

    assert_eq!(sensor.get_concentration(), 20.0);
}
