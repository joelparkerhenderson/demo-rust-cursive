//! Show a TextView that shows the text "Hello World".
pub fn main() {
    let mut c = cursive::default();
    let view = cursive::views::TextView::new("Hello World");
    c.add_layer(view);
    c.run();
}
