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
pub struct StringError<'a>(&'a str);

impl<'a> fmt::Display for StringError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl<'a> Error for StringError<'a> {
    fn description(&self) -> &str {
        &*self.0
    }
}

pub fn init(xmz_server: Arc<Mutex<XMZServer>>) -> Result<(), XMZServerError> {
    let mut router = Router::new();

    /// curl http://localhost:3000
    let xmz_server_clone = xmz_server.clone();
    router.get("/*",
               move |req: &mut Request| index(req, xmz_server_clone.clone()),
               "index");
    /// curl http://localhost:3000/api/v1
    let xmz_server_clone = xmz_server.clone();
    router.get("/api/v1",
               move |req: &mut Request| index(req, xmz_server_clone.clone()),
               "index_with_api");

    /// curl http://localhost:3000/api/v1/zones
    let xmz_server_clone = xmz_server.clone();
    router.get("/api/v1/zones",
               move |req: &mut Request| zones_index(req, xmz_server_clone.clone()),
               "zones_index");

    fn index(_req: &mut Request, xmz_server: Arc<Mutex<XMZServer>>) -> IronResult<Response> {
        if let Ok(xmz_server) = xmz_server.lock() {
            let payload = serde_json::to_string_pretty(&*xmz_server).unwrap();
            Ok(Response::with((status::Ok, payload)))
        } else {
            Err(IronError::new(StringError("Mutex XMZServer lock failed"), status::BadRequest))
        }
    }

    fn zones_index(_req: &mut Request, xmz_server: Arc<Mutex<XMZServer>>) -> IronResult<Response> {
        if let Ok(xmz_server) = xmz_server.lock() {
            let payload = serde_json::to_string_pretty(&*xmz_server.get_zones()).unwrap();
            Ok(Response::with((status::Ok, payload)))
        } else {
            Err(IronError::new(StringError("Mutex XMZServer lock failed"), status::BadRequest))
        }
    }

    println!("Webinterface: http://localhost:3000");
    Iron::new(router).http("localhost:3000").unwrap();

    Ok(())
}
