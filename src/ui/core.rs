use crate::ui::{commands, dialog};
use crossbeam::channel::unbounded;

pub fn start() {
    let (ui_tx, ui_rx) = unbounded::<commands::UI>();
    let (net_tx, ui_rx) = unbounded::<commands::Net>();

    let mut siv = cursive::default();

    dialog::interface::show_interface_select_dialog(&mut siv);

    siv.run();
}
