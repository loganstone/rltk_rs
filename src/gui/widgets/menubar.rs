use crate::{RGB, Rltk, Rect, to_cp437, Console};
use super::{Element, Event, Theme};
use std::any::Any;

pub struct MenuBar {
    pub glyph : u8,
    pub fg : RGB,
    pub bg : RGB,
    bounds : Rect,
    children: Vec<String>,
    id : String
}

impl MenuBar {
    pub fn default(ctx : &mut Rltk, id : &str, theme : Theme) -> Box<MenuBar> {
        let size = ctx.get_char_size();
        Box::new(MenuBar {
            glyph : theme.menubar.glyph,
            fg : theme.menubar.fg,
            bg : theme.menubar.bg,
            bounds : Rect::new(0, 0, size.0 as i32, 1),
            children : Vec::new(),
            id : id.to_string()
        })
    }
}

impl Element for MenuBar {
    fn render(&self, ctx : &mut Rltk, _parent : Rect, _events : &mut Vec<Event>) {
        for y in self.bounds.y1 .. self.bounds.y2 {
            for x in self.bounds.x1 .. self.bounds.x2 {
                ctx.set(x, y, self.fg, self.bg, self.glyph);
            }
        }
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