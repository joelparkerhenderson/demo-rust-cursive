//! Set window title, which works on some systems, yet not on others.
pub fn main() {
    let mut c = cursive::default();
    c.set_window_title("Demo Title");
    c.run();
}
