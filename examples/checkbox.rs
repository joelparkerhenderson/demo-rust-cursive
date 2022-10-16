//! Show a Checkbox; press space or enter to toggle.
pub fn main() {
    let mut c = cursive::default();
    let checkbox = cursive::views::Checkbox::new()
    .on_change(|c, value| {
        let dialog = cursive::views::Dialog::info(value.to_string());
        c.add_layer(dialog);
    });
    c.add_layer(checkbox);
    c.run();
}
