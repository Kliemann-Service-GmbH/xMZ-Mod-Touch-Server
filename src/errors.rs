//! Fehlerbehandlung mit [error-chain](https://github.com/brson/error-chain)
//!
//! https://github.com/brson/error-chain
//!
error_chain!{
    links {
        Libmodbus(::libmodbus_rs::errors::Error, ::libmodbus_rs::errors::ErrorKind);
    }

    foreign_links {
        Io(::std::io::Error) #[cfg(unix)];
        SerdeJson(::serde_json::Error);
        SysfsGpio(::sysfs_gpio::Error);
    }
}
