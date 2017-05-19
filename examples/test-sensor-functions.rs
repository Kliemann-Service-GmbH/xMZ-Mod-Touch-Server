extern crate xmz_mod_touch_server;

use xmz_mod_touch_server::kombisensor::{Sensor, SensorType};


fn main() {
    let mut sensor = Sensor::new_with_type(SensorType::SimulationNO2Fix);

    for i in 0..1024 {
        sensor.set_adc_value(i);
        println!("ADC: {}, Konzentration: {:.02} ppm", i, sensor.get_concentration());
    }

    println!("{:#?}", sensor);
}
