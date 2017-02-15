extern crate xmz_server;

use std::thread;
use std::time::{Duration, Instant};
use xmz_server::*;

fn main() {
    let mut sensor_co = Sensor::new_with_type(SensorType::SimulationCO);
    // Sensor kalibrieren
    sensor_co.set_adc_value_at_nullgas(114);
    sensor_co.set_adc_value_at_messgas(875);
    sensor_co.set_concentration_at_nullgas(0);
    sensor_co.set_concentration_at_messgas(280);
    sensor_co.set_adc_value(0);

    let start = Instant::now();

    // Simulation
    //
    // 1h Loop
    while start.elapsed() < Duration::from_secs(360) {
        // 30 Sekunden Nullgas
        if start.elapsed() > Duration::from_secs(35) && start.elapsed() < Duration::from_secs(50) {
            sensor_co.set_adc_from_concentration(900.0);
        } else {
            sensor_co.set_adc_from_concentration(0.0);
        }

        sensor_co.update_adc_values_time();
        let average = sensor_co.average(Duration::from_secs(30));
        let weighten_average = sensor_co.weighten_average(Duration::from_secs(30));

        // Status line
        println!("Laufzeit: {}s,\nADC: {:?}, {:.02}ppm, gespeicherte Werte: {:?},",
            start.elapsed().as_secs(),
            sensor_co.get_adc_value(),
            sensor_co.get_concentration(),
            sensor_co.get_adc_values_time().len(),
        );
        println!("Ø ADC: {:04?}, Ø {:.02}ppm", &average, sensor_co.get_concentration_from_adc(&average));
        println!("Ø ADC: {:04?}, Ø {:.02}ppm gewichtet!", &weighten_average, sensor_co.get_concentration_from_adc(&weighten_average));
        for &(adc_value, _) in sensor_co.get_adc_values_time() {
            print!("{:04?} ", adc_value);
        }
        println!("\n\n");

        thread::sleep(Duration::from_millis(1000));
    }

    println!("Testlauf, E N D E!")
}
