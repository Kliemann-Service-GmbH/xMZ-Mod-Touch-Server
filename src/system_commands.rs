use error::*;
use std::process::Command;
use std;


/// Führt den als Parameter `command` übergebenen Befehl in einer `sh -c` aus, oder wirft ein
/// `XMZError::SystemCommandFailed` Fehler aus.
pub fn call<C: AsRef<str>>(command: C) -> Result<()>
    where C: std::convert::AsRef<std::ffi::OsStr>
{
    match Command::new("sh")
        .arg("-c")
        .arg(command)
        .status() {
        Ok(status) => {
            if status.success() {
                Ok(())
            } else {
                Err(XMZError::SystemCommandFailed)
            }
        }
        Err(err) => return Err(err.into()),
    }
}
