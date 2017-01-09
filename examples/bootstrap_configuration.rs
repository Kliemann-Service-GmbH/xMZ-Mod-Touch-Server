/// Dieses Beispiel dient zum Erstellen einer validen Konfigurationsdatei.
///
/// Dazu wird eine Server Instanz erzeugt und konfiguriert. Anschließend wird der
/// Server in das Json Format serialisiert und in eine Text Datei gespeichert.

extern crate serde_json;
extern crate xmz_server;

use std::fs::File;
use std::io::prelude::*;
use xmz_server::*;


fn run() -> Result<()> {
    let server = Server::new();
    let server_str = try!(serde_json::to_string_pretty(&server));
    let mut f = try!(File::create("Configuration.json"));

    try!(f.write(server_str.as_bytes()));

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
