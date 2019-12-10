use super::{RGB, Rltk, Element, Rect, Console, Theme};
use std::any::Any;

pub struct StatusBarText {
    pub text : String,
    pub fg : RGB,
    pub bg : RGB,
    pub dirty : bool
}

impl StatusBarText {
    pub fn new<S: ToString>(theme: &Theme, text : S) -> Box<StatusBarText> {
        Box::new(StatusBarText{            
            text : text.to_string(),
            fg : theme.status_bar_text.fg, 
            bg : theme.status_bar_text.bg,
            dirty : true
        })
    }

    pub fn set_text<S: ToString>(&mut self, text : S) {
        self.text = text.to_string();
        self.dirty = true;
    }    
}

impl Element for StatusBarText {
    fn render(&self, ctx : &mut Rltk, physical_bounds : Rect) {
        ctx.print_color(physical_bounds.x1, physical_bounds.y1, self.fg, self.bg, &self.text);
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
    fn flow_dirty(&self) -> bool { self.dirty }
    fn mark_flow_clean(&mut self) { self.dirty = false }
    fn desired_width(&self) -> i32 { self.text.len() as i32 + 1 }
}