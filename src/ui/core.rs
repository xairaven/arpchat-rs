use crate::ui::dialog;

pub fn start() {
    let mut siv = cursive::default();

    dialog::interface::show_interface_select_dialog(&mut siv);

    siv.run();
}
