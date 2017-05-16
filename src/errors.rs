error_chain!{
    foreign_links {
        Io(::std::io::Error) #[cfg(unix)];
        SerdeJson(::serde_json::Error);
        SysfsGpio(::sysfs_gpio::Error);
    }
}
