//! Konfiguration Datei Managment
//!
use errors::*;
use std::fs::File;
use std::path::Path;
use std::io::Read;


pub struct Configuration;

impl Configuration {
    /// Liest die Konfiguration
    ///
    /// # Return values
    ///
    /// Diese Funktion liefert ein Result. Das Result enthält die Konfiguration, als String, oder ein Error,
    /// wenn die Konfiguration nicht ausgelesen werden konnte.
    ///
    /// # Parameters
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert!(true);
    /// ```
    pub fn get_config() -> Result<String> {
        // TODO: In production nur Konfig von `/boot` verwenden!
        let possible_paths = vec![
            Path::new("/boot/xMZ-Mod-Touch.json"),
            Path::new("/usr/share/xmz-mod-touch-server/xMZ-Mod-Touch.json.production"),
            Path::new("xMZ-Mod-Touch.json"),
        ];

        let mut ret = String::new();
        for p in possible_paths {
            if Path::new(p).exists() {
                match File::open(&p) {
                    Ok(mut file) => {
                        println!("Verwende Konfigurationsdatei: {}", p.display());
                        file.read_to_string(&mut ret)?;
                    }
                    Err(_) => panic!("Could not open file: {}", p.display()),
                };
                break;
            }
        }
        Ok(ret)
    }
}
