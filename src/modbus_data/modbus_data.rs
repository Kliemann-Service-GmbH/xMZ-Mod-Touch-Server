use errors::*;
use libmodbus_rs::{Modbus, ModbusRTU, ModbusClient, MODBUS_RTU_MAX_ADU_LENGTH};


/// Beispieldaten: 256 u16 values
/// Firmware Version: 0.14.0
/// Modbus Adresse: 247
pub const SIMULATION_DATA_STATIC: &[u16] = &[0, 14, 0, 247, 0, 0, 0, 0, 0, 0, 1, 923, 0, 30, 920, 564, 0, 20, 1, 0, 2, 107, 0, 300, 112, 760, 0, 270, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];


#[derive(Debug)]
#[derive(PartialEq,Eq)]
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

impl ModbusData {
    pub fn new() -> Self {

    }
}

fn get_from_modbus() -> Result<Vec<u16>> {
    let device: String = std::env::args().nth(1).unwrap_or("/dev/ttyUSB0".to_string());
    let slave_id: u8 = std::env::args().nth(2).unwrap_or("247".to_string()).parse()?;

    let mut modbus = Modbus::new_rtu(&device, 9600, 'N', 8, 1)?;
    modbus.set_slave(247)?;

    modbus.set_debug(true);
    modbus.connect()?;

    let mut response_register = vec![0u16; MODBUS_RTU_MAX_ADU_LENGTH as usize];
    modbus.read_registers(0, 30, &mut response_register)?;

    Ok(response_register)
}

fn parse(input: &[u16]) -> Result<ModbusData> {
    // Check input
    println!("{}", input.len());
    if input.len() < 28 { panic!("Modbus Data invalid") }

    let firmware_version_major = *input.get(0).unwrap();
    let firmware_version_minor = *input.get(1).unwrap();
    let firmware_version_patch = *input.get(2).unwrap();
    let modbus_address = *input.get(3).unwrap();
    let sensor1_num = *input.get(0).unwrap();
    let sensor1_adc_value = *input.get(1).unwrap();
    let sensor1_min_value = *input.get(2).unwrap();
    let sensor1_max_value = *input.get(3).unwrap();
    let sensor1_adc_value_at_nullgas = *input.get(4).unwrap();
    let sensor1_adc_value_at_messgas = *input.get(5).unwrap();
    let sensor1_concentration_at_nullgas = *input.get(6).unwrap();
    let sensor1_concentration_at_messgas = *input.get(7).unwrap();
    let sensor1_configuration_bits = *input.get(8).unwrap();
    let sensor2_num = *input.get(0).unwrap();
    let sensor2_adc_value = *input.get(1).unwrap();
    let sensor2_min_value = *input.get(2).unwrap();
    let sensor2_max_value = *input.get(3).unwrap();
    let sensor2_adc_value_at_nullgas = *input.get(4).unwrap();
    let sensor2_adc_value_at_messgas = *input.get(5).unwrap();
    let sensor2_concentration_at_nullgas = *input.get(6).unwrap();
    let sensor2_concentration_at_messgas = *input.get(7).unwrap();
    let sensor2_configuration_bits = *input.get(8).unwrap();

    let modbus_data = ModbusData {
        firmware_version_major: firmware_version_major,
        firmware_version_minor: firmware_version_minor,
        firmware_version_patch: firmware_version_patch,
        modbus_address: modbus_address,
        sensor1_num: sensor1_num,
        sensor1_adc_value: sensor1_adc_value,
        sensor1_min_value: sensor1_min_value,
        sensor1_max_value: sensor1_max_value,
        sensor1_adc_value_at_nullgas: sensor1_adc_value_at_nullgas,
        sensor1_adc_value_at_messgas: sensor1_adc_value_at_messgas,
        sensor1_concentration_at_nullgas: sensor1_concentration_at_nullgas,
        sensor1_concentration_at_messgas: sensor1_concentration_at_messgas,
        sensor1_configuration_bits: sensor1_configuration_bits,
        sensor2_num: sensor2_num,
        sensor2_adc_value: sensor2_adc_value,
        sensor2_min_value: sensor2_min_value,
        sensor2_max_value: sensor2_max_value,
        sensor2_adc_value_at_nullgas: sensor2_adc_value_at_nullgas,
        sensor2_adc_value_at_messgas: sensor2_adc_value_at_messgas,
        sensor2_concentration_at_nullgas: sensor2_concentration_at_nullgas,
        sensor2_concentration_at_messgas: sensor2_concentration_at_messgas,
        sensor2_configuration_bits: sensor2_configuration_bits,
    };

    Ok(modbus_data)
}
