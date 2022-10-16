//! Show a menu bar at the top of the screen; press the escape key to use the menu.
pub fn main() {
    let mut c = cursive::default();
    c.menubar()
    .add_leaf("Quit", |c| c.quit());
    c.set_autohide_menu(false);
    c.add_global_callback(cursive::event::Key::Esc, |c| c.select_menubar());
    c.run();
}
