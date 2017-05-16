#[derive(Debug,PartialEq,Eq)]
pub struct ModbusData {
    firmware_version_major: u16,
    firmware_version_minor: u16,
    firmware_version_patch: u16,
}

pub const SIMMULATION: &[u16] = &[0, 14, 0, 247, 0, 0, 0, 0, 0, 0, 1, 923, 0, 30, 920, 564, 0, 20, 1, 0, 2, 107, 0, 300, 112, 760, 0, 270, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

fn parse(input: &[u16]) -> Result<ModbusData, ::std::io::Error> {

    let firmware_version_major = input[0];
    let firmware_version_minor = input[1];
    let firmware_version_patch = input[2];

    let modbus_data = ModbusData {
        firmware_version_major: firmware_version_major,
        firmware_version_minor: firmware_version_minor,
        firmware_version_patch: firmware_version_patch,
    };

    Ok(modbus_data)
}


fn main() {
    let sim = SIMMULATION;
    let modbus_data = parse(&sim);

    println!("{:#?}", modbus_data);
}
