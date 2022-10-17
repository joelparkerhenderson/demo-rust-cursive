//! Add a fullscreen layer i.e. a view with zero margin and zero padding.
pub fn main() {
    let mut c = cursive::default();
    let view = cursive::views::TextView::new("Hello World");
    c.add_fullscreen_layer(view.full_screen());
    c.run();
}
