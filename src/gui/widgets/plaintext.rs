use crate::{RGB, Rltk, Rect, Console};
use super::{Element, Event};
use std::any::Any;

pub struct PlainText {
    pub text : String,
    pub fg : RGB,
    pub bg : RGB,
    bounds : Rect,
    children: Vec<String>,
    id : String
}

impl PlainText {
    pub fn default(_ctx : &mut Rltk, id : &str, text : &str, x : i32, y : i32, fg : RGB, bg : RGB) -> Box<PlainText> {
        Box::new(PlainText {
            text : text.to_string(),
            fg,
            bg,
            bounds : Rect::new(x, y, text.len() as i32, 1),
            children : Vec::new(),
            id : id.to_string()
        })
    }
}

impl Element for PlainText {
    fn render(&self, ctx : &mut Rltk, parent : Rect, events : &mut Vec<Event>) {
        ctx.print_color(self.bounds.x1 + parent.x1, self.bounds.y1 + parent.y1, self.fg, self.bg, &self.text);
    }

    fn get_bounds(&self) -> Rect {
        self.bounds
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
