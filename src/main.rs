fn main() {
    demo_run();
}

fn demo_run() {
    let mut c = cursive::default();
    c.run();
}

fn demo_add_global_callback() {
    let mut c = cursive::default();
    c.add_global_callback(cursive::event::Key::Esc, |s| s.quit());
    c.run();
}
