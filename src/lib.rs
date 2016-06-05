//! xMZ-Mod-Touch Server
//!
//! Der Server Prozess ist der Hauptprozess der 'xMZ-Mod-Touch' Plattform.
//! Er steuert die LED, Relais und Kontrolliert die Module mit den Sensoren.
mod server;
mod module;
mod sensor;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
