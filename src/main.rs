fn main() {
    demo_run();
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
