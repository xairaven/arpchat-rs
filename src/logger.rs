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
        .map_err(LoggerError::SetLoggerError)
}
