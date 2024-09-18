pub mod config;
pub mod error;
pub mod logger;
pub mod net;
pub mod session_settings;
pub mod ui;

#[macro_use]
extern crate rust_i18n;

// Defining folder with locales. Path: crate-root/locales
rust_i18n::i18n!("locales", fallback = "English");

fn main() {
    logger::init(
        config::lock_get_log_level(),
        &config::lock_get_log_filename(),
    )
    .unwrap_or_else(|err| {
        println!("Logger initialization failed. Error: {}", err);
        std::process::exit(1);
    });

    log::info!("Logger initialized. Starting UI.");

    ui::core::start();
}
