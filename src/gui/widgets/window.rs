use crate::{RGB, Rltk, Rect, Console};
use super::Element;
use std::any::Any;

pub struct Window {
    pub fg : RGB,
    pub bg : RGB,
    bounds : Rect,
    title : String,
    children: Vec<String>,
    id : String
}

impl Window {
    pub fn new(_ctx : &mut Rltk, id : &str, x : i32, y : i32, w: i32, h: i32, title : &str) -> Box<Window> {
        Box::new(Window {
            fg : RGB::named(crate::NAVY),
            bg : RGB::named(crate::LIGHT_GRAY),
            bounds : Rect::new(x, y, w, h),
            children : Vec::new(),
            title : title.to_string(),
            id : id.to_string()
        })
    }
}

impl Element for Window {
    fn render(&self, ctx : &mut Rltk, parent : Rect) {
        let x1 = self.bounds.x1 + parent.x1;
        let y1 = self.bounds.y1 + parent.y1;
        ctx.draw_box_double(x1, y1, self.bounds.x2 - self.bounds.x1, self.bounds.y2 - self.bounds.y1, self.fg, self.bg);
        ctx.set(x1 + 2, y1, RGB::named(crate::YELLOW), RGB::named(crate::NAVY), crate::to_cp437('['));
        ctx.set(x1 + 3 + self.title.len() as i32, y1, RGB::named(crate::YELLOW), RGB::named(crate::NAVY), crate::to_cp437(']'));
        ctx.print_color(x1 + 3, y1, RGB::named(crate::YELLOW), RGB::named(crate::NAVY), &self.title);
    }

    fn get_bounds(&self) -> Rect {
        Rect::new(
            self.bounds.x1+1, 
            self.bounds.y1+1, 
            (self.bounds.x2-self.bounds.x1)-2, 
            (self.bounds.y2-self.bounds.y1)-2
        )
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