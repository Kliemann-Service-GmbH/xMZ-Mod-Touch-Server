/// Dieses Beispiel dient auch dazu eine valide Konfiguration aus den einzelenen Modulen der
/// Software zu erstellen.
///
/// Hier wird eine Konfiguration erstellt. Die Kombisensoren werden wie der Server von Hand
/// initalisiert, anschließend wird das Configuration Objekt in JSON kodiert in
/// eine Datei (Configuration.json) geschrieben.

extern crate xmz_server;

use std::fs::File;
use std::io::prelude::*;
use xmz_server::*;


fn run() -> Result<()> {
    let configuration = Configuration {
        server: Server::new(),
    };

    let configuration_json = try!(configuration.to_json());
    let mut f = try!(File::create("Configuration.json"));
    try!(f.write(configuration_json.as_bytes()));

    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        use ::std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}
