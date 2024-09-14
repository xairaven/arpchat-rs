pub mod config;
pub mod error;
pub mod logger;
pub mod net;
pub mod ui;

#[macro_use]
extern crate rust_i18n;
extern crate core;

// Defining folder with locales. Path: crate-root/locales
rust_i18n::i18n!("locales", fallback = "English");

const LOG_FILENAME: &str = "log.txt";

fn main() {
    logger::init(log::LevelFilter::Info, LOG_FILENAME).unwrap_or_else(|err| {
        log::error!("Error: {err}");
        std::process::exit(1);
    });

    log::info!("Logger initialized. Starting UI.");

    ui::core::start();
}
