// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#![feature(proc_macro)]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

pub mod errors;
mod server;
pub mod system_commands;
mod configuration;

pub use self::configuration::*;
pub use self::server::*;
