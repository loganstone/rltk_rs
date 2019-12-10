use super::{RGB, Rltk, Element, Rect, Console};
use std::any::Any;

pub struct PlainText {
    pub text : String,
    pub fg : RGB,
    pub bg : RGB,
    pub dirty : bool
}

impl PlainText {
    pub fn new<S: ToString>(text : S, fg : RGB, bg : RGB) -> Box<PlainText> {
        Box::new(PlainText{            
            text : text.to_string(),
            fg, 
            bg,
            dirty : true
        })
    }

    pub fn set_text<S: ToString>(&mut self, text : S) {
        self.text = text.to_string();
        self.dirty = true;
    }    
}

impl Element for PlainText {
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