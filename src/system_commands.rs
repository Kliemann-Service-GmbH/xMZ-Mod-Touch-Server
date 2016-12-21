use errors::*;
use std::process::Command;


pub fn mount() -> Result<()> {
    Command::new("mount")
        .arg("/dev/mmcblk0p1")
        .arg("/boot")
        .spawn()
        .chain_err(|| "Partition /dev/mmcblk0p1 konnte nicht gemounted werden.")?;

    Ok(())
}

pub fn umount() -> Result<()> {
    Command::new("umount")
        .arg("/boot")
        .spawn()
        .chain_err(|| "/dev/mmcblk0p1 konnte nicht unmounted werden.")?;

    Ok(())
}
