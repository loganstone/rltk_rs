use super::{RGB, Rltk, Element, Theme, Rect, Console};
use std::any::Any;

pub struct SolidBackground {
    pub glyph : u8,
    pub fg : RGB,
    pub bg : RGB
}

impl SolidBackground {
    pub fn screen_background(theme : &Theme) -> Box<SolidBackground> {
        Box::new(SolidBackground{
            glyph : theme.solid_background.glyph,
            fg : theme.solid_background.fg, 
            bg : theme.solid_background.bg
        })
    }
}

impl Element for SolidBackground {
    fn render(&self, ctx : &mut Rltk, physical_bounds : Rect) {
        physical_bounds.for_each(|(x,y)| {
            ctx.set(x, y, self.fg, self.bg, self.glyph);
        });
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}