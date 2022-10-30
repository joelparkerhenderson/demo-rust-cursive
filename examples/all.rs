//! Show a menu bar with a few subtree menus and a few leaf items.

use cursive::{
    view::{Nameable, Resizable, View},
};

pub fn main() {
    let mut c = cursive::default();
    initialize_menus(&mut c);
    let status = c.status_bar("Ready");
    let mut runner = c.runner();
    while runner.is_running() {
        //c.cb_sink().send(Box::new(|s| s.set_status_bar_content(format!("{:?}", secs())))).unwrap();
        status.set_content(format!("{:?}", secs()));
        runner.refresh();
        runner.step();
    }
}

/// Get the current time as seconds since the epoch.
pub fn secs() -> u64 { 
    std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .expect("now").as_secs()
}

/// Initialize the menus.
pub fn initialize_menus(c: &mut cursive::Cursive) {
    c.menubar()
    .add_subtree(
        "File",
        cursive::menu::Tree::new()
        .leaf("Quit", |c| c.quit())
    )
    .add_subtree(
        "Edit",
        cursive::menu::Tree::new()
        .leaf("Find", |c| c.add_layer(cursive::views::Dialog::info("Find")))
    )
    .add_subtree(
        "Help",
        cursive::menu::Tree::new()
        .leaf("About", |c| c.add_layer(cursive::views::Dialog::info("About")))
    );
    c.set_autohide_menu(false);
    c.add_global_callback(cursive::event::Key::Esc, |c| c.select_menubar());
}

////
// 
// StatusBarExt
//
////

//use cursive::view::{Nameable, Resizable};
//use cursive::view::View;

/// Show a status bar line at the bottom of the screen.
pub trait StatusBarExt {
    fn status_bar(&mut self, content: impl Into<cursive::utils::span::SpannedString<cursive::theme::Style>>) -> cursive::views::TextContent;
    fn get_status_bar_content(&mut self) -> cursive::views::TextContentRef;
    fn set_status_bar_content(&mut self, content: impl Into<cursive::utils::span::SpannedString<cursive::theme::Style>>);
}

impl StatusBarExt for cursive::Cursive {

    /// Create a new status bar, set to the given content.
    fn status_bar(&mut self, content: impl Into<cursive::utils::span::SpannedString<cursive::theme::Style>>) -> cursive::views::TextContent {
        let text_content = cursive::views::TextContent::new(content);
        self.screen_mut()
        .add_transparent_layer(
            cursive::views::OnLayoutView::new(
                cursive::views::FixedLayout::new().child(
                    cursive::Rect::from_point(cursive::Vec2::zero()),
                    cursive::views::Layer::new(
                        cursive::views::TextView::new_with_content(text_content.clone()).with_name("status"),
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
        text_content
    }

    fn get_status_bar_content(&mut self) -> cursive::views::TextContentRef {
        self.call_on_name("status", |text_view: &mut cursive::views::TextView| {
            text_view.get_content()
        })
        .expect("get_status")
    }    

    fn set_status_bar_content(&mut self, content: impl Into<cursive::utils::span::SpannedString<cursive::theme::Style>>) {
        self.call_on_name("status", |text_view: &mut cursive::views::TextView| {
            text_view.set_content(content);
        })
        .expect("set_status")
    }    

}
