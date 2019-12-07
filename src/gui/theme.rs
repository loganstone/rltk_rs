use crate::{RGB, to_cp437};

#[derive(PartialEq, Copy, Clone, Default, Debug)]
pub struct GlyphColor {
    pub glyph : u8,
    pub fg : RGB,
    pub bg : RGB
}

impl GlyphColor {
    pub fn new(glyph : u8, fg : RGB, bg : RGB) -> GlyphColor {
        GlyphColor { glyph, fg, bg }
    }
}

#[derive(PartialEq, Copy, Clone, Default, Debug)]
pub struct Theme {
    pub background : GlyphColor,
    pub menubar : GlyphColor,
    pub statusbar : GlyphColor
}

impl Theme {
    pub fn turbo_vision() -> Theme {
        Theme {
            background : GlyphColor::new( to_cp437('▒'), RGB::named(crate::LIGHT_BLUE), RGB::named(crate::DARK_BLUE) ),
            menubar : GlyphColor::new(to_cp437('█'), RGB::named(crate::LIGHT_GRAY), RGB::named(crate::BLACK)),
            statusbar : GlyphColor::new(to_cp437('█'), RGB::named(crate::LIGHT_GRAY), RGB::named(crate::BLACK))
        }
    }
}