use super::{ElementInfo, RGB, Rltk, Element, Placement, Rect, Console, ElementStore, Theme};
use std::any::Any;

pub struct StatusBar {
    pub info : ElementInfo,
    pub glyph : u8,
    pub fg : RGB,
    pub bg : RGB
}

impl StatusBar {
    pub fn new(ctx : &mut Rltk, tui : &ElementStore, theme: &Theme, parent : Option<usize>) -> Box<StatusBar> {
        if let Some(parent_id) = parent {
            let parent_bounds = tui.get_bounds(parent_id);
            Box::new(StatusBar{
                info : ElementInfo{
                    placement: Placement::Absolute,
                    bounds : Rect::new(parent_bounds.x1, parent_bounds.y2 - 1, parent_bounds.width(), 1),
                    parent,
                    children : Vec::new()
                },
                glyph : theme.status_bar_background.glyph,
                fg : theme.status_bar_background.fg, 
                bg : theme.status_bar_background.bg
            })
        } else {
            let screen_bounds = ctx.get_char_size();
            Box::new(StatusBar{
                info : ElementInfo{
                    placement: Placement::Relative,
                    bounds : Rect::new(0, screen_bounds.1 as i32 - 1, screen_bounds.0 as i32, 1),
                    parent,
                    children : Vec::new()
                },
                glyph : theme.status_bar_background.glyph,
                fg : theme.status_bar_background.fg, 
                bg : theme.status_bar_background.bg
            })
        }
    }
}

impl Element for StatusBar {
    fn get_info(&self) -> &ElementInfo { &self.info }
    fn get_info_mut(&mut self) -> &mut ElementInfo { &mut self.info }
    fn render(&self, ctx : &mut Rltk) {
        self.info.bounds.for_each(|(x,y)| {
            ctx.set(x, y, self.fg, self.bg, self.glyph);
        });
    }
    fn add_child(&mut self, id : usize) { self.info.children.push(id); }
    fn get_child_widths(&self, ui : &ElementStore) -> i32 {
        let mut width : i32 = 0;
        for child in self.info.children.iter() {
            width += ui.get_bounds(*child).width();
        }
        width
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}