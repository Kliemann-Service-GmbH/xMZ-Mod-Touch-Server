extern crate nanomsg;
extern crate xmz_server;

use std::str::FromStr;
use std::time::Duration;
use xmz_server::server::server_command::{ServerCommand, ServerCommandError};
use xmz_server::server::server::{Server};
use std::error::Error;
use nanomsg::{Socket, Protocol};



fn run() -> Result<(), Box<Error + Send + Sync>> {
    let mut server = Server::new();
    let mut socket = try!(Socket::new(Protocol::Rep));

    // Liste alle möglichen Befehle
    let commands: Vec<_> = vec![
        try!(ServerCommand::from_str("led list")),
        try!(ServerCommand::from_str("led set 1")),
        try!(ServerCommand::from_str("led get 1")),
        try!(ServerCommand::from_str("led toggle 1")),
        try!(ServerCommand::from_str("led clear 1")),

        try!(ServerCommand::from_str("relais list")),
        try!(ServerCommand::from_str("relais set 1")),
        try!(ServerCommand::from_str("relais get 1")),
        try!(ServerCommand::from_str("relais toggle 1")),
        try!(ServerCommand::from_str("relais clear 1")),
    ];

    for command in commands {
        // println!("Call: {:?}", command);
        server.execute(command, &mut socket);
    }


    Ok(())
}


fn main() {
    match run() {
        Ok(_) => println!("OK"),
        Err(err) => println!("FEHLER: {}", err),
    }
}
