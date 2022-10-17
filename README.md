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
let mut c = cursive::default();
c.run();
```


## Add global callback

Add global callback, so the user can press the escape key to quit.

```rust
let mut c = cursive::default();
c.add_global_callback(cursive::event::Key::Esc, |c| c.quit());
c.run();
```


## Button

Show a button that the user can click to quit.

```rust
let mut c = cursive::default();
let button = cursive::views::Button::new("Quit", |c| c.quit());
c.add_layer(button);
c.run();
```


## TextView

Show a TextView that shows the text "Hello World".

```rust
let mut c = cursive::default();
let view = cursive::views::TextView::new("Hello World");
c.add_layer(view);
c.run();
```


## Dialog

Show a Dialog info box with a message and a default "Ok" button.

```rust
let mut c = cursive::default();
let view = cursive::views::Dialog::info("Hello World");
c.add_layer(view);
c.run();
```


## SelectView

Show a SelectView with items to pick, then quit.

```rust
let mut c = cursive::default();
let mut view = cursive::views::SelectView::new();
view.add_item("Demo 1", 1);
view.add_item("Demo 2", 2);
view.add_item("Demo 3", 3);
view.set_on_submit(|c, value| {
    let dialog = cursive::views::Dialog::info(value.to_string());
    c.add_layer(dialog);
});
c.add_layer(view);
c.run();
```


## Set window title

Set window title, which works on some systems, yet not on others.

```rust
let mut c = cursive::default();
siv.set_window_title("Demo Title");
c.run();
```


## Menubar

Show a menu bar at the top of the screen; press the escape key to use the menu.

```rust
let mut c = cursive::default();
c.menubar()
.add_leaf("Quit", |c| c.quit());
c.set_autohide_menu(false);
c.add_global_callback(cursive::event::Key::Esc, |c| c.select_menubar());
c.run();
```


## LinearLayout vertical

Use a linear layout manager to arrange items vertically.

```rust
let mut c = cursive::default();
let linear_layout = cursive::views::LinearLayout::vertical()
.child(cursive::views::TextView::new("Demo 1"))
.child(cursive::views::TextView::new("Demo 2"))
.child(cursive::views::TextView::new("Demo 3"));
c.add_layer(linear_layout);
c.run();
```


## LinearLayout horizontal

Use a linear layout manager to arrange items horizontally.

```rust
let mut c = cursive::default();
let linear_layout = cursive::views::LinearLayout::horizontal()
.child(cursive::views::TextView::new("Demo 1"))
.child(cursive::views::TextView::new("Demo 2"))
.child(cursive::views::TextView::new("Demo 3"));
c.add_layer(linear_layout);
c.run();
```


## Checkbox

Show a Checkbox; press space or enter to toggle.

```rust
let mut c = cursive::default();
let checkbox = cursive::views::Checkbox::new().on_change(|c, value| {
    let dialog = cursive::views::Dialog::info(value.to_string());
    c.add_layer(dialog);
});
c.add_layer(checkbox);
```


## EditView with Resizable

Show an EditView with a Resizable.

```rust
let mut c = cursive::default();
let edit_view = cursive::views::EditView::new()
.max_content_width(20)
.on_submit(|c, value| {
    let dialog = cursive::views::Dialog::info(value.to_string());
    c.add_layer(dialog);
});
use cursive::view::Resizable;
c.add_layer(edit_view.fixed_width(20));
c.run();
```


## RadioGroup & RadioButton

Show a RadioGroup and RadioButton items using a LinearLayout.

```rust
let mut c = cursive::default();
let mut radio_group: cursive::views::RadioGroup<String> = cursive::views::RadioGroup::new()
.on_change(|c, value: &String| {
    let dialog = cursive::views::Dialog::info(value);
    c.add_layer(dialog);
});
let linear_layout = cursive::views::LinearLayout::vertical()
.child(radio_group.button_str("Demo 1"))
.child(radio_group.button_str("Demo 2"))
.child(radio_group.button_str("Demo 3"));
c.add_layer(linear_layout);
c.run();
```


## Theme with BorderStyle and Palette

Theme settings with shadow, BorderStyle borders, and Palette colors.

```rust
use cursive::theme::{BaseColor::*, Color::*, PaletteColor::*};
let mut c = cursive::default();
let mut palette = cursive::theme::Palette::default();
palette[Background] =Dark(Blue);
palette[Shadow] = Dark(Black); 
palette[View] = Dark(White);
palette[Primary] = Dark(Black);
palette[Secondary] = Dark(Blue);
palette[Tertiary] = Light(White);
palette[TitlePrimary] = Dark(Red);
palette[TitleSecondary] = Dark(Yellow);
palette[Highlight] = Dark(Red);
palette[HighlightInactive] = Dark(Blue);
palette[HighlightText] = Dark(White);    
let theme = cursive::theme::Theme{
    shadow: true,
    borders: cursive::theme::BorderStyle::Simple,
    palette: palette,
};
c.set_theme(theme);
c.add_layer(cursive::views::TextView::new("Hello World"));
c.run();
```


## Theme palette with green on black

Theme settings with green foreground colors on black background colors.

```rust
use cursive::theme::{BaseColor::*, Color::*, PaletteColor::*};
let mut c = cursive::default();
let mut palette = cursive::theme::Palette::default();
palette[Background] =Dark(Black);
palette[Shadow] = Dark(Black); 
palette[View] = Dark(Black);
palette[Primary] = Light(Green);
palette[Secondary] = Light(Green);
palette[Tertiary] = Light(Green);
palette[TitlePrimary] = Light(Green);
palette[TitleSecondary] = Light(Green);
palette[Highlight] = Light(Green);
palette[HighlightInactive] = Light(Green);
palette[HighlightText] = Light(Green);    
let theme = cursive::theme::Theme{
    shadow: true,
    borders: cursive::theme::BorderStyle::None,
    palette: palette,
};
c.set_theme(theme);
c.add_layer(cursive::views::TextView::new("Hello World"));
c.run();
```


## Theme palette with terminal default

Theme palette customized with terminal default colors.

```rust
let mut c = cursive::default();
let mut theme = c.current_theme().clone();
theme.shadow = false;
theme.borders = cursive::theme::BorderStyle::None;
theme.palette[cursive::theme::PaletteColor::Background] = cursive::theme::Color::TerminalDefault;
theme.palette[cursive::theme::PaletteColor::View] = cursive::theme::Color::TerminalDefault;
c.set_theme(theme);
c.run();
```


## Add fullscreen layer

Add a fullscreen layer i.e. a view with zero margin and zero padding.

```rust
let mut c = cursive::default();
let view = cursive::views::TextView::new("Hello World");
c.add_fullscreen_layer(view.full_screen());
c.run();
```
