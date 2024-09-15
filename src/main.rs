pub mod config;
pub mod error;
pub mod logger;
pub mod net;
pub mod session;
pub mod ui;

#[macro_use]
extern crate rust_i18n;
extern crate core;

// Defining folder with locales. Path: crate-root/locales
rust_i18n::i18n!("locales", fallback = "English");

fn main() {
    // TODO: Logger settings & To set up writing log level & log_filename to config
    logger::init(
        config::DEFAULT_LOG_LEVEL_FILTER,
        config::DEFAULT_LOG_FILENAME,
    )
    .unwrap_or_else(|err| {
        log::error!("Error: {err}");
        std::process::exit(1);
    });

    log::info!("Logger initialized. Starting UI.");

    ui::core::start();
}
