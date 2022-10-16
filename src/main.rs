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
