use crate::config::CONFIG;
use crate::error::logger::LoggerError;
use chrono::Local;
use log::LevelFilter;

pub fn init(log_level: LevelFilter, file_name: &str) -> Result<(), LoggerError> {
    let file = fern::log_file(file_name).map_err(|_| LoggerError::CannotAccessLogFile)?;

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                Local::now().format("%Y-%m-%d %H:%M"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log_level)
        .chain(fern::Output::file(file, "\r\n"))
        .apply()
        .map_err(|err| LoggerError::SetLoggerError(err))
}

pub fn config_filename() -> String {
    if let Ok(config) = CONFIG.try_lock() {
        if let Some(filename) = &config.log_filename {
            return filename.to_string();
        }
    }

    const DEFAULT_FILENAME: &str = "log.txt";
    DEFAULT_FILENAME.to_string()
}

pub fn config_log_level() -> LevelFilter {
    if let Ok(config) = CONFIG.try_lock() {
        let level_option = config.get_log_level();
        if let Some(level) = level_option {
            return level;
        }
    }

    // TODO: To set up writing log level & log_filename to config
    const DEFAULT_LOG_LEVEL: LevelFilter = LevelFilter::Info;
    DEFAULT_LOG_LEVEL
}
