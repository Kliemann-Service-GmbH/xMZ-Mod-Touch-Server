use errors::*;
use std;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;


/// Prüft ob der übergebene Pfad gemouted ist
///
/// # Examples
/// ```
/// use xmz_server::system_command::*;
///
/// assert_eq!(is_mounted("/"), true);
/// assert_eq!(is_mounted("/not-existend-hope-so"), false);
/// ```
pub fn is_mounted(path: &str) -> bool {
    match Command::new("sh")
        .arg("-c")
        .arg(format!("mountpoint -q {}", path))
        .status()
    {
        Ok(status) => if status.success() { true } else { false },
        Err(_) => false
    }
}

/// Führt den als Parameter `command` übergebenen Befehl in einer `sh -c` aus,
/// oder wirft ein Fehler aus.
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
                Err(ErrorKind::SystemCommandError.into())
            }
        }
        Err(err) => return Err(err.into()),
    }
}

/// Liest eine Text Datei ein und liefert ein String Result
pub fn read_in<P: AsRef<str>>(path: P) -> Result<String>
    where P: std::convert::AsRef<std::path::Path> + std::fmt::Debug
{
    let mut f = try!(File::open(&path).chain_err(|| format!("unable to open file: {:?}", path)));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));

    Ok(s)
}
