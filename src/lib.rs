#![doc(html_logo_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/xmz-logo.png",
       html_favicon_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/favicon.ico",
       html_root_url = "https://gaswarnanlagen.com/")]
#![feature(stmt_expr_attributes)]
//! xMZ-Mod-Touch Server
//extern crate xmz_shift_register;
/// Shift register, Zugriff und Steuerung der an die Shift Register angeschlossenen Hardware
pub mod shift_register;
/// Server
///
/// Der Server Prozess ist der Hauptprozess der 'xMZ-Mod-Touch' Plattform.
/// Er verbindet die LED, Relais, Alarmzonen und kontrolliert die Module mit deren Sensoren.
pub mod server;
mod module;
/// Sensoren
///
/// Dieses Modul beinhaltet alle Funktionen und Datenstrukturen die zur Sensorauswertung nötig sind.
pub mod sensor;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
