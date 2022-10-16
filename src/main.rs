fn main() {
    demo_linear_layout_vertical();
}

pub fn demo_run() {
    let mut c = cursive::default();
    c.run();
}

pub fn demo_add_global_callback() {
    let mut c = cursive::default();
    c.add_global_callback(cursive::event::Key::Esc, |s| s.quit());
    c.run();
}

pub fn demo_text_view() {
    let mut c = cursive::default();
    let view = cursive::views::TextView::new("Hello World");
    c.add_layer(view);
    c.run();
}

pub fn demo_dialog() {
    let mut c = cursive::default();
    let view = cursive::views::Dialog::info("Hello World");
    c.add_layer(view);
    c.run();
}

pub fn demo_select_view() {
    let mut c = cursive::default();
    let mut view = cursive::views::SelectView::new();
    view.add_item("Demo 1", 1);
    view.add_item("Demo 2", 2);
    view.add_item("Demo 3", 3);
    view.set_on_submit(|s, value| {
        let dialog = cursive::views::Dialog::info(value.to_string());
        s.add_layer(dialog);
    });
    c.add_layer(view);
    c.run();
}

pub fn demo_set_window_title() {
    let mut c = cursive::default();
    c.set_window_title("Demo Title");
    c.run();
}

pub fn demo_menubar() {
    let mut c = cursive::default();
    c.menubar()
    .add_leaf("Quit", |s| s.quit());
    c.set_autohide_menu(false);
    c.add_global_callback(cursive::event::Key::Esc, |s| s.select_menubar());
    c.run();
}

pub fn demo_linear_layout_vertical() {
    let mut c = cursive::default();
    let linear_layout = cursive::views::LinearLayout::vertical()
    .child(cursive::views::TextView::new("Demo 1"))
    .child(cursive::views::TextView::new("Demo 2"))
    .child(cursive::views::TextView::new("Demo 3"));
    c.add_layer(linear_layout);
    c.run();
}

