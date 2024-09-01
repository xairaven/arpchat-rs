use crate::config::CONFIG;
use crate::ui;
use crate::ui::commands::UICommand;
use crossbeam::channel::Sender;
use cursive::views::SelectView;
use cursive::{traits::Resizable, views::Dialog, Cursive};

pub fn show_select_dialog(
    siv: &mut Cursive, ui_tx: Sender<UICommand>,
) {
    let locales = rust_i18n::available_locales!();

    let preferred_language_index: usize = CONFIG
        .try_lock()
        .ok()
        .and_then(|locked_config| locked_config.language.clone())
        .and_then(|config_lang| {
            locales.iter().position(|lang| lang.eq(&config_lang))
        })
        .unwrap_or_default();

    siv.add_layer(
        Dialog::new()
            .title(t!("title.language_selection"))
            .content(
                SelectView::new()
                    .with_all(locales.into_iter().map(|language| {
                        (language.to_string(), language.to_owned())
                    }))
                    .selected(preferred_language_index)
                    .on_submit(move |siv, language_name_id: &String| {
                        let result =
                            ui_tx.try_send(UICommand::SetLanguage(
                                language_name_id.to_string(),
                            ));
                        //////////////// TEMPORARY
                        rust_i18n::set_locale(language_name_id);
                        ////////////////

                        match result {
                            Ok(_) => {
                                siv.pop_layer();
                                ui::dialog::interface::show_select_dialog(
                                    siv,
                                    ui_tx.clone(),
                                );
                            },
                            Err(err) => {
                                ui::dialog::error::show_try_again(
                                    siv,
                                    err.to_string(),
                                );
                            },
                        }
                    }),
            )
            .button(t!("button.quit"), |siv| ui::core::quit(siv))
            .min_width(32),
    );
}
