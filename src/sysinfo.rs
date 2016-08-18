use std::borrow::Cow;

/// IP Konfiguration der Hardware Platform
pub fn ifconfig<'a>() -> String {
    match ::std::process::Command::new("ifconfig").output() {
        Ok(ifconfig) => String::from_utf8_lossy(&ifconfig.stdout).to_string(),
        Err(_) => format!("Error failed to execute `ifconfig`"),
    }
}
