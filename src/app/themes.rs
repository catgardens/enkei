use cursive::theme::Color::TerminalDefault;
use cursive::theme::PaletteColor::{
    Background, Highlight, HighlightInactive, HighlightText, Primary, Secondary, Shadow,
    Tertiary, TitlePrimary, TitleSecondary, View,
};
use cursive::theme::{BorderStyle, Palette, Theme};

pub fn init_pallete() -> Palette {
    let mut p = Palette::default();
    p[Background] = TerminalDefault;
    p[Shadow] = TerminalDefault;
    p[View] = TerminalDefault;
    p[Primary] = TerminalDefault;
    p[Secondary] = TerminalDefault;
    p[Tertiary] = TerminalDefault;
    p[TitlePrimary] = TerminalDefault;
    p[TitleSecondary] = TerminalDefault;
    p[Highlight] = TerminalDefault;
    p[HighlightInactive] = TerminalDefault;
    p[HighlightText] = TerminalDefault;
    p
}

pub fn init_theme() -> Theme {
    let mut t = Theme::retro();
    t.borders = BorderStyle::None;
    t.palette = init_pallete();
    t
}
