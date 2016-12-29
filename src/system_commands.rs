use error::*;
use std::process::{Command, ExitStatus};
use std;


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
