//! Theme palette customized with terminal default colors.
fn main() {
    let mut c = cursive::default();
    let mut theme = c.current_theme().clone();
    theme.shadow = false;
    theme.borders = cursive::theme::BorderStyle::None;
    theme.palette[cursive::theme::PaletteColor::Background] = cursive::theme::Color::TerminalDefault;
    theme.palette[cursive::theme::PaletteColor::View] = cursive::theme::Color::TerminalDefault;
    c.set_theme(theme);
    c.run();
}