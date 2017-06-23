use serde_json::error::Error as SerdeError;

error_chain!{
    links {
        Libmodbus(::libmodbus_rs::errors::Error, ::libmodbus_rs::errors::ErrorKind);
    }

    foreign_links {
        Io(::std::io::Error) #[cfg(unix)];
        SysfsGpio(::sysfs_gpio::Error);
        Json(SerdeError);
    }
}
