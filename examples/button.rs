//! Show a button that the user can click to quit.
pub fn main() {
    let mut c = cursive::default();
    let button = cursive::views::Button::new("Quit", |c| c.quit());
    c.add_layer(button);
    c.run();
}
