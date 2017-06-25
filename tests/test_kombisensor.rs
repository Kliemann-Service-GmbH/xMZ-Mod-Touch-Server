extern crate xmz_mod_touch_server;

use xmz_mod_touch_server::Kombisensor;

#[test]
fn basic() {
    let kombisensor = Kombisensor::new();

    assert_eq!(kombisensor.get_sensors().len(), 0);
}
