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
pub struct ColorPair {
    pub fg : RGB,
    pub bg : RGB
}

impl ColorPair {
    pub fn new(fg : RGB, bg : RGB) -> ColorPair {
        ColorPair{ fg, bg }
    }
}

#[derive(PartialEq, Copy, Clone, Default, Debug)]
pub struct Theme {
    pub background : GlyphColor,
    pub menubar : GlyphColor,
    pub statusbar : GlyphColor,
    pub statusbutton : ColorPair,
    pub statusbutton_hover_bg : RGB,
    pub statustext : ColorPair,
}

impl Theme {
    pub fn turbo_vision() -> Theme {
        Theme {
            background : GlyphColor::new( to_cp437('▒'), RGB::named(crate::LIGHT_BLUE), RGB::named(crate::DARK_BLUE) ),
            menubar : GlyphColor::new(to_cp437('█'), RGB::named(crate::LIGHT_GRAY), RGB::named(crate::BLACK)),
            statusbar : GlyphColor::new(to_cp437('█'), RGB::named(crate::LIGHT_GRAY), RGB::named(crate::BLACK)),
            statusbutton : ColorPair::new(RGB::named(crate::NAVY), RGB::named(crate::LIGHT_GRAY)),
            statusbutton_hover_bg : RGB::named(crate::YELLOW),
            statustext : ColorPair::new(RGB::named(crate::NAVY), RGB::named(crate::LIGHT_GRAY)),
        }
    }
}