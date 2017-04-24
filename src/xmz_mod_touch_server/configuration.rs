use std::sync::{Arc, Mutex};
use serde_json;
use xmz_mod_touch_server::XMZModTouchServer;
use error::*;
use std::fs::File;
use std::path::Path;
use std::io::Read;


fn get_config() -> Result<String> {
    let possible_paths = vec![
        Path::new("/boot/xMZ-Mod-Touch.json.production"),
        Path::new("/usr/share/xmz-mod-touch-server/xMZ-Mod-Touch.json.production"),
        Path::new("xMZ-Mod-Touch.json"),
    ];

    let mut ret = String::new();
    for p in possible_paths {
        if Path::new(p).exists() {
            let display = p.display();
            match File::open(&p) {
                Ok(mut file) => {
                    println!("Verwende Konfigurationsdatei: {}", display);
                    file.read_to_string(&mut ret);
                }
                Err(_) => panic!("Could not open file: {}", display),
            };
            break;
        }
    }
    Ok(ret)
}
// /usr/share/xmz-mod-touch-server/xMZ-Mod-Touch.json.production


pub fn parse() -> Arc<Mutex<XMZModTouchServer>> {
    let xmz_mod_touch_server = match serde_json::from_str(&get_config().unwrap()) {
        Ok(xmz_mod_touch_server) => xmz_mod_touch_server,
        _ => panic!("Konnte Konfigurationsdatei nicht lesen. Server konnte nicht erstellt werden."),
    };
    Arc::new(Mutex::new(xmz_mod_touch_server))
}
