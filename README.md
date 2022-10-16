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
