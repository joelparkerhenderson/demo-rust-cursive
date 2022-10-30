//! Implement the ViewWrapper trait to wrap a view and intercept events.

pub struct MyView {
    pub button: cursive::views::Button,
}

impl cursive::view::ViewWrapper for MyView {

    // Use the macro `wrap_impl!` which creates the typical functionality.
    cursive::wrap_impl!(self.button: cursive::views::Button);

    // Demonstrate how to get events, process them, and send them along.
    // For example, this demonstration 1) gets all events, 2) selects all
    // character events, 3) sets the button label to the typed character.
    // This is merely an easy way to show the user that the code works.
    fn wrap_on_event(&mut self, event: cursive::event::Event) -> cursive::event::EventResult {
        match event {
            cursive::event::Event::Char(c) => {
                self.button.set_label(c);
                cursive::event::EventResult::Consumed(None)
            },
            _ => cursive::event::EventResult::Ignored,
        }
    }
}

pub fn main() {
    let mut c = cursive::default();
    let button = cursive::views::Button::new("Example Button", |s| { s.quit() });
    let view = MyView {button: button};
    c.add_layer(view);
    c.run();
}

