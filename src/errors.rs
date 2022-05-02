use fern::colors::{Color, ColoredLevelConfig};
use clap::ArgEnum;

#[derive(Debug, Copy, Clone, PartialEq, Eq, ArgEnum)]
pub enum LogLevel {
    Critical,
    Info,
}

pub fn setup_logging(verbosity: &LogLevel) -> Result<(), fern::InitError> {
    let mut base_config = fern::Dispatch::new();
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .debug(Color::Yellow)
        .warn(Color::Magenta);

    base_config = match verbosity {
        LogLevel::Critical => base_config.level(log::LevelFilter::Error),
        LogLevel::Info => base_config.level(log::LevelFilter::Info),
    };

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
        .chain(std::io::stdout());

    base_config
        .chain(stdout_config)
        .apply()?;
    Ok(())
}
