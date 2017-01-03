use co_no2_kombisensor::sensor::Sensor;

#[derive(Debug)]
pub struct Platine {
    version: String,
    modbus_address: u8,
    sensors: Vec<Sensor>,
}
