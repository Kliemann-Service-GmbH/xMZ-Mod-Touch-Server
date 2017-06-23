extern crate xmz_mod_touch_server;

use xmz_mod_touch_server::{Kombisensor, KombisensorType, Sensor, SensorType, Zone, ZoneStatus};


#[test]
fn zone_default_status() {
    let mut zone = Zone::new();
    zone.set_status(ZoneStatus::Normal);
}

#[test]
fn ein_kombisensor_ein_co_sensor_diw() {
    let mut zone = Zone::new();
    let mut kombisensor = Kombisensor::new();
    kombisensor.add_sensor(Sensor::new_with_type(SensorType::SimulationCO));
    zone.add_kombisensor(kombisensor);

    // Ist eine Zone im Normal Status,
    assert_eq!(zone.get_status(), ZoneStatus::Normal);
    // und überschreitet ein Sensor, des Kombisensors
    // dieser Zone den Direktwert DIW,
    zone.get_kombisensor_mut(0).unwrap().get_sensor_mut(0).unwrap().set_concentration(150.0); // DIW 150 überschritten
    // dann ist nach einem Update,
    zone.update();
    // der neue Status der Zone DIW
    assert_eq!(zone.get_status(), ZoneStatus::DIW);
}

#[test]
fn ein_kombisensor_ein_co_sensor_ap1() {
    let mut zone = Zone::new();
    let mut kombisensor = Kombisensor::new();
    kombisensor.add_sensor(Sensor::new_with_type(SensorType::SimulationCO));
    zone.add_kombisensor(kombisensor);

    // Ist eine Zone im Normal Status,
    assert_eq!(zone.get_status(), ZoneStatus::Normal);
    // und überschreitet ein Sensor, des Kombisensors
    // dieser Zone den Alarmpunkt1,
    zone.get_kombisensor_mut(0).unwrap().get_sensor_mut(0).unwrap().set_concentration(31.0); // Alarmpunkt1 bei 30 überschritten
    // dann ist nach einem Update,
    zone.update();
    // der neue Status der Zone AP1
    assert_eq!(zone.get_status(), ZoneStatus::AP1);
}

#[test]
fn ein_kombisensor_zwei_sensoren_no2_co_einer_ap1() {
    let mut zone = Zone::new();
    zone.add_kombisensor(Kombisensor::new_with_type(KombisensorType::RAGasSimulation));

    // Ist eine Zone im Normal Status,
    assert_eq!(zone.get_status(), ZoneStatus::Normal);
    // und überschreitet der erste Sensor, des Kombisensors
    // dieser Zone den Alarmpunkt1,
    zone.get_kombisensor_mut(0).unwrap().get_sensor_mut(1).unwrap().set_concentration(31.0); // Alarmpunkt1 bei 30 überschritten

    // dann ist nach einem Update,
    zone.update();
    // der neue Status der Zone AP1
    assert_eq!(zone.get_status(), ZoneStatus::AP1);
}

#[test]
fn ein_kombisensor_zwei_sensoren_no2_co_einer_ap1_einer_diw() {
    let mut zone = Zone::new();
    zone.add_kombisensor(Kombisensor::new_with_type(KombisensorType::RAGasSimulation));

    // Ist eine Zone im Normal Status,
    assert_eq!(zone.get_status(), ZoneStatus::Normal);
    // und überschreitet der erste Sensor, des Kombisensors
    // dieser Zone den Alarmpunkt1,
    zone.get_kombisensor_mut(0).unwrap().get_sensor_mut(0).unwrap().set_concentration(4.0); // Alarmpunkt1 NO2 bei 4 überschritten
    // und überschreitet der zweite Sensor, des Kombisensors
    // dieser Zone den Direktwert,
    zone.get_kombisensor_mut(0).unwrap().get_sensor_mut(1).unwrap().set_concentration(151.0); // DIW bei 150 überschritten

    // dann ist nach einem Update,
    zone.update();
    // der neue Status der Zone DIW
    assert_eq!(zone.get_status(), ZoneStatus::DIW);
}

#[test]
fn ein_kombisensor_zwei_sensoren_no2_co_einer_diw_einer_ap1() {
    // Dieser Test ist die gedrehte Version des vorheringen Tests. Der Sensor mit DIW überschritten
    // muss den ZoneStatus::DIW triggern. Der Sensor mit AP1 überschritten darf dann aber nicht
    // den ZoneStatus verändern.
    let mut zone = Zone::new();
    zone.add_kombisensor(Kombisensor::new_with_type(KombisensorType::RAGasSimulation));

    // Ist eine Zone im Normal Status,
    assert_eq!(zone.get_status(), ZoneStatus::Normal);

    // und überschreitet der erste Sensor, des Kombisensors
    // dieser Zone den Direktwert,
    zone.get_kombisensor_mut(0).unwrap().get_sensor_mut(0).unwrap().set_concentration(16.0); // Direktwert NO2 bei 15 überschritten
    // und überschreitet der zweite Sensor, des Kombisensors
    // dieser Zone den Alarmpunkt1,
    zone.get_kombisensor_mut(0).unwrap().get_sensor_mut(1).unwrap().set_concentration(31.0); // Alarmpunkt1 NO2 bei 4 überschritten

    // dann ist nach einem Update,
    zone.update();
    // der neue Status der Zone DIW
    assert_eq!(zone.get_status(), ZoneStatus::DIW);
}

// Zwei Kombisensoren
#[test]
fn zwei_kombisensoren_ein_sensor_diw() {
    let mut zone = Zone::new();
    let mut kombisensor1 = Kombisensor::new();
    let mut kombisensor2 = Kombisensor::new();
    kombisensor1.add_sensor(Sensor::new_with_type(SensorType::SimulationCO));
    kombisensor2.add_sensor(Sensor::new_with_type(SensorType::SimulationCO));
    zone.add_kombisensor(kombisensor1);
    zone.add_kombisensor(kombisensor2);

    // Ist eine Zone im Normal Status,
    assert_eq!(zone.get_status(), ZoneStatus::Normal);
    // und überschreitet ein Sensor, eines Kombisensors
    // dieser Zone den Direktwert DIW,
    zone.get_kombisensor_mut(1).unwrap().get_sensor_mut(0).unwrap().set_concentration(151.0); // DIW 150 überschritten
    // dann ist nach einem Update,
    zone.update();
    // der neue Status der Zone DIW
    assert_eq!(zone.get_status(), ZoneStatus::DIW);
}

#[test]
fn zwei_kombisensoren_zwei_sensoren_einer_ap1_einer_diw() {
    let mut zone = Zone::new();
    let mut kombisensor1 = Kombisensor::new();
    let mut kombisensor2 = Kombisensor::new();
    kombisensor1.add_sensor(Sensor::new_with_type(SensorType::SimulationCO));
    kombisensor2.add_sensor(Sensor::new_with_type(SensorType::SimulationCO));
    zone.add_kombisensor(kombisensor1);
    zone.add_kombisensor(kombisensor2);

    // Ist eine Zone im Normal Status,
    assert_eq!(zone.get_status(), ZoneStatus::Normal);
    // und überschreitet der erste Sensor, des Kombisensors
    // dieser Zone den Alarmpunkt1,
    zone.get_kombisensor_mut(0).unwrap().get_sensor_mut(0).unwrap().set_concentration(31.0); // Alarmpunkt1 bei 30 überschritten
    // und überschreitet der zweite Sensor, des Kombisensors
    // dieser Zone den Direktwert,
    zone.get_kombisensor_mut(1).unwrap().get_sensor_mut(0).unwrap().set_concentration(151.0); // DIW bei 150 überschritten

    // dann ist nach einem Update,
    zone.update();
    // der neue Status der Zone DIW
    assert_eq!(zone.get_status(), ZoneStatus::DIW);
}

#[test]
fn zwei_kombisensoren_zwei_sensoren_einer_diw_einer_ap1() {
    // Dieser Test ist die gedrehte Version des vorheringen Tests. Der Sensor mit DIW überschritten
    // muss den ZoneStatus::DIW triggern. Der Sensor mit AP1 überschritten darf dann aber nicht
    // den ZoneStatus verändern.
    let mut zone = Zone::new();
    let mut kombisensor1 = Kombisensor::new();
    let mut kombisensor2 = Kombisensor::new();
    kombisensor1.add_sensor(Sensor::new_with_type(SensorType::SimulationCO));
    kombisensor2.add_sensor(Sensor::new_with_type(SensorType::SimulationCO));
    zone.add_kombisensor(kombisensor1);
    zone.add_kombisensor(kombisensor2);

    // Ist eine Zone im Normal Status,
    assert_eq!(zone.get_status(), ZoneStatus::Normal);
    // und überschreitet der erste Sensor, des Kombisensors
    // dieser Zone den Direktwert,
    zone.get_kombisensor_mut(0).unwrap().get_sensor_mut(0).unwrap().set_concentration(151.0); // Direktwert bei 150 überschritten
    // und überschreitet der zweite Sensor, des Kombisensors
    // dieser Zone den Alarmpunkt1,
    zone.get_kombisensor_mut(1).unwrap().get_sensor_mut(0).unwrap().set_concentration(31.0); // Alarmpunkt1 bei 30 überschritten

    // dann ist nach einem Update,
    zone.update();
    // der neue Status der Zone DIW
    assert_eq!(zone.get_status(), ZoneStatus::DIW);
}

// Nicht halten Funktion testen

#[test]
fn zone_in_status_diw_ein_kombisensor_ohne_messwert() {
    // Dieser Test soll das "nicht Halten" des ZonenStatus testen.
    // War eine Zone im Status DIW, und sind die Sensoren alle ohne überhöhten Messwert,
    // dann muss der ZonenStatus wieder auf Normal "zurück fallen"
    let mut zone = Zone::new();
    zone.add_kombisensor(Kombisensor::new_with_type(KombisensorType::RAGasSimulation));

    // Ist eine Zone im DIW Status,
    zone.set_status(ZoneStatus::DIW);
    assert_eq!(zone.get_status(), ZoneStatus::DIW);

    // und kein Sensor der Zone überschreitet einen Wert
    zone.get_kombisensor_mut(0).unwrap().get_sensor_mut(0).unwrap().set_concentration(0.0); // NO2 Sensor, keine Messwertüberschreitung
    zone.get_kombisensor_mut(0).unwrap().get_sensor_mut(1).unwrap().set_concentration(0.0); // CO Sensor, keine Messwertüberschreitung

    // dann ist nach einem Update,
    zone.update();
    // der neue Status der Zone Normal
    assert_eq!(zone.get_status(), ZoneStatus::Normal);
}

#[test]
fn zone_in_status_diw_ein_kombisensor_ein_sensor_ap2() {
    let mut zone = Zone::new();
    zone.add_kombisensor(Kombisensor::new_with_type(KombisensorType::RAGasSimulation));
    zone.set_status(ZoneStatus::DIW);

    // Ist eine Zone im DIW Status,
    zone.set_status(ZoneStatus::DIW);
    assert_eq!(zone.get_status(), ZoneStatus::DIW);

    // und ein Sensor der Zone überschreitet den AP2 Wert
    zone.get_kombisensor_mut(0).unwrap().get_sensor_mut(0).unwrap().set_concentration(0.0); // NO2 Sensor, keine Messwertüberschreitung
    zone.get_kombisensor_mut(0).unwrap().get_sensor_mut(1).unwrap().set_concentration(61.0); // AP2 bei 60 überschritten

    // dann ist nach einem Update,
    zone.update();
    // der neue Status der Zone Normal
    assert_eq!(zone.get_status(), ZoneStatus::AP2);
}

#[test]
fn zone_in_status_ap2_ein_kombisensor_ein_sensor_diw() {
    let mut zone = Zone::new();
    zone.add_kombisensor(Kombisensor::new_with_type(KombisensorType::RAGasSimulation));
    zone.set_status(ZoneStatus::AP2);

    // Ist eine Zone im AP2 Status,
    zone.set_status(ZoneStatus::AP2);
    assert_eq!(zone.get_status(), ZoneStatus::AP2);

    // und ein Sensor der Zone überschreitet den DIW Wert
    zone.get_kombisensor_mut(0).unwrap().get_sensor_mut(1).unwrap().set_concentration(151.0); // DIW bei 150 überschritten

    // dann ist nach einem Update,
    zone.update();
    // der neue Status der Zone Normal
    assert_eq!(zone.get_status(), ZoneStatus::DIW);
}
