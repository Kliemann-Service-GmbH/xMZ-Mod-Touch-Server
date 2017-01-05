use errors::*;
use std;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;


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
                Err("System Command failed!".into())
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
