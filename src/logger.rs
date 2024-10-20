use crate::error::logger::LoggerError;
use chrono::{Datelike, Local};
use log::LevelFilter;

pub fn init(log_level: LevelFilter) -> Result<(), LoggerError> {
    let mut dispatcher = fern::Dispatch::new().level(log_level);

    if log_level != LevelFilter::Off {
        let file_name = generate_log_name();

        let file =
            fern::log_file(file_name).map_err(|_| LoggerError::CannotAccessLogFile)?;

        dispatcher = dispatcher
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{} {} {}] {}",
                    Local::now().format("%Y-%m-%d %H:%M"),
                    record.level(),
                    record.target(),
                    message
                ))
            })
            .chain(fern::Output::file(file, "\r\n"));
    }

    dispatcher.apply().map_err(LoggerError::SetLoggerError)
}

pub fn generate_log_name() -> String {
    let now = Local::now();
    let date = format!(
        "{year:04}-{month:02}-{day:02}",
        year = now.year(),
        month = now.month(),
        day = now.day(),
    );

    format!("X-ARPCHAT-LOG_{date}.log")
}
