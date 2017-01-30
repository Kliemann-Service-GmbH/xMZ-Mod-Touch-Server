
use errors::*;
use system_command;

/// Mounted die erste Partition der SDCard nach /boot
#[allow(dead_code)]
fn mount_boot() -> Result<()> {
    if system_command::is_mounted("/boot") == false {
        system_command::call("mount /dev/mmcblk0p1 /boot")?;
    }

    Ok(())
}

/// Unmount /boot
#[allow(dead_code)]
fn umount_boot() -> Result<()> {
    system_command::call("umount /boot")?;

    Ok(())
}

/// Diese Funktion liest die Konfigurationsdatei ein, je nach Umgebung
///
/// Entweder wird das programm im `development` Modus aufgerufen, hier wird die Konfigurationsdatei
/// lokal gesucht und gelesen.
/// Oder aber das Programm wird im `produktiv` Modus (not(feature = "development")) ausgeführt,
/// in diesem wird zunächste /boot gemounted, anschließend die Konfigurationsdatei eingelesen
/// und zum Schluss /boot umounted.
#[allow(unused_assignments)]
pub fn read_config_file() -> Result<String> {
    let mut config_file = String::new();

    #[cfg(feature = "development")]
    {
        info!("Development System, Konfiguration einlesen.");
        config_file = try!(system_command::read_in("xMZ-Mod-Touch.json"));
    }
    #[cfg(not(feature = "development"))]
    {
        info!("Produktiv System, Konfiguration einlesen.");
        try!(mount_boot());
        // Hier kann nicht einfach ein try!(system_command::read_in(..)) angewannt werden,
        // da im Fehlerfall (Konfig ungültig) noch /boot umounted werden muss
        config_file = match system_command::read_in("/boot/xMZ-Mod-Touch.json") {
            Ok(config_file) => config_file,
            Err(_) => {
                try!(umount_boot());
                // Im Fehlerfall wird ein leerer String zurück gegeben
                String::new()
            },
        };
        try!(umount_boot());
    }

    Ok(config_file)
}
