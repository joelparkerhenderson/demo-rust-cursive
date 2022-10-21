//! Show a status bar line at the bottom of the screen.
//! 
//! This example shows to create and use a `StatusExt` trait extension,
//! which enables us to add new functionality to the cursive struct,
//! while also providing code encapsulation for the status functions.

use cursive::view::{Nameable, Resizable};
use cursive::view::View;

pub trait StatusExt {
    fn initialize_status(&mut self, content: impl Into<cursive::utils::span::SpannedString<cursive::theme::Style>>);
    fn get_status(&mut self) -> cursive::views::TextContentRef;
    fn set_status(&mut self, content: impl Into<cursive::utils::span::SpannedString<cursive::theme::Style>>);
}

impl StatusExt for cursive::Cursive {

    fn initialize_status(&mut self, content: impl Into<cursive::utils::span::SpannedString<cursive::theme::Style>>) {
        self.screen_mut()
        .add_transparent_layer(
            cursive::views::OnLayoutView::new(
                cursive::views::FixedLayout::new().child(
                    cursive::Rect::from_point(cursive::Vec2::zero()),
                    cursive::views::Layer::new(
                        cursive::views::TextView::new(content).with_name("status"),
                    )
                    .full_width(),
                ),
                |layout, size| {
                    let rect = cursive::Rect::from_size((0, size.y - 1), (size.x, 1));
                    layout.set_child_position(0, rect);
                    layout.layout(size);
                },
            )
            .full_screen(),
        );
    }

    fn get_status(&mut self) -> cursive::views::TextContentRef {
        self.call_on_name("status", |text_view: &mut cursive::views::TextView| {
            text_view.get_content()
        })
        .expect("get_status")
    }    

    fn set_status(&mut self, content: impl Into<cursive::utils::span::SpannedString<cursive::theme::Style>>) {
        self.call_on_name("status", |text_view: &mut cursive::views::TextView| {
            text_view.set_content(content);
        })
        .expect("set_status")
    }    

}

pub fn main() {
    let mut c = cursive::default();
    c.initialize_status("Hello World");
    c.run();
}
