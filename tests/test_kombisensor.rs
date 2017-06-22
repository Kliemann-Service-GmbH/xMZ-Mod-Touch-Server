extern crate xmz_mod_touch_server;

use xmz_mod_touch_server::Server;

/// 256 u16 values
pub const SIMULATION_DATA_STATIC: &[u16] = &[0, 14, 0, 247, 0, 0, 0, 0, 0, 0, 1, 923, 0, 30, 920, 564, 0, 20, 1, 0, 2, 107, 0, 300, 112, 760, 0, 270, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];


#[derive(Debug,PartialEq,Eq)]
pub struct ModbusData {
    firmware_version_major: u16,
    firmware_version_minor: u16,
    firmware_version_patch: u16,
    modbus_address: u16,

    sensor1_num: u16,
    sensor1_adc_value: u16,
    sensor1_min_value: u16,
    sensor1_max_value: u16,
    sensor1_adc_value_at_nullgas: u16,
    sensor1_adc_value_at_messgas: u16,
    sensor1_concentration_at_nullgas: u16,
    sensor1_concentration_at_messgas: u16,
    sensor1_configuration_bits: u16,

    sensor2_num: u16,
    sensor2_adc_value: u16,
    sensor2_min_value: u16,
    sensor2_max_value: u16,
    sensor2_adc_value_at_nullgas: u16,
    sensor2_adc_value_at_messgas: u16,
    sensor2_concentration_at_nullgas: u16,
    sensor2_concentration_at_messgas: u16,
    sensor2_configuration_bits: u16,
}


#[test]
fn basic() {
    let server = Server::new();

    assert_eq!(server.get_zones().len(), 0);
}
