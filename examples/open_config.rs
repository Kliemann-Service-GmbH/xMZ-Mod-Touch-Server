/// TODO: Dokument das

use std::error::Error;
use std::fs::File;
use std::path::Path;


fn main() {
    let path_boot = Path::new("/boot/fstab");
    let path_fstab = Path::new("/etc/fstab");
    let display_fstab = path_fstab.display();

    let file = match File::open(&path_boot) {
        Ok(file) => file,
        Err(_) => {
            match File::open(&path_fstab) {
                Ok(file) => file,
                Err(err) => panic!("Could not open {}: {}", display_fstab, err.description()),
            }
        }
    };

    println!("{:?}", file);
}