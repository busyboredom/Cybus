// std::log is only a wrapper for existing loggers, designed to help with portability and
// maintenance down the road. The following code prepares a logging implementation for std::log.

use fern::colors;
use std::io;

pub fn setup_logger(verbosity: u8) -> Result<(), fern::InitError> {
    // Get base configuration
    let mut base_config = fern::Dispatch::new();
    let colors = colors::ColoredLevelConfig::new();

    // Parse verbosity
    base_config = match verbosity {
        0 => base_config.level(log::LevelFilter::Error),

        1 => base_config.level(log::LevelFilter::Warn),

        2 => base_config.level(log::LevelFilter::Info),

        3 => base_config.level(log::LevelFilter::Debug),

        _4_or_more => base_config.level(log::LevelFilter::Trace),
    };

    // Format for files.
    let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file("cybus.log")?);

    // Format for stdout.
    let stdout_config = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now().format("%H:%M"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .chain(io::stdout());

    // Tie it all together
    base_config
        .chain(file_config)
        .chain(stdout_config)
        .apply()?;

    Ok(())
}
