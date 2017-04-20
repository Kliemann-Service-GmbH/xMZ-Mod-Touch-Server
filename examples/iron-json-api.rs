#[macro_use] extern crate serde_derive;
extern crate iron;
extern crate router;
extern crate serde_json;

use iron::headers::ContentType;
use iron::prelude::*;
use iron::status;
use router::Router;
use std::error;
use std::error::Error;
use std::fmt::{self, Debug};
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;


// Error handling boilerplate
#[derive(Debug)]
struct StringError(String);
impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}
impl Error for StringError {
    fn description(&self) -> &str { &*self.0 }
} // End error handling

// This is my shared state example. It's a very reduced version ><
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
struct SharedStruct {
    data: u64,
}
impl SharedStruct {
    fn new() -> Self {
        SharedStruct { data: 0 }
    }
    fn set(&mut self, data: u64) {
        self.data = data;
    }
    fn clear(&mut self) {
        self.data = 0;
    }
}
// One update thread is modifying the global struct.
fn start_update_threads(shared_struct: Arc<Mutex<SharedStruct>>) -> Result<(), Box<error::Error>> {
    thread::spawn(move || {
        loop {
            {
                if let Ok(mut shared_struct) = shared_struct.lock() {
                    shared_struct.data += 1;
                }
            }
            thread::sleep(Duration::from_millis(100));
        }
    });
    Ok(())
}
// A web interface to "view" into the global struct, and change some parameters.
fn start_web_interface(shared_struct: Arc<Mutex<SharedStruct>>) -> Result<(), Box<error::Error>> {
    let mut router = Router::new();

    let shared_struct_clone = shared_struct.clone();
    router.get("/", move |req: &mut Request| index(req, shared_struct_clone.clone()), "index");
    let shared_struct_clone = shared_struct.clone();
    router.get("/data", move |req: &mut Request| get_data(req, shared_struct_clone.clone()), "get_data");
    let shared_struct_clone = shared_struct.clone();
    router.post("/data", move |req: &mut Request| post_data(req, shared_struct_clone.clone()), "post_data");
    let shared_struct_clone = shared_struct.clone();
    router.put("/data/set/:value", move |req: &mut Request| put_data(req, shared_struct_clone.clone()), "put_data");

    /// curl http://localhost:3000
    fn index(req: &mut Request, shared_struct: Arc<Mutex<SharedStruct>>) -> IronResult<Response> {
        if let Ok(shared_struct) = shared_struct.lock() {
            let payload = serde_json::to_string(&*shared_struct).unwrap();
            Ok(Response::with((ContentType::json().0, status::Ok, payload)))
        } else {
            Err(IronError::new(StringError("Error, unable to lock() the shared_struct.".to_string()), status::BadRequest))
        }
    }

    /// curl http://localhost:3000/data
    fn get_data(req: &mut Request, shared_struct: Arc<Mutex<SharedStruct>>) -> IronResult<Response> {
        if let Ok(shared_struct) = shared_struct.lock() {
            let payload = serde_json::to_string(&shared_struct.data).unwrap();
            Ok(Response::with((ContentType::json().0, status::Ok, payload)))
        } else {
            Err(IronError::new(StringError("Error, unable to lock() the shared_struct.".to_string()), status::BadRequest))
        }
    }
    
    /// curl -X PUT http://localhost:3000/data/set/666
    fn put_data(req: &mut Request, shared_struct: Arc<Mutex<SharedStruct>>) -> IronResult<Response> {
        if let Ok(mut shared_struct) = shared_struct.lock() {
            // Extract the parameter(s)
            let value = req.extensions.get::<Router>()
                .unwrap().find("value").unwrap_or("0").parse::<u64>().unwrap_or(0);
            
            shared_struct.data = value;

            // Return the "updated" SharedStruct, exact like in the index() handler
            let payload = serde_json::to_string(&*shared_struct).unwrap();
            Ok(Response::with((ContentType::json().0, status::Ok, payload)))
        } else {
            Err(IronError::new(StringError("Error, unable to lock() the shared_struct.".to_string()), status::BadRequest))
        }
    }
    
    /// curl -X POST -d '{ "data": 1337 }' http://localhost:3000/data
    fn post_data(req: &mut Request, shared_struct: Arc<Mutex<SharedStruct>>) -> IronResult<Response> {
        if let Ok(mut shared_struct) = shared_struct.lock() {
            let mut payload = String::new();
            req.body.read_to_string(&mut payload).unwrap();
            // Show received data, only for debugging purpose
            // println!("{:?}", payload);

            // We're expecting the POST to match the format of our SharedStruct
            // ie '{ "data": 1337 }'
            let incoming: SharedStruct = serde_json::from_str(&payload).unwrap();

            // Update the global shared struct with the new data
            *shared_struct = incoming;
            
            // Return the "updated" SharedStruct, exact like in the index() handler
            let payload = serde_json::to_string(&*shared_struct).unwrap();
            Ok(Response::with((ContentType::json().0, status::Ok, payload)))
        } else {
            Err(IronError::new(StringError("Error, unable to lock() the shared_struct.".to_string()), status::BadRequest))
        }
    }


    println!("web interface: http://localhost:3000");
    Iron::new(router).http("localhost:3000")?;

    Ok(())
}

fn run() -> Result<(), Box<error::Error>> {
    let shared_struct = Arc::new(Mutex::new(SharedStruct::new()));

    start_update_threads(shared_struct.clone())?;

    start_web_interface(shared_struct.clone())?;

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("error: {}", err);

        ::std::process::exit(1);
    }
}
