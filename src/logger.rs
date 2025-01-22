use std::{fs::OpenOptions, time::SystemTime};

use anyhow::{Context, Result};

pub fn set_up_logger() {
    let mut logger = set_up_console_logger();
    if let Ok(file_logger) = set_up_file_logger() {
        logger = logger.chain(file_logger);
    }

    if let Ok(deps_logger) = set_up_dependencies_logger() {
        logger = logger.chain(deps_logger);
    }

    logger.apply().expect("Failed to set up logger");
}

fn set_up_file_logger() -> Result<fern::Dispatch> {
    let path = "bloop.log";
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .context(format!("Opening log file {path}"))?;

    Ok(fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .filter(|metadata| metadata.target().contains("bloop"))
        .level(log::LevelFilter::Debug)
        .chain(log_file))
}

fn set_up_dependencies_logger() -> Result<fern::Dispatch> {
    let path = "bloop.deps.log";
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .context(format!("Opening log file {path}"))?;

    Ok(fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(log_file))
}

fn set_up_console_logger() -> fern::Dispatch {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .filter(|metadata| metadata.target().contains("bloop") && metadata.level() <= log::LevelFilter::Debug)
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
}
