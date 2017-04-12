#[derive(Debug)]
struct Sensor {
    value: u64,
    reverse: bool,   // Boolen um die Richtung bei der Simulation zu halten.
}
impl Sensor {
    fn new() -> Self {
        Sensor {
            value: 0,
            reverse: false,
        }
    }
    fn update(&mut self) {
        if self.value == 300 { self.reverse = true }
        if !self.reverse { self.value += 1 } else { self.value -= 1 }
        if self.value == 0 { self.reverse = false }
    }
    fn check(&mut self) {
        println!("Sensor checked!");
    }
}

#[derive(Debug)]
struct Kombisensor {
    sensors: Vec<Sensor>,
}
impl Kombisensor {
    fn new() -> Self {
        Kombisensor {
            sensors: vec![
                Sensor::new(),
                Sensor::new(),
            ]
        }
    }
    fn update(&mut self) {
        for mut sensor in &mut self.sensors {
            sensor.update();
        }
    }
    fn check(&mut self) {
        for mut sensor in &mut self.sensors {
            sensor.check();
        }
    }
}

#[derive(Debug)]
struct Zone {
    kombisensors: Vec<Kombisensor>,
}
impl Zone {
    fn new() -> Self {
        Zone {
            kombisensors: vec![
                Kombisensor::new(),
                Kombisensor::new(),
            ]
        }
    }
    fn update(&mut self) {
        for mut kombisensor in &mut self.kombisensors {
            kombisensor.update();
        }
    }
    fn check(&mut self) {
        for mut kombisensor in &mut self.kombisensors {
            kombisensor.check();
        }
    }
}

#[derive(Debug)]
struct Server {
    zones: Vec<Zone>,
}
impl Server {
    fn new() -> Self {
        Server {
            zones: vec![
                Zone::new(),
            ]
        }
    }
    fn update(&mut self) {
        for mut zone in &mut self.zones {
            zone.update();
        }
    }
    fn check(&mut self) {
        for mut zone in &mut self.zones {
            zone.check();
        }
    }
}



fn main() {
    let mut server = Server::new();
    // println!("{:#?}", server);

    loop {
        // Ausnahmen prüfen
        server.check();

        // Server Kombonenten aktualisieren, Kombisensoren auslesen, ....
        server.update();
        for zone in &server.zones {
            for kombisensor in &zone.kombisensors {
                for sensor in &kombisensor.sensors {
                    println!("\t{:?}", sensor);
                }
            }
        }
    }
}
