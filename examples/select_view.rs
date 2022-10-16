
//! Show a SelectView with items to pick, then quit.
pub fn main() {
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
