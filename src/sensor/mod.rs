use server::zone::Zone;

#[derive(Debug,Eq,PartialEq)]
pub enum SensorType {
    NemotoCO,
    NemotoNO,
}

#[derive(Debug,Eq,PartialEq)]
enum Alarmauswertung {
    On,
    Off,
    Simulation,
}

pub struct Sensor<'a> {
    pub sensor_type: SensorType,
    adc_value: Option<u32>,
    modbus_register_address: u32,
    alarmauswertung: Alarmauswertung,
    zones: Vec<&'a Zone>,
}

impl<'a> Sensor<'a> {
    pub fn new(sensor_type: SensorType) -> Self {
        match sensor_type {
            SensorType::NemotoCO => {
                Sensor {
                    sensor_type: SensorType::NemotoCO,
                    adc_value: None,
                    modbus_register_address: 1,
                    alarmauswertung: Alarmauswertung::Simulation,
                    zones: vec!(),
                }
            },
            SensorType::NemotoNO => {
                Sensor {
                    sensor_type: SensorType::NemotoNO,
                    adc_value: None,
                    modbus_register_address: 2,
                    alarmauswertung: Alarmauswertung::Simulation,
                    zones: vec!(),
                }
            },
        }
    }

}
#[cfg(test)]
mod test {
    use server::Server;
    use server::zone::Zone;
    use sensor::{Alarmauswertung, Sensor, SensorType};

    #[test]
    fn modbus_register_address_nemoto_co() {
        let sensor = Sensor::new(SensorType::NemotoCO);
        assert_eq!(sensor.modbus_register_address, 1);
    }

    #[test]
    fn modbus_register_adresse_nemoto_no() {
        let sensor = Sensor::new(SensorType::NemotoNO);
        assert_eq!(sensor.modbus_register_address, 2);
    }

    #[test]
    fn alarmauswertung() {
        let sensor1 = Sensor::new(SensorType::NemotoCO);
        let sensor2 = Sensor::new(SensorType::NemotoNO);
        assert_eq!(sensor1.alarmauswertung, Alarmauswertung::Simulation);
        assert_eq!(sensor2.alarmauswertung, Alarmauswertung::Simulation);
    }

    #[test]
    fn sensor_ohne_zone() {
        let sensor = Sensor::new(SensorType::NemotoCO);
        assert_eq!(sensor.zones.len(), 0);
    }

    #[test]
    fn sensor_mit_einer_zone() {
        let sensor = Sensor::new(SensorType::NemotoCO);
        let server = Server::new();
    }

    #[test]
    fn sensor_mit_mehr_als_einer_zone() {
        let sensor = Sensor::new(SensorType::NemotoCO);

    }
}
