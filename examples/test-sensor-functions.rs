extern crate xmz_mod_touch_server;

use xmz_mod_touch_server::{Kombisensor, KombisensorType, Zone, ZoneStatus};


fn main() {
    let mut zone = Zone::new();
    zone.add_kombisensor(Kombisensor::new_with_type(KombisensorType::RAGasSimulation));

    {
        for sensor in zone.get_kombisensor(0).unwrap().get_sensors() {
            println!("self.get_concentration_average_15min() {} >= self.alarm1_average_15min as f64 {}", sensor.get_concentration_average_15min(), sensor.alarm1_average_15min);
            println!("{:?}", sensor.get_sensor_type());
            println!("concentration: {}", sensor.get_concentration());
            println!("is_enabled?: {}", sensor.is_enabled());
            println!("adc_value_average_15min: {}", sensor.get_adc_value_average_15min());
            println!("adc_values_average: {:?}", sensor.adc_values_average);
            println!("AP1?: {:#?}", sensor.alarmpunkt1_reached());
            println!("AP2?: {:#?}", sensor.alarmpunkt2_reached());
            println!("DIW?: {:#?}", sensor.direct_value_reached());
            println!("ZoneStaus: {:?}", zone.get_status());
        }
    }
    // Ist eine Zone im Normal Status,
    assert_eq!(zone.get_status(), ZoneStatus::Normal);
    // und überschreitet ein Sensor, des Kombisensors
    // dieser Zone den Alarmpunkt1,
    zone.get_kombisensor_mut(0).unwrap().get_sensor_mut(1).unwrap().set_concentration(31.0); // Alarmpunkt1 bei 30 überschritten
    // dann ist nach einem Update,
    zone.update();

    {
        for sensor in zone.get_kombisensor(0).unwrap().get_sensors() {
            println!("{:?}", sensor.get_sensor_type());
            println!("concentration: {}", sensor.get_concentration());
            println!("is_enabled?: {}", sensor.is_enabled());
            println!("adc_value_average_15min: {}", sensor.get_adc_value_average_15min());
            println!("adc_values_average: {:?}", sensor.adc_values_average);
            println!("AP1?: {:#?}", sensor.alarmpunkt1_reached());
            println!("AP2?: {:#?}", sensor.alarmpunkt2_reached());
            println!("DIW?: {:#?}", sensor.direct_value_reached());
            println!("ZoneStaus: {:?}", zone.get_status());
        }
    }

    // der neue Status der Zone AP1
    assert_eq!(zone.get_status(), ZoneStatus::AP1);

}
