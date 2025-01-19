use std::time::SystemTime;

pub fn set_up_logger() -> Result<(), fern::InitError> {
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
        .filter(|metadata| {
            if metadata.target().contains("libmdns") {
                return metadata.level() <= log::LevelFilter::Info;
            }

            if metadata.target().contains("wgpu_core") {
                return metadata.level() <= log::LevelFilter::Warn;
            }

            true
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("bloop.log")?)
        .apply()?;
    Ok(())
}
