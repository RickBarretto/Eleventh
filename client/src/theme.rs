use cursive::theme::{BaseColor, BorderStyle, Color, Palette, PaletteColor, Theme};

pub fn theme() -> Theme {
    let mut palette = Palette::default();

    const WHITE: Color = Color::Dark(BaseColor::White);
    const GRAY_150: Color = Color::Rgb(150, 150, 150);
    const GRAY_180: Color = Color::Rgb(180, 180, 180);
    const GLOBAL_BACKGROUND: Color = Color::Rgb(20, 20, 28);
    const NEON_BLUE: Color = Color::Rgb(0, 150, 255);
    const METALLIC_GOLD: Color = Color::Rgb(212, 175, 55);
    const DARK_SHADOW: Color = Color::Rgb(10, 10, 15);

    // Global
    palette[PaletteColor::View] = GLOBAL_BACKGROUND;
    palette[PaletteColor::Background] = GLOBAL_BACKGROUND;

    // Primary text
    palette[PaletteColor::Primary] = WHITE;
    palette[PaletteColor::TitlePrimary] = WHITE;

    // Secondary text
    palette[PaletteColor::Secondary] = GRAY_150;
    palette[PaletteColor::TitleSecondary] = GRAY_180;

    palette[PaletteColor::Highlight] = NEON_BLUE;
    palette[PaletteColor::HighlightText] = WHITE;
    palette[PaletteColor::HighlightInactive] = METALLIC_GOLD;

    palette[PaletteColor::Shadow] = DARK_SHADOW;

    Theme {
        shadow: true,
        borders: BorderStyle::Outset,
        palette,
    }
}
