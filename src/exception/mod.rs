//! Exceptions Ausnahmen/ Fehlerbehandlung
//!
//!
//! Playground URL: https://play.rust-lang.org/?gist=6c0100d86a96f116615f43389f7b8af6&version=nightly&backtrace=0
//! Gist URL: https://gist.github.com/6c0100d86a96f116615f43389f7b8af6
//!
mod action;
mod exception;

pub use self::action::Action;
pub use self::exception::{Exception, ExceptionType};
