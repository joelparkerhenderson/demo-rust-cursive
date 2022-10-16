//! Show a Dialog info box with a message and a default "Ok" button.
pub fn main() {
    let mut c = cursive::default();
    let view = cursive::views::Dialog::info("Hello World");
    c.add_layer(view);
    c.run();
}
