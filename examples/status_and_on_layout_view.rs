//! Show a status line at the bottom of the screen.
//! 
use cursive::view::{Nameable, Resizable};
use cursive::view::View; // For the `layout` function

fn main() {
    let mut c = cursive::default();

    // Get a mutable reference to the currently active screen.
    c.screen_mut()

    // Add the status bar by using a transparent layer.
    .add_transparent_layer(

        // Keep the status line at bottom by using OnLayoutView for a custom layout.
        cursive::views::OnLayoutView::new(

            // Use FixedLayout to arrange children using the layout closure.
            cursive::views::FixedLayout::new().child(

                // Position is (0,0)
                cursive::Rect::from_point(cursive::Vec2::zero()),

                // Add the status as a layer.
                cursive::views::Layer::new(

                    // Use a typical TextView that wraps the status string.
                    cursive::views::TextView::new("Hello World").with_name("status"),
            
                )
                .full_width(),
            ),

            // Customize the layout to keep the status line at bottom
            |layout, size| {
                // We could also keep the status bar at the top instead.
                layout.set_child_position(
                    0,
                    cursive::Rect::from_size((0, size.y - 1), (size.x, 1)),
                );
                layout.layout(size);
            },
        )
        .full_screen(),
    );

    c.run();
}
