use anyhow::anyhow;
use log::LevelFilter;
use simplelog::{Config, WriteLogger};
use std::{env, fs, io, path::PathBuf};

pub fn get_default_log_path() -> anyhow::Result<PathBuf> {
    let dir =
        env::current_exe().map_err(|_| anyhow::anyhow!("could not identify path of executable"))?;
    let log_path = dir.parent().unwrap().to_owned().join("postal.log");
    Ok(log_path)
}

pub fn init_logging() -> anyhow::Result<()> {
    let log_path = get_default_log_path()?;

    let file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .truncate(true)
        .open(log_path)?;

    WriteLogger::init(LevelFilter::Trace, Config::default(), file)?;
    log::trace!("initialised logging");
    Ok(())
}
