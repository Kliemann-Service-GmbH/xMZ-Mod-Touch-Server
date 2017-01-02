#[derive(Serialize, Deserialize, Debug)]
pub struct Sensor {
    adc_value: u32,
    adc_at_nullgas: u32,
    adc_at_messgas: u32,
    value_at_nullgas: u32,
    value_at_messgas: u32,
}

impl Sensor {
    pub fn new() -> Sensor {
        Sensor {
            adc_value: 0,
            adc_at_nullgas: 0,
            adc_at_messgas: 0,
            value_at_nullgas: 0,
            value_at_messgas: 0,
        }
    }
}
