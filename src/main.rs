fn main() {
    demo_checkbox();
}

/// Create the cursive root and run it; press ctrl-c to quit.
pub fn demo_run() {
    let mut c = cursive::default();
    c.run();
}

/// Add global callback, so the user can press the escape key to quit.
pub fn demo_add_global_callback() {
    let mut c = cursive::default();
    c.add_global_callback(cursive::event::Key::Esc, |c| c.quit());
    c.run();
}

/// Show a button that the user can click to quit.
pub fn demo_button() {
    let mut c = cursive::default();
    let button = cursive::views::Button::new("Quit", |c| c.quit());
    c.add_layer(button);
    c.run();
}

/// Show a TextView that shows the text "Hello World".
pub fn demo_text_view() {
    let mut c = cursive::default();
    let view = cursive::views::TextView::new("Hello World");
    c.add_layer(view);
    c.run();
}

/// Show a Dialog info box with a message and a default "Ok" button.
pub fn demo_dialog() {
    let mut c = cursive::default();
    let view = cursive::views::Dialog::info("Hello World");
    c.add_layer(view);
    c.run();
}

/// Show a SelectView with items to pick, then quit.
pub fn demo_select_view() {
    let mut c = cursive::default();
    let mut view = cursive::views::SelectView::new();
    view.add_item("Demo 1", 1);
    view.add_item("Demo 2", 2);
    view.add_item("Demo 3", 3);
    view.set_on_submit(|c, value| {
        let dialog = cursive::views::Dialog::info(value.to_string());
        c.add_layer(dialog);
    });
    c.add_layer(view);
    c.run();
}

/// Set window title, which works on some systems, yet not on others.
pub fn demo_set_window_title() {
    let mut c = cursive::default();
    c.set_window_title("Demo Title");
    c.run();
}

/// Show a menu bar at the top of the screen; press the escape key to use the menu.
pub fn demo_menubar() {
    let mut c = cursive::default();
    c.menubar()
    .add_leaf("Quit", |c| c.quit());
    c.set_autohide_menu(false);
    c.add_global_callback(cursive::event::Key::Esc, |c| c.select_menubar());
    c.run();
}

/// Use a linear layout manager to arrange items vertically.
pub fn demo_linear_layout_vertical() {
    let mut c = cursive::default();
    let linear_layout = cursive::views::LinearLayout::vertical()
    .child(cursive::views::TextView::new("Demo 1"))
    .child(cursive::views::TextView::new("Demo 2"))
    .child(cursive::views::TextView::new("Demo 3"));
    c.add_layer(linear_layout);
    c.run();
}

/// Use a linear layout manager to arrange items horizontally.
pub fn demo_linear_layout_horizontal() {
    let mut c = cursive::default();
    let linear_layout = cursive::views::LinearLayout::horizontal()
    .child(cursive::views::TextView::new("Demo 1"))
    .child(cursive::views::TextView::new("Demo 2"))
    .child(cursive::views::TextView::new("Demo 3"));
    c.add_layer(linear_layout);
    c.run();
}

/// Show an EditView with a Resizable
pub fn demo_edit_view() {
    let mut c = cursive::default();
    let edit_view = cursive::views::EditView::new()
    .max_content_width(20)
    .on_submit(|c, value| {
        let dialog = cursive::views::Dialog::info(value.to_string());
        c.add_layer(dialog);
    });
    use cursive::view::Resizable;
    c.add_layer(edit_view.fixed_width(20));
    c.run();
}

/// Show a Checkbox
pub fn demo_checkbox() {
    let mut c = cursive::default();
    let checkbox = cursive::views::Checkbox::new().on_change(|c, value| {
        let dialog = cursive::views::Dialog::info(value.to_string());
        c.add_layer(dialog);
    });
    c.add_layer(checkbox);
    c.run();
}
