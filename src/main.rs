#![doc(html_logo_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/xmz-logo.png",
       html_favicon_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/favicon.ico",
       html_root_url = "https://gaswarnanlagen.com/")]
//! # Serverteil der 'xMZ-Mod-Touch'-Platform&nbsp;[![Build Status](https://travis-ci.org/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server.svg?branch=master)](https://travis-ci.org/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server)
//! [Projektseite |][homepage]&nbsp;[Dokumentation |][doku]&nbsp;[Repo auf Github.com |][repo]
//!
//! Dies ist der Hauptprozess der sehr früh, noch vor der GUI, gestartet wird.
//! Er steuert die Hardware (LEDs, Relais, Akku usw.) und händelt die angeschlossenen
//! Module und deren Sensoren.
//!
//! [1]: https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Software
//! [homepage]: http://kliemann-service-gmbh.github.io/xMZ-Mod-Touch-Server
//! [repo]: https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server
//! [doku]: http://kliemann-service-gmbh.github.io/xMZ-Mod-Touch-Server/xmz_server/index.html

#[macro_use] extern crate log;
extern crate env_logger;

pub mod exceptions;
mod kombisensors;
mod sensors;
mod server;
mod shift_register;
mod zones;


fn main() {
    env_logger::init().unwrap();

    let mut server = server::Server::new();

    loop {
        server.update();

        server.check_exceptions();
        info!("Ausnahmen\n{:#?}", server.exceptions());
    }
}
