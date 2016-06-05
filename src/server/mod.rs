//! xMZ-Mod-Touch Server Core
//!
//! Der Kern des Servers.
use module::Module;

struct Server {
    led: u8,
    relais: u8,
    module: Vec<Module>,
}

impl Server {
    pub fn new() -> Self {
        Server { led: 0, relais: 0, module: vec!() }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn basics() {
        assert!(true);
    }
}
