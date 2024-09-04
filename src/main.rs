pub mod config;
pub mod error;
pub mod net;
pub mod ui;

#[macro_use]
extern crate rust_i18n;
extern crate core;

// Defining folder with locales. Path: crate-root/locales
rust_i18n::i18n!("locales", fallback = "English");

fn main() {
    ui::core::start();
}
