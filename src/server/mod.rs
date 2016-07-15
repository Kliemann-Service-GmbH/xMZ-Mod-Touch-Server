/// Server  - Serverteil der Software
pub mod server;

/// Zonen   - Verwaltung der Störungen und Alarme
///
/// Jede Zone hat mindestens ein Alarmpunkt. Jedem dieser Alarmpunkte können Relais und LED zugewiesen werden.
/// Diese werden dann aktiviert/ deaktiviert, je nach Schaltrichtung.
pub mod zone;

/// ServerCommand   - Befehle die der Server verarbeiten kann_module_modbus_adresse_abfragen
///
pub mod server_command;
