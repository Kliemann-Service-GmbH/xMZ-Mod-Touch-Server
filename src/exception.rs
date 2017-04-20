/// Exception
///
/// Playground URL: https://play.rust-lang.org/?gist=6c0100d86a96f116615f43389f7b8af6&version=nightly&backtrace=0
/// Gist URL: https://gist.github.com/6c0100d86a96f116615f43389f7b8af6
///
#[derive(Debug)]
#[derive(Hash, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum ExceptionType {
    WartungsintervalReached,
    KombisensorOffline { zone: usize },
    SensorDirectValue { zone: usize, sensor: usize },
}

#[derive(Debug)]
#[derive(Hash, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Exception {
    exception_type: ExceptionType,
}
impl Exception {
    pub fn new(exception_type: ExceptionType) -> Self {
        Exception {
            exception_type: exception_type,
        }
    }
}