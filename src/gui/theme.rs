use super::{RGB, to_cp437};

pub struct GlyphColor {
    pub glyph : u8,
    pub fg : RGB,
    pub bg : RGB
}

pub struct ColorPair {
    pub fg : RGB,
    pub bg : RGB
}

pub struct Theme {
    pub solid_background : GlyphColor,
    pub status_bar_background : GlyphColor,
    pub status_bar_text : ColorPair,
    pub window_background : ColorPair,
    pub window_border : ColorPair,
    pub window_title : ColorPair
}

impl Theme {
    pub fn turbo_vision() -> Theme {
        Theme{
            solid_background : GlyphColor{ glyph : to_cp437('▒'), fg : RGB::named(crate::NAVY), bg : RGB::named(crate::GRAY) },
            status_bar_background : GlyphColor{ glyph : to_cp437('█'), fg : RGB::named(crate::LIGHT_GRAY), bg : RGB::named(crate::LIGHT_GRAY) },
            status_bar_text : ColorPair{ fg : RGB::named(crate::NAVY), bg : RGB::named(crate::LIGHT_GRAY) },
            window_background : ColorPair { fg : RGB::named(crate::BLACK), bg : RGB::named(crate::DARK_GRAY) },
            window_border : ColorPair { fg : RGB::named(crate::GRAY), bg : RGB::named(crate::BLACK) },
            window_title : ColorPair { fg : RGB::named(crate::WHITE), bg : RGB::named(crate::BLACK) },
        }
    }
}