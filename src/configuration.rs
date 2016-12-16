use errors::*;
use std::fs::File;

pub fn read_in() -> Result<()> {
    // This operation will fail
    File::open("xmz-server-configuration.toml")
        .chain_err(|| "unable to open the configuration file.")?;

    Ok(())
}
