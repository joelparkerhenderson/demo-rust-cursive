# Demo Rust Cursive

Demonstration of the Rust programming language and Cursvie crate for terminal user interface (TUI).


## Setup

Create:

```sh
cargo new demo
}
```

Add crate:

```toml
[dependencies]
cursive = "*"
}
```

You need ncurses installed.


## Examples

* [Run](examples/run.rs): 
  Create the cursive root and run it; press ctrl-c to quit.

* [add_global_callback](examples/add_global_callback.rs): 
  Add a global callback so the user can press the escape key to quit.


## Views

* [TextView](examples/text_view.rs): 
  Show a TextView that shows the text "Hello World".

* [Dialog](examples/dialog.rs): 
  Show a Dialog info box with a message and a default "Ok" button.

* [SelectView](examples/select_view.rs):
  Show a SelectView with items to pick, then quit.


### Layouts

* [LinearLayout::vertical](examples/linear_layout_vertical.rs):
  Use a linear layout manager to arrange items vertically.

* [LinearLayout::horizontal](examples/linear_layout_horizontal.rs):
  Use a linear layout manager to arrange items horizontally.


### Clickables

* [Button](examples/button.rs): 
  Show a button that the user can click to quit.

* [Checkbox](examples/checkbox.rs):
  Show a Checkbox; press space or enter to toggle.

* [RadioGroup & RadioButton](examples/radio_group_radio_button.rs)::
  Show a RadioGroup and RadioButton items using a LinearLayout.


### More

* [EditView & Resizable](examples/edit_view_and_resizable.rs):
  Show an EditView with a Resizable.

* [Set UserData](examples/set_user_data.rs):
  Set user data via a custom struct with fields we want.

* [Event loop & runner & step](examples/event_loop_and_runner_and_step.rs):
  Run a custom event loop with a runner and its step function.


## Themes

* [Theme & Palette & BorderStyle](examples/theme_and_palette_and_border_style.rs):
  Theme settings with shadow, BorderStyle borders, and Palette colors.

* [Theme & Palette with Green on Black](examples/theme_and_palette_with_green_on_black.rs):
  Theme settings with green foreground colors on black background colors.

* [Theme & Palette with TerminalDefault](examples/theme_and_palette_with_terminal_default.rs): 
  Theme palette customized with terminal default colors.

* [Add fullscreen layer](examples/add_fullscreen_layer.rs):
  Add a fullscreen layer i.e. a view with zero margin and zero padding.


### Edges

* [set_window_title](examples/set_window_title.rs): 
  Set window title, which works on some systems, yet not on others.

* [menubar](examples/menubar.rs):
  Show a menu bar at the top of the screen; press the escape key to use the menu.


## Tips

Convert `TextView` content to a `&str`:

```rust
main() {
    let text_view = cursive::views::TextView::new("Hello World");
    let s: &str = text_view.get_content().source();
}
```

## Backends

Cursive can use various backends. In priority order:

* blt-backend

* termion-backend
  
* crossterm-backend

* pancurses-backend

* ncurses-backend

* run_dummy
