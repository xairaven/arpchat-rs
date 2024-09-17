use cursive::traits::Nameable;
use cursive::utils::markup::StyledString;
use cursive::views::{LinearLayout, TextView};
use cursive::Cursive;

/// If a `TextView` with the provided name exists, set its content. Otherwise,
/// append a new `TextView` to the `LinearLayout` with the provided parent name.
pub fn update_or_append_txt<S>(siv: &mut Cursive, parent_id: &str, id: &str, content: S)
where
    S: Into<StyledString> + Clone,
{
    let mut updated = false;
    siv.call_on_name(id, |child: &mut TextView| {
        child.set_content(content.clone());
        updated = true;
    });

    if !updated {
        siv.call_on_name(parent_id, |parent: &mut LinearLayout| {
            parent.add_child(TextView::new(content).with_name(id));
        });
    }
}

/// Append a new `TextView` to the `LinearLayout` with the provided parent name.
pub fn append_txt<S>(siv: &mut Cursive, parent_id: &str, content: S)
where
    S: Into<StyledString> + Clone,
{
    siv.call_on_name(parent_id, |parent: &mut LinearLayout| {
        parent.add_child(TextView::new(content));
    });
}
