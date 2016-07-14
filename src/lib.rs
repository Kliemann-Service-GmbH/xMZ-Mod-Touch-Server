#![doc(html_logo_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/xmz-logo.png",
       html_favicon_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/favicon.ico",
       html_root_url = "https://gaswarnanlagen.com/")]
#![feature(stmt_expr_attributes)]

//! xMZ-Mod-Touch Server

extern crate libmodbus_rs;
extern crate nanomsg;
extern crate sysfs_gpio;

//extern crate xmz_shift_register;
/// Shift register, Zugriff und Steuerung der an die Shift Register angeschlossenen Hardware
pub mod shift_register;
/// Server  - Der Server Prozess ist der Hauptprozess der 'xMZ-Mod-Touch' Plattform
///
/// Er verbindet die LED, Relais, Alarmzonen und kontrolliert die Module mit deren Sensoren.
pub mod server;
/// Module  - Sensorplatinen mit Modbus Transceiver und ein oder mehr Sensor Messzellen
///
/// Eine Sensorplatine verfügt immer über ein Modbus Transceiver (Sender/ Empfänger). Auf der Platine oder an
/// der Platine sind Sensormesszellen angeschlossen. Diese haben entsprechende Register (Speicherbereiche) die
/// dann über das Modbus Protokoll abgefragt werden konnen.
pub mod module;
/// Sensore     - Einzellne Messzelle
///
/// Dieses Modul beinhaltet alle Funktionen und Datenstrukturen die zur Sensorauswertung nötig sind.
/// Also die Umwandlung des Analog Signals in ein Wert, die Mittelwert Bildung und in Teilen auch die Störungsüberwachung.
pub mod sensor;

pub mod nanomsg_device;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
