//! Show a menu bar with a few subtree menus and leaf items.
pub fn main() {
    let mut c = cursive::default();
    c.menubar()
    .add_subtree(
        "File",
        cursive::menu::Tree::new()
        .leaf("Quit", |c| c.quit())
    )
    .add_subtree(
        "Edit",
        cursive::menu::Tree::new()
        .leaf("Find", |c| c.add_layer(cursive::views::Dialog::info("Demo")))
    )
    .add_subtree(
        "Help",
        cursive::menu::Tree::new()
        .leaf("About", |c| c.add_layer(cursive::views::Dialog::info("Demo")))
    );
    c.set_autohide_menu(false);
    c.add_global_callback(cursive::event::Key::Esc, |c| c.select_menubar());
    c.run();
}
