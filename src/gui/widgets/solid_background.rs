use super::{ElementInfo, RGB, Rltk, Element, Theme, Placement, Rect, Console};
use std::any::Any;

pub struct SolidBackground {
    pub info : ElementInfo,
    pub glyph : u8,
    pub fg : RGB,
    pub bg : RGB
}

impl SolidBackground {
    pub fn screen_background(ctx : &mut Rltk, theme : &Theme, parent : Option<usize>) -> Box<SolidBackground> {
        let screen_bounds = ctx.get_char_size();
        Box::new(SolidBackground{
            info : ElementInfo{
                placement: Placement::Absolute,
                bounds : Rect::new(0, 0, screen_bounds.0 as i32, screen_bounds.1 as i32),
                parent,
                children : Vec::new()
            },
            glyph : theme.solid_background.glyph,
            fg : theme.solid_background.fg, 
            bg : theme.solid_background.bg
        })
    }
}

impl Element for SolidBackground {
    fn get_info(&self) -> &ElementInfo { &self.info }
    fn get_info_mut(&mut self) -> &mut ElementInfo { &mut self.info }
    fn render(&self, ctx : &mut Rltk) {
        self.info.bounds.for_each(|(x,y)| {
            ctx.set(x, y, self.fg, self.bg, self.glyph);
        });
    }
    fn add_child(&mut self, id : usize) { self.info.children.push(id); }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}