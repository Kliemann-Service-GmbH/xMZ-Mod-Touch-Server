// Inspiration: http://cosmo0920.github.io/ruroonga_command/src/ruroonga_command/src/command.rs.html#17-63
use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// Mögliche Fehler die auftreten können
#[derive (Debug, Eq, PartialEq)]
pub enum ServerCommandError {
    InvalidCommand,
    InvalidSubCommand,
}

/// Liste der Befehle die der Server verarbeiten kann
///
#[derive (Clone, Debug, Eq, PartialEq)]
pub enum ServerCommand {
    Led { subcommand: String, params: String },
    Relais { subcommand: String, params: String },
    Server { subcommand: String, config_entry: String, config_value: Option<String> },
    Module { subcommand: String, config_entry: Option<String>, config_value: Option<String>, module_num: Option<String> }
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
            "led" => Ok(
                ServerCommand::Led {
                    subcommand: String::from(v[1]),
                    params: String::from(v[2])
                }),
            "relais" => Ok(
                ServerCommand::Relais {
                    subcommand: String::from(v[1]),
                    params: String::from(v[2])
                }),
            "server" => Ok(
                ServerCommand::Server {
                    subcommand: String::from(v[1]),
                    config_entry: String::from(v[2]),
                    config_value: match v.get(3) {
                        Some(x) => Some(String::from(*x)),
                        None => None,
                    },
                }),
            "module" => Ok(
                ServerCommand::Module {
                    subcommand: String::from(v[1]),
                    config_entry: match v.get(2) {
                        Some(x) => Some(String::from(*x)),
                        None => None,
                    },
                    config_value: match v.get(3) {
                        Some(x) => Some(String::from(*x)),
                        None => None,
                    },
                    module_num: match v.get(4) {
                        Some(x) => Some(String::from(*x)),
                        None => None,
                    },
                }),
            _ => Err(ServerCommandError::InvalidCommand),
        }
    }
}

impl fmt::Display for ServerCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerCommandError::InvalidCommand => write!(f, "Ungültiger Befehl"),
            ServerCommandError::InvalidSubCommand => write!(f, "Ungültiger Unterbefehl (SubCommand)"),
        }
    }
}

impl fmt::Display for ServerCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerCommand::Led   {ref subcommand, ref params} => write!(f, "led {:?} {:?}", subcommand, params),
            ServerCommand::Relais{ref subcommand, ref params} => write!(f, "relais {:?} {:?}", subcommand, params),
            ServerCommand::Server{ref subcommand, ref config_entry, ref config_value} => write!(f, "server {:?} {:?} {:?}", subcommand, config_entry, config_value),
            ServerCommand::Module{ref subcommand, ref config_entry, ref config_value, ref module_num} => write!(f, "module {:?} {:?} {:?} {:?}", subcommand, config_entry, config_value, module_num),
        }
    }
}

impl Error for ServerCommandError {
    fn description(&self) -> &str {
        match *self {
            ServerCommandError::InvalidCommand    => "Ungültiger Server Befehl.",
            ServerCommandError::InvalidSubCommand => "Unbekannter Unterbefehlt.",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ServerCommandError::InvalidCommand    => None,
            ServerCommandError::InvalidSubCommand => None,
        }
    }
}
