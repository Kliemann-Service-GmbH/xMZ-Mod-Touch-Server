extern crate xmz_server;
use xmz_server::server::Server;

fn main() {
    let mut server = Server::new();
    server.init();
}
