extern crate rustc_serialize;
extern crate xmz_server;

use rustc_serialize::json;
use xmz_server::server::server::Server;



fn main() {
    let mut server = Server::new();
    let _ = server.init();

    println!("{:#?}", &server.modules);
    println!("{:#?}", json::encode(&server.modules).unwrap());
}
