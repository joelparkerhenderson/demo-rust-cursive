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

You need a Cursive backend installed, such as ncurses, pancurses, etc.


## Examples

* [Run](examples/run.rs): 
  Create the cursive root and run it; press ctrl-c to quit.

* [add_global_callback](examples/add_global_callback.rs): 
  Add a global callback so the user can press the escape key to quit.


## Views

* [TextView](examples/text_view.rs): 
  Show a TextView that shows the text "Hello World".

* [TextView get_shared_content](examples/text_view_get_shared_content.rs)
  Get a text view shared content reference, to do content mutation.

* [Dialog](examples/dialog.rs): 
  Show a Dialog info box with a message and a default "Ok" button.

* [SelectView](examples/select_view.rs):
  Show a SelectView with items to pick, then quit.

* [ViewWrapper](examples/view_wrapper.rs):
  Implement the ViewWrapper trait to wrap a view and intercept events.


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

* [Event loop & runner & step](examples/event_loop_runner_step.rs):
  Run a custom event loop with a runner and its step function.


## Themes

* [Theme & Palette & BorderStyle](examples/theme_and_palette_and_border_style.rs):
  Theme settings Palette colors, BorderStyle borders, and shadow flag.

* [Theme & Palette with Green on Black](examples/theme_and_palette_with_green_on_black.rs):
  Theme settings with green foreground colors on black background colors.

* [Theme & Palette with TerminalDefault](examples/theme_and_palette_with_terminal_default.rs): 
  Theme settings customized with terminal default colors.

* [Add fullscreen layer](examples/add_fullscreen_layer.rs):
  Add a fullscreen layer i.e. a view with zero margin and zero padding.


### Edges

* [set_window_title](examples/set_window_title.rs): 
  Set window title, which works on some systems, yet not on others.

* [menubar](examples/menubar.rs):
  Show a menu bar at the top of the screen; press the escape key to use the menu.

* [menubar & subtree & leaf](examples/menubar_and_subtree_and_leaf.rs):
   Show a menu bar with a few subtree menus and a few leaf items.

* [Status & OnLayoutView](examples/status_and_on_layout_view.rs):
  Show a status bar line at the bottom of the screen.

* [StatusBarExt trait](examples/status_bar_ext.rs):
  Show a status bar line by creating a custom trait extension.


## Tips

Convert `TextView` content to a `&str`:

```rust
main() {
    let text_view = cursive::views::TextView::new("Hello World");
    let s: &str = text_view.get_content().source();
}
```

## Backends

Cursive can use various backends. We prefer to use the Termion backend. Discussion below.

Cursive will choose the backend to use, with this priority order:

* blt-backend

* [termion-backend](examples/termion_backend.rs): pure-Rust Unix-specific backend
  
* [crossterm-backend](examples/crossterm_backend.rs): pure-Rust cross-platform backend

* pancurses-backend

* ncurses-backend: default on Unix

* run_dummy


### ncurses considerations

Discussion from [source](https://github.com/gyscos/cursive/issues/411):

* The `ncurses` crate is wildly unsound. It simply wraps calls to C functions in Rust functions and declares them safe, with no validation whatsoever. It has format string vulnerabilities, returns invalid UTF-8 in &str, and so much other unsoundness that you can cause pretty much arbitrary memory corruption. It is also unmaintained. See Tracker: All unsafe blocks must be removed and then re-added one by one after careful verification of actual safety. 

* The `pancurses` crate depends on ncurses and inherits the issues.

* One of the reasons `ncurses` still the default backend is the input it supports: things like Ctrl+F1, Ctrl+Insert are currently working on the ncurses backend. Last time I quickly surveyed the other backends, none of them were returning enough information to implement that (even if it's raw input code from the terminal to be parsed in cursive itself). It's not critical to support these modifiers, but I was a bit sad having to cut features.


### Crossterm discussion

Does Crossterm work on Apple macOS with M1/M2/ARM/AARCH?

* Crossterm seems cause unresponsive terminals on my system, which is macOS Ventura 13.0 on a MacBook Pro with Apple M1 Max.

* I reported the issue to the GitHub repositories for Cursive and for Crossterm.


### BearLibTerminal discussion

Is BearLibTerminal maintained or not? 

* The BearLibTerminal GitHub repository shows the last update 3 years ago.

* The BearLibTerminal GitHub installation instructions say to use `pip install bearlibterminal`. But on my system, the instructions don't work. The errors are: "ERROR: Could not find a version that satisfies the requirement bearlibterminal (from versions: none)" and "ERROR: No matching distribution found for bearlibterminal". 
  
* The BearLibTerminal example in this repo is `examples/blt_terminal.rs`. But on my system, the example doesn't work. The errors include: "note: ld: library not found for -lBearLibTerminal" and "clang: error: linker command failed with exit code 1 (use -v to see invocation)".
