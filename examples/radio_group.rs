//! Show a RadioGroup and RadioButton items within a LinearLayout.
fn main() {
    let mut c = cursive::default();
    let mut radio_group: cursive::views::RadioGroup<String> = cursive::views::RadioGroup::new()
    .on_change(|c, value: &String| {
        let dialog = cursive::views::Dialog::info(value);
        c.add_layer(dialog);
    });
    let linear_layout = cursive::views::LinearLayout::vertical()
    .child(radio_group.button_str("Demo 1"))
    .child(radio_group.button_str("Demo 2"))
    .child(radio_group.button_str("Demo 3"));
    c.add_layer(linear_layout);
    c.run();
}
