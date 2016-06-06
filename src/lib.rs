#![doc(html_logo_url = "../../../share/xmz-logo.png",
       html_favicon_url = "../../../share/favicon.ico",
       html_root_url = "https://doc.rust-lang.org/")]
#![feature(stmt_expr_attributes)]
//! xMZ-Mod-Touch Server
//!
//! Der Server Prozess ist der Hauptprozess der 'xMZ-Mod-Touch' Plattform.
//! Er steuert die LED, Relais, Alarmzonen und kontrolliert die Module mit den Sensoren.
extern crate xmz_shift_register;
pub mod server;
mod module;
mod sensor;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
