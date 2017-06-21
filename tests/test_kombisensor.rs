extern crate xmz_mod_touch_server;

use xmz_mod_touch_server::Server;

#[test]
fn basic() {
    let server = Server::new();

    assert_eq!(server.get_zones().len(), 0);
}
