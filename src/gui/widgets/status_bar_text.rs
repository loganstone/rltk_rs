use super::{RGB, Rltk, Element, Rect, Console, Theme};
use std::any::Any;

pub struct StatusBarText {
    pub text : String,
    pub fg : RGB,
    pub bg : RGB
}

impl StatusBarText {
    pub fn new<S: ToString>(theme: &Theme, text : S) -> Box<StatusBarText> {
        Box::new(StatusBarText{            
            text : text.to_string(),
            fg : theme.status_bar_text.fg, 
            bg : theme.status_bar_text.bg
        })
    }

    pub fn set_text<S: ToString>(&mut self, text : S) {
        self.text = text.to_string();
    }
}

impl Element for StatusBarText {
    fn render(&self, ctx : &mut Rltk, physical_bounds : Rect) {
        ctx.print_color(physical_bounds.x1, physical_bounds.y1, self.fg, self.bg, &self.text);
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}