use log::SetLoggerError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoggerError {
    #[error("Cannot access log file")]
    CannotAccessLogFile,

    #[error("Log init error")]
    SetLoggerError(SetLoggerError),
}
