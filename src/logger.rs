use std::{
    fs::OpenOptions,
    path::{Path, PathBuf},
    time::SystemTime,
};

use anyhow::{Context, Result};

pub struct LogOptions {
    log_to_console: bool,
    log_to_file: Option<PathBuf>,
    log_dependencies_to_file: Option<PathBuf>,
}

impl Default for LogOptions {
    fn default() -> Self {
        Self {
            log_to_console: true,
            log_to_file: None,
            log_dependencies_to_file: None,
        }
    }
}

#[allow(dead_code)]
impl LogOptions {
    pub fn log_to_console(mut self, log_to_console: bool) -> Self {
        self.log_to_console = log_to_console;
        self
    }

    pub fn log_to_file(mut self, log_to_file: PathBuf) -> Self {
        self.log_to_file = Some(log_to_file);
        self
    }

    pub fn log_dependencies_to_file(mut self, log_dependencies_to_file: PathBuf) -> Self {
        self.log_dependencies_to_file = Some(log_dependencies_to_file);
        self
    }
}

pub fn set_up_logger(options: LogOptions) {
    let mut logger = fern::Dispatch::new();

    if options.log_to_console {
        let file_logger = set_up_console_logger();
        logger = logger.chain(file_logger);
    }

    if let Some(path) = options.log_to_file {
        if let Ok(file_logger) = set_up_file_logger(&path) {
            logger = logger.chain(file_logger);
        }
    }

    if let Some(path) = options.log_dependencies_to_file {
        if let Ok(deps_logger) = set_up_dependencies_logger(&path) {
            logger = logger.chain(deps_logger);
        }
    }

    logger.apply().expect("Failed to set up logger");
}

fn set_up_file_logger(path: &Path) -> Result<fern::Dispatch> {
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .context(format!("Opening log file {:#?}", path))?;

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

fn set_up_dependencies_logger(path: &Path) -> Result<fern::Dispatch> {
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .context(format!("Opening log file {:#?}", path))?;

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
