/// Dies soll der Command Line Config Generator werden
///
/// TODO: Add clap parameters config_file name and path
/// TODO: Add clap parameter num_zones
/// TODO: Add clap parameter num_kombisensors for zone_num
/// TODO: Add clap parameter num_sensors for kombisensor_num
/// TODO: Add clap parameter max_server_uptime
extern crate clap;
extern crate env_logger;
extern crate serde_json;
extern crate xmz_mod_touch_server;

use clap::{App, Arg, ArgMatches};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use xmz_mod_touch_server::{Server, Kombisensor, KombisensorType};
use xmz_mod_touch_server::errors::*;


// Die verschiedenen Umgebungen für die eine Konfigurationsdatei erstellt werden kann
#[derive(Debug)]
enum Environment {
    Production,
    Development,
}
#[derive(Debug)]
struct Config<'a> {
    environment: Environment,
    config_file: &'a str,
    num_zones: u8,
    kombisensors: u8,
    debug: bool,
}
impl<'a> Config<'a> {
    fn new(
        environment: Environment,
        config_file: &'a str,
        num_zones: u8,
        kombisensors: u8,
        debug: bool,
    ) -> Self
{
        Config {
            environment: environment,
            config_file: config_file,
            num_zones: num_zones,
            kombisensors: kombisensors,
            debug: debug,
        }
    }
}


fn generate_config(config: &Config) -> Result<()> {
    // Config file
    let mut config_file = File::create(config.config_file)?;
    // The Server
    let mut xmz_mod_touch_server = Server::new();

    // Kombisensor Typ nach Environment
    let kombisensor_type = match config.environment {
        Environment::Development => KombisensorType::RAGasSimulation,
        Environment::Production  => KombisensorType::RAGas,
    };

    // Add Zones
    for num_zone in 1..(config.num_zones + 1) {
        xmz_mod_touch_server.add_zone();

        for num_kombisensor in 1..(config.kombisensors + 1) {
            let mut kombisensor = Kombisensor::new_with_type(kombisensor_type.clone());
            kombisensor.set_modbus_address(num_kombisensor);
            kombisensor.set_modbus_debug(config.debug);
            xmz_mod_touch_server.get_zone_mut((num_zone - 1) as usize).unwrap().add_kombisensor( kombisensor );
        }
    }

    // write to config file
    let xmz_mod_touch_server_json = serde_json::to_string_pretty(&xmz_mod_touch_server)?;
    config_file.write_all(xmz_mod_touch_server_json.as_bytes())?;

    // Zusammenfassung ausgeben
    println!("Konfiguration:");
    println!("Umgebung '{:?}', '{}' Zonen mit je '{}' Kombisensoren", config.environment, config.num_zones, config.kombisensors);
    println!("Modbus DEBUG Modus: {}", config.debug);
    println!("nach '{}' geschrieben", config.config_file);

    Ok(())
}

fn run(matches: &ArgMatches) -> Result<()> {
    let environment = match matches.value_of("environment").unwrap(){
        "production"  => Environment::Production,
        "development" => Environment::Development,
        _ => unreachable!(),
    };
    let config_file = matches.value_of("config_file").unwrap();
    let num_zones: u8 = matches.value_of("num_zones").unwrap().parse().unwrap();
    let kombisensors: u8 = matches.value_of("kombisensors").unwrap().parse().unwrap();
    let debug: bool = matches.value_of("debug").unwrap().parse().unwrap();


    let config = Config::new(environment, config_file, num_zones, kombisensors, debug);
    // println!("config {:#?}", config);
    generate_config(&config)?;

    Ok(())
}


fn main() {
    // Initalisiere Logger (erst nach diesem Aufruf sind `trace!()`, `debug!()` usw. functional)
    env_logger::init().unwrap();

    let matches = App::new("XKE - xMZ-Mod-Touch Konfigurationsdatei Ersteller")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Erstellt Konfigurationsdateien für die 'xMZ-Mod-Touch-Platform'")
        .author("Stefan Müller (zzeroo) <s.mueller@it.kls-glt.de>")
        .arg(Arg::with_name("environment")
            .help("für welche Umgebung soll die Konfigurationsdateien erstellt werden")
            .long("environment")
            .short("e")
            .possible_values(&["production", "development"])
            .takes_value(true)
            .required(true)
            .default_value("production"))
        .arg(Arg::with_name("config_file")
            .help("Name der Konfigurationsdaatei")
            .long("config_file")
            .short("c")
            .takes_value(true)
            .required(true)
            .default_value("xMZ-Mod-Touch.json"))
        .arg(Arg::with_name("num_zones")
            .help("wieviele Zone sollen konfiguriert werden")
            .long("num_zones")
            .short("n")
            .takes_value(true)
            .required(true)
            .default_value("1"))
        .arg(Arg::with_name("kombisensors")
            .help("wieviele Kombisensoren sollen pro Zone konfiguriert werden")
            .long("kombisensors")
            .short("k")
            .takes_value(true)
            .required(true)
            .default_value("2"))
        .arg(Arg::with_name("debug")
            .help("soll der Modbus DEBUG Modus gesetzt werden")
            .long("debug")
            .short("d")
            .possible_values(&["true", "false"])
            .takes_value(true)
            .required(true)
            .default_value("false"))
        .get_matches();


    if let Err(ref e) = run(&matches) {
        println!("error: {}", e);

        if let Some(cause) = e.cause() {
            println!("caused by: {}", cause);
        }

        ::std::process::exit(1);
    }
}
