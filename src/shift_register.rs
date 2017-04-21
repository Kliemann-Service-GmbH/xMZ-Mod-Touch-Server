#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum ShiftRegisterType {
    LED,
    Relais,
}
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ShiftRegister {
    shift_register_type: ShiftRegisterType,
    data: u64,
}
impl ShiftRegister {
    pub fn new(shift_register_type: ShiftRegisterType) -> Self {
        match shift_register_type {
            ShiftRegisterType::LED | ShiftRegisterType::Relais => {
                ShiftRegister {
                    shift_register_type: shift_register_type,
                    data: 0,
                }
            }
        }
    }
    pub fn set(&mut self, num: usize) {
        debug!("{:?}: SET Pin: {}", self.shift_register_type, num);
        self.data |= 1 << (num - 1);
    }
    pub fn clear(&mut self, num: usize) {
        debug!("{:?}: CLEAR Pin: {}", self.shift_register_type, num);
        self.data &= !(1 << (num - 1));
    }
}
