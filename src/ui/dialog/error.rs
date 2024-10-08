use crate::ui;
use core::fmt;
use cursive::views::Dialog;
use cursive::Cursive;

pub fn show_try_again(siv: &mut Cursive, err: impl fmt::Display) {
    siv.add_layer(
        Dialog::text(err.to_string())
            .title(t!("title.error"))
            .button(t!("button.try_again"), |siv| {
                siv.pop_layer();
            })
            .button(t!("button.quit"), ui::core::quit),
    );
}

pub fn show(siv: &mut Cursive, err: impl fmt::Display) {
    siv.add_layer(
        Dialog::text(err.to_string())
            .title(t!("title.error"))
            .button(t!("button.ok"), |siv| {
                siv.pop_layer();
            })
            .button(t!("button.quit"), ui::core::quit),
    );
}

pub fn show_breaking(siv: &mut Cursive, err: impl fmt::Display) {
    siv.add_layer(
        Dialog::text(err.to_string())
            .title(t!("title.error"))
            .button(t!("button.quit"), ui::core::quit),
    );
}
