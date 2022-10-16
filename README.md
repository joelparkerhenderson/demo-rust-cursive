# Demo Rust Cursive

Demonstration of the Rust programming language and Cursvie crate for terminal user interface (TUI).


## Setup

Create:

```sh
cargo new demo
```

Add crate:

```toml
[dependencies]
cursive = "*"
```

You need ncurses installed.


## Run

Create the cursive root and run it; press ctrl-c to quit.

```rust
fn main() {
    let mut c = cursive::default(); // Create the cursive root
    c.run();
}
```


## Add global callback

Add global callback, so the user can press the escape key to quit.

```rust
fn main() {
    let mut c = cursive::default();
    c.add_global_callback(cursive::event::Key::Esc, |s| s.quit());
    c.run();
}
```


## TextView

Show a TextView.

```rust
fn main() {
    let mut c = cursive::default();
    let view = cursive::views::TextView::new("Hello World");
    c.add_layer(view);
    c.run();
}
```


## Dialog

Show a Dialog infobox with a message and a default "Ok" button.

```rust
fn main() {
    let mut c = cursive::default();
    let view = cursive::views::Dialog::info("Hello World");
    c.add_layer(view);
    c.run();
}
```


## SelectView

Show a SelectView with items to pick, then quit.

```rust
fn main() {
    let mut c = cursive::default();
    let mut view = cursive::views::SelectView::new();
    view.add_item("Demo 1", 1);
    view.add_item("Demo 2", 2);
    view.add_item("Demo 3", 3);
    view.set_on_submit(|s, value| {
        let dialog = cursive::views::Dialog::info(value.to_string());
        s.add_layer(dialog);
    });
    c.add_layer(view);
    c.run();
}
```


## Set window title

Set window title, which works on some systems, yet not on others.

```rust
fn main() {
    let mut c = cursive::default();
    siv.set_window_title("Demo Title");
    c.run();
}
```


## Menubar

Show a menu bar at the top of the screen; press the escape key to use the menu.

```rust
fn main() {
    let mut c = cursive::default();
    c.menubar()
    .add_leaf("Quit", |s| s.quit());
    c.set_autohide_menu(false);
    c.add_global_callback(cursive::event::Key::Esc, |s| s.select_menubar());
    c.run();
}
```


## LinearLayout vertical

Use a linear layout manager to arrange items vertically.

```rust
fn main() {
    let mut c = cursive::default();
    let linear_layout = cursive::views::LinearLayout::vertical()
    .child(cursive::views::TextView::new("Demo 1"))
    .child(cursive::views::TextView::new("Demo 2"))
    .child(cursive::views::TextView::new("Demo 3"));
    c.add_layer(linear_layout);
    c.run();
}
```

## LinearLayout horizontal

Use a linear layout manager to arrange items horizontally.

```rust
fn main() {
    let mut c = cursive::default();
    let linear_layout = cursive::views::LinearLayout::horizontal()
    .child(cursive::views::TextView::new("Demo 1"))
    .child(cursive::views::TextView::new("Demo 2"))
    .child(cursive::views::TextView::new("Demo 3"));
    c.add_layer(linear_layout);
    c.run();
}
```
