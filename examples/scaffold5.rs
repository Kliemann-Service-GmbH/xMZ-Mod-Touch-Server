use std::cell::RefCell;
use std::rc::Rc;


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

    fn is_online(&self) -> bool {
        false
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
    leds: Rc<RefCell<ShiftRegister>>,
    relais: Rc<RefCell<ShiftRegister>>,
}
impl Server {
    fn new() -> Self {
        Server {
            zones: vec![
                Zone::new(),
            ],
            leds: Rc::new(RefCell::new(ShiftRegister::new(ShiftRegisterType::LED))),
            relais: Rc::new(RefCell::new(ShiftRegister::new(ShiftRegisterType::Relais))),
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
    fn uptime(&self) -> u64 {
        365
    }
}

trait HasException {}
impl HasException for Server {}
impl HasException for Kombisensor {}
struct Exception<'a, S, F, H>
    where   S: PartialEq,
            F: Fn(&H) -> S,
            H: HasException + 'a,
{
    source: F,
    threshold: S,
    target: &'a H,
    actions: Vec<Box<Action<'a>>>,
}
impl<'a, S, F, H> Exception<'a, S, F, H>
    where   S: PartialEq,
            F: Fn(&H) -> S,
            H: HasException + 'a,
{
    /// Erstellt eine neue Ausnahme
    ///
    /// ```
    ///  let wartungsintervall_erreicht = Exception {
    ///     source: Server::uptime,
    ///     threshold: 365,
    ///     target: &server,
    ///     actions: vec![
    ///         Box::new(Action::new(ShiftRegister::set, 3, &server.leds)),
    ///         Box::new(Action::new(ShiftRegister::clear, 1, &server.relais)),
    ///     ],
    /// };
    /// ```
    fn new(source: F, threshold: S, target: &'a H, actions: Vec<Box<Action<'a>>>) -> Self {
        Exception {
            source: source,
            threshold: threshold,
            target: target,
            actions: actions,
        }
    }

    fn auswerten(&self) {
        if (self.source)(self.target) == self.threshold {
            for action in &self.actions {
                action.execute();
            }
        }
    }
}

struct Action<'a> {
    action: fn(&mut ShiftRegister, u64),
    pin: u64,
    shift_register: &'a Rc<RefCell<ShiftRegister>>,
}
impl<'a> Action<'a> {
    /// Erstellt eine neue Action
    /// ```
    /// let mut action1 = Action::new(ShiftRegister::set, 1, &server.leds);
    /// let mut action2 = Action::new(ShiftRegister::clear, 1, &server.relais);
    /// let actions = vec![Box::new(action1), Box::new(action2)];
    /// for mut action in actions {
    ///     action.execute();
    /// }
    /// ```
    fn new(action: fn(&mut ShiftRegister, u64), pin: u64, shift_register: &'a Rc<RefCell<ShiftRegister>>) -> Self {
        Action {
            action: action,
            pin: pin,
            shift_register: shift_register,
        }
    }
    fn execute(&self) {
        (self.action)(&mut *self.shift_register.borrow_mut(), self.pin);
    }
}


fn main() {
    let mut server = Server::new();
    // println!("{:#?}", server);


    let wartungsintervall_erreicht = Exception {
        source: Server::uptime,
        threshold: 365,
        target: &server,
        actions: vec![
            Box::new(Action::new(ShiftRegister::set, 3, &server.leds)),
            Box::new(Action::new(ShiftRegister::clear, 1, &server.relais)),
        ],
    };
    wartungsintervall_erreicht.auswerten();
    let zone1_kombisensor1_kabelbruch = Exception {
        source: Kombisensor::is_online,
        threshold: false,
        target: &server.zones[0].kombisensors[0],
        actions: vec![
            Box::new(Action::new(ShiftRegister::set, 2, &server.leds)),
            Box::new(Action::new(ShiftRegister::clear, 1, &server.relais)),
        ],
    };
    zone1_kombisensor1_kabelbruch.auswerten();



    std::process::exit(1);




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
