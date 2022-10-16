//! Show an EditView with a Resizable view.
pub fn main() {
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

