//! # Exceptions (Ausnahmen)
//!


#[derive(Debug)]
#[derive(Eq, PartialEq, Hash)]
#[derive(Clone)]
pub struct Action;
impl Action {
    pub fn new() -> Self {
        Action
    }
}

#[derive(Debug)]
#[derive(Eq, PartialEq, Hash)]
#[derive(Clone)]
pub enum ExceptionType {
    /// Wartungsintervall erreicht
    Wartungsintervall,
    /// Kabelbruch (Zone Nummer)
    Kabelbruch,
}

#[derive(Debug)]
#[derive(Eq, PartialEq, Hash)]
#[derive(Clone)]
pub struct Exception {
    actions: Vec<Action>,
    exception_type: ExceptionType,
}

impl Exception {
    pub fn new(exception_type: ExceptionType, actions: Vec<Action>) -> Self {
        match exception_type {
            ExceptionType::Wartungsintervall => Exception {
                actions: actions,
                exception_type: exception_type,
            },
            ExceptionType::Kabelbruch => Exception {
                actions: actions,
                exception_type: exception_type,
            },
        }
    }
}
