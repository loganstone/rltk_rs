use super::{RGB, Rltk, Element, Rect, Console, Theme, ReflowType};
use std::any::Any;

pub struct StatusBar {
    pub glyph : u8,
    pub fg : RGB,
    pub bg : RGB
}

impl StatusBar {
    pub fn new(theme: &Theme) -> Box<StatusBar> {
        Box::new(StatusBar{
            glyph : theme.status_bar_background.glyph,
            fg : theme.status_bar_background.fg, 
            bg : theme.status_bar_background.bg
        })
    }
}

impl Element for StatusBar {
    fn render(&self, ctx : &mut Rltk, physical_bounds : Rect) {
        physical_bounds.for_each(|(x,y)| {
            ctx.set(x, y, self.fg, self.bg, self.glyph);
        });
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn is_container(&self) -> ReflowType { ReflowType::Horizontal }
}