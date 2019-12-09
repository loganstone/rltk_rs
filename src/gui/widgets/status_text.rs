use crate::{RGB, Rltk, Rect, Console};
use super::{Element, Event, Theme};
use std::any::Any;

pub struct StatusText {
    pub text : String,
    pub fg : RGB,
    pub bg : RGB,
    bounds : Rect,
    children: Vec<String>,
    id : String
}

impl StatusText {
    pub fn default(id : &str, text : &str, theme : Theme) -> Box<StatusText> {
        Box::new(StatusText {
            text : text.to_string(),
            fg : theme.statustext.fg,
            bg : theme.statustext.bg,
            bounds : Rect::new(0, 0, text.len() as i32, 1),
            children : Vec::new(),
            id : id.to_string()
        })
    }
}

impl Element for StatusText {
    fn render(&self, ctx : &mut Rltk, parent : Rect, _events : &mut Vec<Event>) {
        ctx.print_color(self.bounds.x1 + parent.x1, self.bounds.y1 + parent.y1, self.fg, self.bg, &self.text);
    }

    fn get_bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, new_bounds : Rect) {
        self.bounds = new_bounds;
    }

    fn get_children(&self) -> &[String] {
        &self.children
    }

    fn get_id(&self) -> &str {
        &self.id
    }

    fn add_child(&mut self, id : &str) {
        self.children.push(id.to_string());
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
