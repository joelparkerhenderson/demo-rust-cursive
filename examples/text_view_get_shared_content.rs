//! Get a text view shared content reference, to do content mutation.

fn main() {
    let mut c = cursive::default();
    let mut text_view = cursive::views::TextView::new("Hello World");
    let text_content = text_view.get_shared_content();
    text_content.set_content("Lorem Ipsum");
    c.add_layer(text_view);
    c.run();
}
