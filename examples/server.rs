extern crate xmz_server;
use xmz_server::server::Server;

fn main() {
    let mut server = Server::new();
    server.init();
    server.update_sensors();

    for modul in server.modules {
        println!("Modul {}", modul.modbus_address);
        for sensor in modul.sensors {
            println!("Sensor: {} {}", sensor.concentration().unwrap(), sensor.si);
        }
    }
}
