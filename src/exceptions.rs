
#[derive(Debug)]
#[derive(Eq, PartialEq, Hash)]
pub enum ExceptionType {
    Wartungsintervall,
    Kabelbruch (usize),
}

#[derive(Debug)]
#[derive(Eq, PartialEq, Hash)]
pub struct Exception {
    exception_type: ExceptionType,
}

impl Exception {
    pub fn new(exception_type: ExceptionType) -> Self {
        match exception_type {
            ExceptionType::Wartungsintervall => Exception { exception_type: exception_type },
            ExceptionType::Kabelbruch (zone, ..) => Exception { exception_type: exception_type },
        }
    }
}
