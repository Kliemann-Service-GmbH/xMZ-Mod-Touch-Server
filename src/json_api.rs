extern crate serde_json;
extern crate iron;
extern crate router;

use error::XMZServerError;
use iron::prelude::*;
use iron::status;
use router::Router;
use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex};
use xmz_server::XMZServer;


// Json Web Interface
#[derive(Debug)]
pub struct StringError(String);

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Error for StringError {
    fn description(&self) -> &str {
        &*self.0
    }
}

pub fn init(xmz_server: Arc<Mutex<XMZServer>>) -> Result<(), XMZServerError> {
    let mut router = Router::new();

    let xmz_server_clone = xmz_server.clone();
    router.get("/",
               move |req: &mut Request| index(req, xmz_server_clone.clone()),
               "index");
    let xmz_server_clone = xmz_server.clone();
    router.get("/api/v1",
               move |req: &mut Request| index(req, xmz_server_clone.clone()),
               "index_with_api");

    fn index(_req: &mut Request, xmz_server: Arc<Mutex<XMZServer>>) -> IronResult<Response> {
        if let Ok(xmz_server) = xmz_server.lock() {
            let payload = serde_json::to_string_pretty(&*xmz_server).unwrap();
            Ok(Response::with((status::Ok, payload)))
        } else {
            Err(IronError::new(XMZServerError::ConfigNotFound, status::BadRequest))
        }
    }

    println!("Webinterface: http://localhost:3000");
    Iron::new(router).http("localhost:3000").unwrap();

    Ok(())
}
