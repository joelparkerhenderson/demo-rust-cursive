//! Theme settings with shadow, BorderStyle borders, and Palette colors.
fn main() {
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
}
