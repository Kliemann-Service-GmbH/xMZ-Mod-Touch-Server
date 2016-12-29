use error::*;
use std::process::{Command, ExitStatus};
use std;

pub fn mount() -> Result<()> {
    call("mount /boot")?;

    Ok(())
}

pub fn umount() -> Result<()> {
    call("umount /boot")?;

    Ok(())
}

fn call<C: AsRef<str>>(command: C) -> Result<()>
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
                Err(XMZError::NotAllowed)
            }
        }
        Err(err) => return Err(err.into()),
    }
}
