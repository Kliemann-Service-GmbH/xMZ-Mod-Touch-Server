// Inspiration: http://cosmo0920.github.io/ruroonga_command/src/ruroonga_command/src/command.rs.html#17-63
use std::str::FromStr;

/// Mögliche Fehler die auftreten können
#[derive (Debug, Eq, PartialEq)]
pub enum ServerCommandError {
    Invalid,
}

/// Liste der Befehle die der Server verarbeiten kann_module_modbus_adresse_abfragen
///
#[derive (Debug, Eq, PartialEq)]
pub enum ServerCommand {
    Led { subcommand: String, params: String },
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
            _ => Err(ServerCommandError::Invalid),
        }
    }
}
