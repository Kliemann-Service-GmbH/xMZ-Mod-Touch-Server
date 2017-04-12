
#[derive(Clone)]
#[derive(Debug)]
pub enum ShiftRegisterType {
    LED,
    Relais,
}

#[derive(Clone)]
#[derive(Debug)]
pub struct ShiftRegister {
    shift_register_type: ShiftRegisterType,
}

impl ShiftRegister {
    pub fn new(shift_register_type: ShiftRegisterType) -> Self {
        match shift_register_type {
            ShiftRegisterType::LED      => ShiftRegister { shift_register_type: shift_register_type },
            ShiftRegisterType::Relais   => ShiftRegister { shift_register_type: shift_register_type },
        }
    }
}
