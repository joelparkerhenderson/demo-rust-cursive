//! Use a linear layout manager to arrange items horizontally.
pub fn main() {
    let mut c = cursive::default();
    let linear_layout = cursive::views::LinearLayout::horizontal()
    .child(cursive::views::TextView::new("Demo 1"))
    .child(cursive::views::TextView::new("Demo 2"))
    .child(cursive::views::TextView::new("Demo 3"));
    c.add_layer(linear_layout);
    c.run();
}
