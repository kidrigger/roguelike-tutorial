use rltk::{FontCharType, RGB};

pub struct Renderable {
    pub glyph: FontCharType,
    pub foreground: RGB,
    pub background: RGB,
}

impl Renderable {
    pub fn new(glyph: char, foreground: (u8, u8, u8), background: Option<(u8, u8, u8)>) -> Self {
        Self {
            glyph: rltk::to_cp437(glyph),
            foreground: RGB::named(foreground),
            background: RGB::named(background.unwrap_or(rltk::BLACK)),
        }
    }
}
