//! Theme settings with green foreground on black background.
fn main() {
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
}
