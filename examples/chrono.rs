extern crate chrono;

use std::thread;

fn main() {
    use chrono::{Duration, UTC};
    use std::time::Duration as StdDuration;

    let startup_time = chrono::UTC::now();
    thread::sleep(Duration::milliseconds(100).to_std().unwrap());

    if chrono::UTC::now().signed_duration_since(startup_time) > chrono::Duration::milliseconds(500) {
        println!("Mehr als 500ms vergangen");
    }
}
