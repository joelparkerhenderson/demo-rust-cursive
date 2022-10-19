//! Run a custom event loop with a runner and its step function.

fn main() {
    let mut c = cursive::default();

    // Create a content string that Cursive can change later on.
    let mut content = cursive::views::TextContent::new("");

    // Show the content string the typical way.
    c.add_layer(cursive::views::TextView::new_with_content(content.clone()));

    // Get the current runner backend, such as ncurses, pancurses, etc.
    let mut runner = c.runner();

    // Loop forever until the user quits e.g. press CTRL-C.
    while runner.is_running() {

        // Get the current time as seconds.
        let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("now").as_secs();
    
        // Update the content with the current time.
        content.set_content(format!("{:?}", secs));

        // Refresh the screen with the current view tree state.
        runner.refresh();

        // Call the next Cursive runner step.
        runner.step();

    }

}
