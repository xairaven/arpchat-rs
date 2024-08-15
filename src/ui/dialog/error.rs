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
            .button(t!("button.quit"), |siv| siv.quit()),
    );
}
