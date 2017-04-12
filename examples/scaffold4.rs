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
        // println!("Sensor checked!");
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
enum ShiftRegisterType {
    LED,
    Relais,
}
#[derive(Debug)]
struct ShiftRegister {
    shift_register_type: ShiftRegisterType,
    data: u64,
}
impl ShiftRegister {
    fn new(shift_register_type: ShiftRegisterType) -> Self {
        match shift_register_type {
            ShiftRegisterType::LED => ShiftRegister { shift_register_type: shift_register_type, data: 0 },
            ShiftRegisterType::Relais => ShiftRegister { shift_register_type: shift_register_type, data: 0 },
        }
    }
    fn set(&mut self, num: u64) {
        println!("{:?}: SET Pin: {}", self.shift_register_type, num);
    }
    fn clear(&mut self, num: u64) {
        println!("{:?}: CLEAR Pin: {}", self.shift_register_type, num);
    }
}

#[derive(Debug)]
struct Server {
    zones: Vec<Zone>,
    leds: ShiftRegister,
    relais: ShiftRegister,
}
impl Server {
    fn new() -> Self {
        Server {
            zones: vec![
                Zone::new(),
            ],
            leds: ShiftRegister::new(ShiftRegisterType::LED),
            relais: ShiftRegister::new(ShiftRegisterType::Relais),
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


#[derive(Debug)]
struct Exception;
struct Action<'a> {
    action: fn(&mut ShiftRegister, u64),
    pin: u64,
    shift_register: &'a mut ShiftRegister,
}
impl<'a> Action<'a> {
    fn execute(&mut self) {
        (self.action)(self.shift_register, self.pin);
    }
}


fn main() {
    let mut server = Server::new();

    // Exeption {
    //     Server::power_failure,
    //     actions: vec![
    //         Action {
    //             action: ShiftRegister::set,
    //             pin: 1,
    //             shift_register: &mut server.leds,
    //         },
    //         Action {
    //             action: ShiftRegister::clear,
    //             pin: 1,
    //             shift_register: &mut server.relais,
    //         }
    //     ]
    // }

    let mut action1 = Action {
        action: ShiftRegister::set,
        pin: 1,
        shift_register: &mut server.leds,
    };
    action1.execute();
    let mut action2 = Action {
        action: ShiftRegister::clear,
        pin: 1,
        shift_register: &mut server.relais,
    };
    action2.execute();


    std::process::exit(1);


    println!("{:#?}", server);

    loop {
        // Ausnahmen prüfen
        server.check();

        // Server Kombonenten aktualisieren, Kombisensoren auslesen, ....
        server.update();
        for zone in &server.zones {
            for kombisensor in &zone.kombisensors {
                for sensor in &kombisensor.sensors {
                    // println!("\t{:?}", sensor);
                }
            }
        }
    }
}
