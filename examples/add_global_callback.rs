//! Add global callback, so the user can press the escape key to quit.
pub fn main() {
    let mut c = cursive::default();
    c.add_global_callback(cursive::event::Key::Esc, |c| c.quit());
    c.run();
}
