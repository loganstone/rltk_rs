use crate::{RGB, Rltk, Rect, Console};
use super::{Element, Event, EventType};
use std::any::Any;

pub struct MouseOverText {
    pub text : String,
    pub hotkey_text : String,
    pub fg : RGB,
    pub bg : RGB,
    bounds : Rect,
    children: Vec<String>,
    id : String
}

impl MouseOverText {
    pub fn default(_ctx : &mut Rltk, id : &str, hotkey: &str, text : &str, x : i32, y : i32, fg : RGB, bg : RGB) -> Box<MouseOverText> {
        let w = if hotkey.is_empty() {
            text.len() as i32
        } else {
            text.len() as i32 + hotkey.len() as i32 + 1
        };
        Box::new(MouseOverText {
            text : text.to_string(),
            hotkey_text : hotkey.to_string(),
            fg,
            bg,
            bounds : Rect::new(x, y, w, 1),
            children : Vec::new(),
            id : id.to_string()
        })
    }
}

impl Element for MouseOverText {
    fn render(&self, ctx : &mut Rltk, parent : Rect, events : &mut Vec<Event>) {
        let mut bg = self.bg;
        let mut x = self.bounds.x1 + parent.x1;
        let actual_bounds = self.bounds + parent;
        if actual_bounds.xy_in_rect(ctx.mouse_pos()) {
            bg = RGB::named(crate::YELLOW);
            events.push(Event::new(self.get_id(), EventType::MouseOver));
            if ctx.left_click {
                events.push(Event::new(self.get_id(), EventType::Clicked));
            }
        }
        if !self.hotkey_text.is_empty() {
            ctx.print_color(x, self.bounds.y1 + parent.y1, RGB::named(crate::RED), bg, &self.hotkey_text);
            x += self.hotkey_text.len() as i32 + 1;
        }
        ctx.print_color(x, self.bounds.y1 + parent.y1, self.fg, bg, &self.text);
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
