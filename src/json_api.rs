//! JSON Web API der xMZ-Mod-Touch-Server Platform
//!
//! Übersicht über die möglichen URL's
//!
//! |URL|Beschreibung|
//! |------------------------------------------------------------------------|------------------------------------------------------------------------|
//! |[http://localhost:3000](http://localhost:3000)|Index, komplette Server Datenstruktur wird als JSON Objekt zurück gegeben|
//! |[http://localhost:3000/api/v1](http://localhost:3000/api/v1)|Index, komplette Server Datenstruktur wird als JSON Objekt zurück gegeben|
//! |[http://localhost:3000/api/v1/zones](http://localhost:3000/api/v1/zones)|Alle Zonen abfragen|
//! |[http://localhost:3000/api/v1/zone/0](http://localhost:3000/api/v1/zone/0)|Eine Zone abfragen, ZoneID als Parameter übergeben|
//! |[http://localhost:3000/api/v1/zone/0/kombisensors](http://localhost:3000/api/v1/zone/0/kombisensors)|Alle Sensoren des Kombisensors abfragen|
//! |[http://localhost:3000/api/v1/zone/0/kombisensor/0](http://localhost:3000/api/v1/zone/0/kombisensor/0)||
//! |[http://localhost:3000/api/v1/zone/0/kombisensor/0/sensors](http://localhost:3000/api/v1/zone/0/kombisensor/0/sensors)|Alle Sensoren des Kombisensors abfragen|
//! |[http://localhost:3000/api/v1/zone/0/kombisensor/0/sensor/0](http://localhost:3000/api/v1/zone/0/kombisensor/0/sensor/0)|Ein Sensor abfragen, SensorID als weiteren Parameter mit übergeben|
//! |[http://localhost:3000/api/v1/exceptions](http://localhost:3000/api/v1/exceptions)|Ausnahmen des Servers erfragen|
//!

// TODO: https://disconnected.systems/blog/rover-rest-api/
use errors::*;
use iron::prelude::*;
use iron::status;
use router::Router;
use serde_json;
use Server;
use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex};

/// Json Web Interface Fehler
/// TODO: port to error-chain
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

/// Beispiel URL: http://0.0.0.0:3000/api/v1
fn index(_req: &mut Request,
         xmz_mod_touch_server: Arc<Mutex<Server>>)
         -> IronResult<Response>
{
    if let Ok(xmz_mod_touch_server) = xmz_mod_touch_server.lock() {
        let payload = serde_json::to_string_pretty(&*xmz_mod_touch_server).unwrap();
        Ok(Response::with((status::Ok, payload)))
    } else {
        Err(IronError::new(StringError("Mutex Server lock failed"),
                           status::BadRequest))
    }
}

/// Beispiel URL: http://0.0.0.0:3000/api/v1/zones
fn zones_index(_req: &mut Request,
               xmz_mod_touch_server: Arc<Mutex<Server>>)
               -> IronResult<Response> {
    if let Ok(xmz_mod_touch_server) = xmz_mod_touch_server.lock() {
        let payload = serde_json::to_string_pretty(&*xmz_mod_touch_server.get_zones()).unwrap();
        Ok(Response::with((status::Ok, payload)))
    } else {
        Err(IronError::new(StringError("Mutex Server lock failed"),
                           status::BadRequest))
    }
}

/// Beispiel URL: http://0.0.0.0:3000/api/v1/zone/0
fn zone_get(req: &mut Request,
            xmz_mod_touch_server: Arc<Mutex<Server>>)
            -> IronResult<Response> {
    if let Ok(xmz_mod_touch_server) = xmz_mod_touch_server.lock() {
        // Extract the parameter(s)
        let zone_id = req.extensions
            .get::<Router>()
            .unwrap()
            .find("zone_id")
            .unwrap_or("0")
            .parse::<usize>()
            .unwrap_or(0);

        let payload = serde_json::to_string_pretty(&xmz_mod_touch_server.get_zone(zone_id))
            .unwrap();
        Ok(Response::with((status::Ok, payload)))
    } else {
        Err(IronError::new(StringError("Mutex Server lock failed"),
                           status::BadRequest))
    }
}

/// Beispiel URL: http://0.0.0.0:3000/api/v1/zone/0/kombisensors
fn kombisensors_index(req: &mut Request,
                      xmz_mod_touch_server: Arc<Mutex<Server>>)
                      -> IronResult<Response> {
    if let Ok(xmz_mod_touch_server) = xmz_mod_touch_server.lock() {
        // Extract the parameter(s)
        let zone_id = req.extensions
            .get::<Router>()
            .unwrap()
            .find("zone_id")
            .unwrap_or("0")
            .parse::<usize>()
            .unwrap_or(0);

        // Get Kombisensors
        let kombisensors = &xmz_mod_touch_server.get_zone(zone_id)
            .map(|zone| zone.get_kombisensors());

        let payload = serde_json::to_string_pretty(kombisensors).unwrap();
        Ok(Response::with((status::Ok, payload)))
    } else {
        Err(IronError::new(StringError("Mutex Server lock failed"),
                           status::BadRequest))
    }
}

/// Beispiel URL: http://0.0.0.0:3000/api/v1/zone/0/kombisensor/0
fn kombisensor_get(req: &mut Request,
                   xmz_mod_touch_server: Arc<Mutex<Server>>)
                   -> IronResult<Response> {
    if let Ok(xmz_mod_touch_server) = xmz_mod_touch_server.lock() {
        // Extract the parameter(s)
        let zone_id = req.extensions
            .get::<Router>()
            .unwrap()
            .find("zone_id")
            .unwrap_or("0")
            .parse::<usize>()
            .unwrap_or(0);
        let kombisensor_id = req.extensions
            .get::<Router>()
            .unwrap()
            .find("kombisensor_id")
            .unwrap_or("0")
            .parse::<usize>()
            .unwrap_or(0);

        // Get Kombisensor
        let kombisensor = &xmz_mod_touch_server.get_zone(zone_id)
            .map(|zone| zone.get_kombisensor(kombisensor_id));

        let payload = serde_json::to_string_pretty(kombisensor).unwrap();
        Ok(Response::with((status::Ok, payload)))
    } else {
        Err(IronError::new(StringError("Mutex Server lock failed"),
                           status::BadRequest))
    }
}

/// Beispiel URL: http://0.0.0.0:3000/api/v1/zone/0/kombisensor/0/sensors
fn sensors_index(req: &mut Request,
                 xmz_mod_touch_server: Arc<Mutex<Server>>)
                 -> IronResult<Response> {
    if let Ok(xmz_mod_touch_server) = xmz_mod_touch_server.lock() {
        // Extract the parameter(s)
        let zone_id = req.extensions
            .get::<Router>()
            .unwrap()
            .find("zone_id")
            .unwrap_or("0")
            .parse::<usize>()
            .unwrap_or(0);
        let kombisensor_id = req.extensions
            .get::<Router>()
            .unwrap()
            .find("kombisensor_id")
            .unwrap_or("0")
            .parse::<usize>()
            .unwrap_or(0);

        // Get Sensors
        let sensors = &xmz_mod_touch_server.get_zone(zone_id).map(|zone| {
            zone.get_kombisensor(kombisensor_id).map(|kombisensor| kombisensor.get_sensors())
        });

        let payload = serde_json::to_string_pretty(sensors).unwrap();
        Ok(Response::with((status::Ok, payload)))
    } else {
        Err(IronError::new(StringError("Mutex Server lock failed"),
                           status::BadRequest))
    }
}

/// Beispiel URL: http://0.0.0.0:3000/api/v1/zone/0/kombisensor/0/sensor/0
fn sensor_get(req: &mut Request,
              xmz_mod_touch_server: Arc<Mutex<Server>>)
              -> IronResult<Response> {
    if let Ok(xmz_mod_touch_server) = xmz_mod_touch_server.lock() {
        // Extract the parameter(s)
        let zone_id = req.extensions
            .get::<Router>()
            .unwrap()
            .find("zone_id")
            .unwrap_or("0")
            .parse::<usize>()
            .unwrap_or(0);
        let kombisensor_id = req.extensions
            .get::<Router>()
            .unwrap()
            .find("kombisensor_id")
            .unwrap_or("0")
            .parse::<usize>()
            .unwrap_or(0);
        let sensor_id = req.extensions
            .get::<Router>()
            .unwrap()
            .find("sensor_id")
            .unwrap_or("0")
            .parse::<usize>()
            .unwrap_or(0);

        // Get Sensor
        let sensor = &xmz_mod_touch_server.get_zone(zone_id).map(|zone| {
            zone.get_kombisensor(kombisensor_id)
                .map(|kombisensor| kombisensor.get_sensor(sensor_id))
        });

        let payload = serde_json::to_string_pretty(sensor).unwrap();
        Ok(Response::with((status::Ok, payload)))
    } else {
        Err(IronError::new(StringError("Mutex Server lock failed"),
                           status::BadRequest))
    }
}

/// Beispiel Aufruf: `curl -X PUT http://0.0.0.0:3000/api/v1/zone/0/kombisensor/0/sensor/0/reset_error_count`
fn reset_error_count(req: &mut Request,
              xmz_mod_touch_server: Arc<Mutex<Server>>)
              -> IronResult<Response> {
    if let Ok(mut xmz_mod_touch_server) = xmz_mod_touch_server.lock() {
        // Extract the parameter(s)
        let zone_id = req.extensions
            .get::<Router>()
            .unwrap()
            .find("zone_id")
            .unwrap_or("0")
            .parse::<usize>()
            .unwrap_or(0);
        let kombisensor_id = req.extensions
            .get::<Router>()
            .unwrap()
            .find("kombisensor_id")
            .unwrap_or("0")
            .parse::<usize>()
            .unwrap_or(0);
        let sensor_id = req.extensions
            .get::<Router>()
            .unwrap()
            .find("sensor_id")
            .unwrap_or("0")
            .parse::<usize>()
            .unwrap_or(0);

        // Get Sensor
        let sensor = &xmz_mod_touch_server.get_zone_mut(zone_id).map(|zone| {
            zone.get_kombisensor_mut(kombisensor_id)
                .map(|kombisensor| kombisensor.get_sensor_mut(sensor_id))
                .map(|sensor| {
                    match sensor {
                        Some(sensor) => sensor.reset_error_count(),
                        None => {}
                    }
                })
        });


        Ok(Response::with(status::Ok))
    } else {
        Err(IronError::new(StringError("Mutex Server lock failed"),
                           status::BadRequest))
    }
}

/// Beispiel URL: http://0.0.0.0:3000/api/v1/exceptions
fn exceptions_index(_req: &mut Request,
                    xmz_mod_touch_server: Arc<Mutex<Server>>)
                    -> IronResult<Response> {
    if let Ok(xmz_mod_touch_server) = xmz_mod_touch_server.lock() {
        let payload = serde_json::to_string_pretty(&*xmz_mod_touch_server.get_exceptions())
            .unwrap();
        Ok(Response::with((status::Ok, payload)))
    } else {
        Err(IronError::new(StringError("Mutex Server lock failed"),
                           status::BadRequest))
    }
}

/// Initialisiert das Webinterface
///
/// In dieser Funktion ist das gesammte Webinterface definiert.
pub fn init(xmz_mod_touch_server: Arc<Mutex<Server>>)
            -> Result<()>
{
    let mut router = Router::new();

    /// Index Route
    ///
    /// `curl http://0.0.0.0:3000`
    let xmz_mod_touch_server_clone = xmz_mod_touch_server.clone();
    router.get("/",
               move |req: &mut Request| index(req, xmz_mod_touch_server_clone.clone()),
               "index");

    /// Catch All Route
    ///
    /// Die so genannte "Catch All" Route leitet alle GET Anfragen, für die es keine
    /// handler Funktionen (index, zones_index, kombisensor_get, ...) existieren
    /// auf die Index Funktion weiter.
    ///
    /// `curl http://0.0.0.0:3000`
    let xmz_mod_touch_server_clone = xmz_mod_touch_server.clone();
    router.get("/*",
               move |req: &mut Request| index(req, xmz_mod_touch_server_clone.clone()),
               "catch_all");

    /// `curl http://0.0.0.0:3000/api/v1`
    let xmz_mod_touch_server_clone = xmz_mod_touch_server.clone();
    router.get("/api/v1",
               move |req: &mut Request| index(req, xmz_mod_touch_server_clone.clone()),
               "index_with_api");

    /// `curl http://0.0.0.0:3000/api/v1/zones`
    let xmz_mod_touch_server_clone = xmz_mod_touch_server.clone();
    router.get("/api/v1/zones",
               move |req: &mut Request| zones_index(req, xmz_mod_touch_server_clone.clone()),
               "zones_index");

    /// `curl http://0.0.0.0:3000/api/v1/zone/0`
    let xmz_mod_touch_server_clone = xmz_mod_touch_server.clone();
    router.get("/api/v1/zone/:zone_id",
               move |req: &mut Request| zone_get(req, xmz_mod_touch_server_clone.clone()),
               "zone_get");

    /// `curl http://0.0.0.0:3000/api/v1/zone/0/kombisensors`
    let xmz_mod_touch_server_clone = xmz_mod_touch_server.clone();
    router.get("/api/v1/zone/:zone_id/kombisensors",
               move |req: &mut Request| {
                   kombisensors_index(req, xmz_mod_touch_server_clone.clone())
               },
               "kombisensors_index");

    /// `curl http://0.0.0.0:3000/api/v1/zone/0/kombisensor/0`
    let xmz_mod_touch_server_clone = xmz_mod_touch_server.clone();
    router.get("/api/v1/zone/:zone_id/kombisensor/:kombisensor_id",
               move |req: &mut Request| kombisensor_get(req, xmz_mod_touch_server_clone.clone()),
               "kombisensor_get");

    /// `curl http://0.0.0.0:3000/api/v1/zone/0/kombisensor/0/sensors`
    let xmz_mod_touch_server_clone = xmz_mod_touch_server.clone();
    router.get("/api/v1/zone/:zone_id/kombisensor/:kombisensor_id/sensors",
               move |req: &mut Request| sensors_index(req, xmz_mod_touch_server_clone.clone()),
               "sensors_index");

    /// `curl http://0.0.0.0:3000/api/v1/zone/0/kombisensor/0/sensor/0`
    let xmz_mod_touch_server_clone = xmz_mod_touch_server.clone();
    router.get("/api/v1/zone/:zone_id/kombisensor/:kombisensor_id/sensor/:sensor_id",
               move |req: &mut Request| sensor_get(req, xmz_mod_touch_server_clone.clone()),
               "sensor_get");

    /// `curl -X PUT http://0.0.0.0:3000/api/v1/zone/0/kombisensor/0/sensor/0/reset_error_count`
    let xmz_mod_touch_server_clone = xmz_mod_touch_server.clone();
    router.put("/api/v1/zone/:zone_id/kombisensor/:kombisensor_id/sensor/:sensor_id/reset_error_count",
            move |req: &mut Request| reset_error_count(req, xmz_mod_touch_server_clone.clone()),
            "reset_error_count");


    // /// `curl -X POST -d "/dev/ttyS0" http://0.0.0.0:3000/api/v1/zone/0/kombisensor/0/sensor/0/reset_error_count`
    // let xmz_mod_touch_server_clone = xmz_mod_touch_server.clone();
    // router.put("/api/v1/zone/:zone_id/kombisensor/:kombisensor_id/sensor/:sensor_id/reset_error_count",
    //          move |req: &mut Request| reset_error_count(req, xmz_mod_touch_server_clone.clone()),
    //          "reset_error_count");


     /// `curl http://0.0.0.0:3000/api/v1/exceptions`
     let xmz_mod_touch_server_clone = xmz_mod_touch_server.clone();
     router.get("/api/v1/exceptions",
             move |req: &mut Request| exceptions_index(req, xmz_mod_touch_server_clone.clone()),
             "exceptions_index");


    println!("Webinterface: http://0.0.0.0:3000");
    Iron::new(router).http("0.0.0.0:3000").unwrap();

    Ok(())
}
