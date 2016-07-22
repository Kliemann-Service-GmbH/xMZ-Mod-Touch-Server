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
    Led    { subcommand: String, params: Option<String> },
    Relais { subcommand: String, params: Option<String> },
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
    /// use xmz_server::server::server_command::{ServerCommand};
    ///
    /// let cmd = ServerCommand::from_str("led set 1");
    /// assert_eq!(cmd, Ok(ServerCommand::Led { subcommand: "set".to_string(), params: Some("1".to_string()) }));
    /// ```
    fn from_str(s: &str) -> Result<ServerCommand, ServerCommandError> {
        let v: Vec<_> = s.split_whitespace().collect();

        match v[0] {
            "led" if v.len() == 2
            && v[1] == "list" => Ok( // list
                ServerCommand::Led {
                    subcommand: String::from(v[1]),
                    params: None,
                }),
            "led" if v.len() == 3
            && (v[1] == "get" || v[1] == "set" || v[1] == "toggle" || v[1] == "clear" ) => Ok( // get || set || toggle || clear
                ServerCommand::Led {
                    subcommand: String::from(v[1]),
                    params: Some(String::from(v[2])),
                }),

            "relais" if v.len() == 2
                && v[1] == "list" => Ok( // list
                ServerCommand::Relais {
                    subcommand: String::from(v[1]),
                    params: None,
                }),
            "relais" if v.len() == 3
            && (v[1] == "get" || v[1] == "set" || v[1] == "toggle" || v[1] == "clear" ) => Ok( // get || set || toggle || clear
                ServerCommand::Relais {
                    subcommand: String::from(v[1]),
                    params: Some(String::from(v[2])),
                }),

            "server" if v.len() == 3
            && v[1] == "get" => Ok( // get
                ServerCommand::Server {
                    subcommand: String::from(v[1]),
                    config_entry: String::from(v[2]),
                    config_value: None,
                }),
            "server" if v.len() == 4
            && v[1] == "set" => Ok( // set
                ServerCommand::Server {
                    subcommand: String::from(v[1]),
                    config_entry: String::from(v[2]),
                    config_value: Some(String::from(v[3])),
                }),

            "module" if v.len() == 2
            && (v[1] == "new" || v[1] == "list") => Ok( // new || list
                ServerCommand::Module {
                    subcommand: String::from(v[1]),
                    config_entry:None,
                    config_value: None,
                    module_num: None,
                }),
            "module" if v.len() == 3
            && v[1] == "show" => Ok( // show
                ServerCommand::Module {
                    subcommand: String::from(v[1]),
                    config_entry: None,
                    config_value: None,
                    module_num:   Some(String::from(v[2])),
                }),
            "module" if v.len() == 4
            && v[1] == "get" => Ok( // get
                ServerCommand::Module {
                    subcommand: String::from(v[1]),
                    config_entry: Some(String::from(v[2])),
                    config_value: None,
                    module_num:   Some(String::from(v[3])),
                }),
            "module" if v.len() == 5
            && v[1] == "set" => Ok( // set
                ServerCommand::Module {
                    subcommand: String::from(v[1]),
                    config_entry: Some(String::from(v[2])),
                    config_value: Some(String::from(v[3])),
                    module_num:   Some(String::from(v[4])),
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
            ServerCommand::Relais {ref subcommand, ref params} => write!(f, "relais {:?} {:?}", subcommand, params),
            ServerCommand::Server {ref subcommand, ref config_entry, ref config_value} => write!(f, "server {:?} {:?} {:?}", subcommand, config_entry, config_value),
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use server::server_command::{ServerCommand, ServerCommandError};

    #[test]
    fn led() {
        let cmd_list    = ServerCommand::from_str("led list");
        let cmd_get     = ServerCommand::from_str("led get 1");
        let cmd_set     = ServerCommand::from_str("led set 1");
        let cmd_toggle  = ServerCommand::from_str("led toggle 1");
        let cmd_clear   = ServerCommand::from_str("led clear 1");
        assert_eq!(cmd_list,    Ok(ServerCommand::Led { subcommand: "list".to_string(), params: None }));
        assert_eq!(cmd_get,     Ok(ServerCommand::Led { subcommand: "get".to_string(), params: Some("1".to_string()) }));
        assert_eq!(cmd_set,     Ok(ServerCommand::Led { subcommand: "set".to_string(), params: Some("1".to_string()) }));
        assert_eq!(cmd_toggle,  Ok(ServerCommand::Led { subcommand: "toggle".to_string(), params: Some("1".to_string()) }));
        assert_eq!(cmd_clear,   Ok(ServerCommand::Led { subcommand: "clear".to_string(), params: Some("1".to_string()) }));
    }
    #[test]
    fn led_invalid() {
        let cmd                 = ServerCommand::from_str("led");
        let cmd_invalid_sub     = ServerCommand::from_str("led foo");
        let cmd_invalid_param   = ServerCommand::from_str("led foo bar");
        let cmd_invalid         = ServerCommand::from_str("led foo bar baz");
        assert_eq!(cmd,                 Err(ServerCommandError::InvalidCommand));
        assert_eq!(cmd_invalid_sub,     Err(ServerCommandError::InvalidCommand));
        assert_eq!(cmd_invalid_param,   Err(ServerCommandError::InvalidCommand));
        assert_eq!(cmd_invalid,         Err(ServerCommandError::InvalidCommand));
    }

    #[test]
    fn relais() {
        let cmd_list    = ServerCommand::from_str("relais list");
        let cmd_get     = ServerCommand::from_str("relais get 1");
        let cmd_set     = ServerCommand::from_str("relais set 1");
        let cmd_toggle  = ServerCommand::from_str("relais toggle 1");
        let cmd_clear   = ServerCommand::from_str("relais clear 1");
        assert_eq!(cmd_list,    Ok(ServerCommand::Relais { subcommand: "list".to_string(), params: None }));
        assert_eq!(cmd_get,     Ok(ServerCommand::Relais { subcommand: "get".to_string(), params: Some("1".to_string()) }));
        assert_eq!(cmd_set,     Ok(ServerCommand::Relais { subcommand: "set".to_string(), params: Some("1".to_string()) }));
        assert_eq!(cmd_toggle,  Ok(ServerCommand::Relais { subcommand: "toggle".to_string(), params: Some("1".to_string()) }));
        assert_eq!(cmd_clear,   Ok(ServerCommand::Relais { subcommand: "clear".to_string(), params: Some("1".to_string()) }));
    }
    #[test]
    fn relais_invalid() {
        let cmd                 = ServerCommand::from_str("relais");
        let cmd_invalid_sub     = ServerCommand::from_str("relais foo");
        let cmd_invalid_param   = ServerCommand::from_str("relais foo bar");
        // TODO: let cmd_invalid_get     = ServerCommand::from_str("relais get bar");
        let cmd_invalid_set     = ServerCommand::from_str("relais set bar 1");
        let cmd_invalid_set2    = ServerCommand::from_str("relais set bar zoo");
        assert_eq!(cmd,                 Err(ServerCommandError::InvalidCommand));
        assert_eq!(cmd_invalid_sub,     Err(ServerCommandError::InvalidCommand));
        assert_eq!(cmd_invalid_param,   Err(ServerCommandError::InvalidCommand));
        // TODO: assert_eq!(cmd_invalid_get,     Err(ServerCommandError::InvalidCommand));
        assert_eq!(cmd_invalid_set,     Err(ServerCommandError::InvalidCommand));
        assert_eq!(cmd_invalid_set2,    Err(ServerCommandError::InvalidCommand));
    }

    #[test]
    fn server() {
        let cmd_get = ServerCommand::from_str("server get modbus_device");
        let cmd_set = ServerCommand::from_str("server set modbus_device /dev/ttyUSB0");
        assert_eq!(cmd_get, Ok(ServerCommand::Server { subcommand: "get".to_string(), config_entry: "modbus_device".to_string(), config_value: None }));
        assert_eq!(cmd_set, Ok(ServerCommand::Server { subcommand: "set".to_string(), config_entry: "modbus_device".to_string(), config_value: Some("/dev/ttyUSB0".to_string()) }));
    }

    #[test]
    fn module() {
        let cmd_new  = ServerCommand::from_str("module new");
        let cmd_list = ServerCommand::from_str("module list");
        let cmd_show = ServerCommand::from_str("module show 1");
        let cmd_get  = ServerCommand::from_str("module get modbus_slave_id 1");
        let cmd_set  = ServerCommand::from_str("module set modbus_slave_id 100 1");
        assert_eq!(cmd_new,  Ok(ServerCommand::Module { subcommand: "new".to_string(),  config_entry: None, config_value: None, module_num: None }));
        assert_eq!(cmd_list, Ok(ServerCommand::Module { subcommand: "list".to_string(), config_entry: None, config_value: None, module_num: None }));
        assert_eq!(cmd_show,  Ok(ServerCommand::Module { subcommand: "show".to_string(),  config_entry: None, config_value: None, module_num: Some("1".to_string()) }));
        assert_eq!(cmd_get,  Ok(ServerCommand::Module { subcommand: "get".to_string(),  config_entry: Some("modbus_slave_id".to_string()), config_value: None, module_num: Some("1".to_string()) }));
        assert_eq!(cmd_set,  Ok(ServerCommand::Module { subcommand: "set".to_string(),  config_entry: Some("modbus_slave_id".to_string()), config_value: Some("100".to_string()), module_num: Some("1".to_string()) }));
    }

}
