//! Konfiguration Datei Managment
//!
use std::sync::{Arc, Mutex};
use serde_json;
use xmz_mod_touch_server::XMZModTouchServer;
use errors::*;
use std::fs::File;
use std::path::Path;
use std::io::Read;


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
fn get_config() -> Result<String> {
    let possible_paths = vec![
        Path::new("/boot/xMZ-Mod-Touch.json.production"),
        Path::new("/usr/share/xmz-mod-touch-server/xMZ-Mod-Touch.json.production"),
        Path::new("xMZ-Mod-Touch.json"),
    ];

    let mut ret = String::new();
    for p in possible_paths {
        if Path::new(p).exists() {
            match File::open(&p) {
                Ok(mut file) => {
                    println!("Verwende Konfigurationsdatei: {}", p.display());
                    file.read_to_string(&mut ret);
                }
                Err(_) => panic!("Could not open file: {}", p.display()),
            };
            break;
        }
    }
    Ok(ret)
}
// /usr/share/xmz-mod-touch-server/xMZ-Mod-Touch.json.production


///
/// # Return values
///
///
/// # Parameters
///
/// # Examples
///
/// ```rust,ignore
/// // TODO: Write documentation
/// assert!(false);
/// ```
pub fn parse() -> Arc<Mutex<XMZModTouchServer>> {
    let xmz_mod_touch_server = match serde_json::from_str(&get_config().unwrap()) {
        Ok(xmz_mod_touch_server) => xmz_mod_touch_server,
        _ => panic!("Konnte Konfigurationsdatei nicht lesen. Server konnte nicht erstellt werden."),
    };
    Arc::new(Mutex::new(xmz_mod_touch_server))
}
