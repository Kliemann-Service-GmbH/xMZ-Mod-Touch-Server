// Inspiration: http://cosmo0920.github.io/ruroonga_command/src/ruroonga_command/src/command.rs.html#17-63
use std::str::FromStr;

/// Mögliche Fehler die auftreten können
#[derive (Debug, Eq, PartialEq)]
pub enum ServerCommandError {
    InvalidCommand,
    InvalidSubCommand,
}

/// Liste der Befehle die der Server verarbeiten kann
///
#[derive (Debug, Eq, PartialEq)]
pub enum ServerCommand {
    Led { subcommand: String, params: String },
}

impl ServerCommand {
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use xmz_server::server::server_command::{ServerCommand, ServerCommandError};
    ///
    /// let string = "led set 1";
    /// assert_eq!(ServerCommand::from_str(string).unwrap().execute(), Ok(0));
    /// let string = "led get 1";
    /// assert_eq!(ServerCommand::from_str(string).unwrap().execute(), Ok(0));
    /// ```
    ///
    /// Mit ungültigem SubCommand
    ///
    /// ```
    /// use std::str::FromStr;
    /// use xmz_server::server::server_command::{ServerCommand, ServerCommandError};
    ///
    /// let string = "led foo 1";
    /// assert_eq!(ServerCommand::from_str(string).unwrap().execute(), Err(ServerCommandError::InvalidSubCommand));
    /// ```
    pub fn execute(self) -> Result<i32, ServerCommandError> {
        match self {
            ServerCommand::Led { subcommand, params, ..} => {
                match subcommand.as_ref() {
                    "set" => {
                        // server.led.set(1);
                        println!("server.led.{}({});", subcommand, params);
                        Ok(0)
                    },
                    "get" => {
                        // server.led.get(1);
                        println!("server.led.{}({});", subcommand, params);
                        Ok(0)
                    },
                    "clear" => { unimplemented!() },
                    "toggle" => { unimplemented!() },
                    _ => { Err(ServerCommandError::InvalidSubCommand) },
                }
            }
        }
    }
}

impl FromStr for ServerCommand {
    type Err = ServerCommandError;

    /// Generiert ein ServerCommand aus einem String
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use xmz_server::server::server_command::{ServerCommand, ServerCommandError};
    ///
    /// let string = "led set 1";
    ///
    /// assert_eq!(ServerCommand::from_str(string), Ok(ServerCommand::Led{ subcommand: "set".to_string(), params: "1".to_string()}));
    /// ```
    ///
    fn from_str(s: &str) -> Result<ServerCommand, ServerCommandError> {
        let v: Vec<_> = s.split_whitespace().collect();

        match v[0] {
            "led" => Ok(ServerCommand::Led {subcommand: String::from(v[1]), params: String::from(v[2])}),
            _ => Err(ServerCommandError::InvalidCommand),
        }
    }
}
