use crate::{RGB, Rltk, console::Console, Rect};
use std::collections::HashMap;
use std::any::Any;

pub enum Event {

}

pub struct UI {
    pub base_element : String,
    elements : HashMap<String, Box<dyn Element>>
}

impl UI {
    pub fn new(base_element : Box<dyn Element>) -> UI {
        let id = base_element.get_id().to_string();
        let mut ui = UI {
            base_element : id.clone(),
            elements : HashMap::new()
        };
        ui.elements.insert(id, base_element);
        ui
    }

    pub fn add(&mut self, parent: &str, element : Box<dyn Element>) {
        let id = element.get_id().to_string();
        self.elements.insert(id.clone(), element);
        if let Some(p) = self.elements.get_mut(parent) {
            p.add_child(&id);
        }
    }

    pub fn render(&self, ctx : &mut Rltk) -> Vec<Event> {
        let size = ctx.get_char_size();
        let b = Rect::new(0, 0, size.0 as i32, size.1 as i32);
        self.render_element(ctx, &self.base_element, b);
        Vec::new()
    }

    fn render_element(&self, ctx : &mut Rltk, id : &str, parent : Rect) {
        if let Some(e) = self.elements.get(id) {
            e.render(ctx, parent);

            let b = e.get_bounds();
            for child in e.get_children().iter() {
                self.render_element(ctx, child, b);
            }
        } else {
            println!("Unknown GUI element: {}", id);
        }
    }

    #[allow(clippy::borrowed_box)]
    pub fn element_by_id(&mut self, id : &str) -> Option<&mut Box<dyn Element>> {
        self.elements.get_mut(id)
    }
}

pub trait Element {
    fn render(&self, ctx : &mut Rltk, parent : Rect);
    fn get_bounds(&self) -> Rect;
    fn get_children(&self) -> &[String];
    fn get_id(&self) -> &str;
    fn add_child(&mut self, id : &str);
    fn as_any(&mut self) -> &mut dyn Any;
}

pub struct Background {
    pub glyph : u8,
    pub fg : RGB,
    pub bg : RGB,
    bounds : Rect,
    children: Vec<String>,
    id : String
}

impl Background {
    pub fn default(ctx : &mut Rltk, id : &str) -> Box<Background> {
        let size = ctx.get_char_size();
        Box::new(Background {
            glyph : crate::to_cp437('▒'),
            fg : RGB::named(crate::LIGHT_BLUE),
            bg : RGB::named(crate::DARK_BLUE),
            bounds : Rect::new(0, 0, size.0 as i32, size.1 as i32),
            children : Vec::new(),
            id : id.to_string()
        })
    }
}

impl Element for Background {
    fn render(&self, ctx : &mut Rltk, _parent : Rect) {
        for y in self.bounds.y1 .. self.bounds.y2 {
            for x in self.bounds.x1 .. self.bounds.x2 {
                ctx.set(x, y, self.fg, self.bg, self.glyph);
            }
        }
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

pub struct StatusBar {
    pub glyph : u8,
    pub fg : RGB,
    pub bg : RGB,
    bounds : Rect,
    children: Vec<String>,
    id : String
}

impl StatusBar {
    pub fn default(ctx : &mut Rltk, id : &str) -> Box<StatusBar> {
        let size = ctx.get_char_size();
        Box::new(StatusBar {
            glyph : crate::to_cp437('█'),
            fg : RGB::named(crate::LIGHT_GRAY),
            bg : RGB::named(crate::BLACK),
            bounds : Rect::new(0, size.1 as i32 - 1, size.0 as i32, 1),
            children : Vec::new(),
            id : id.to_string()
        })
    }
}

impl Element for StatusBar {
    fn render(&self, ctx : &mut Rltk, parent : Rect) {
        for y in self.bounds.y1 .. self.bounds.y2 {
            for x in self.bounds.x1 .. self.bounds.x2 {
                ctx.set(x, y, self.fg, self.bg, self.glyph);
            }
        }
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

pub struct MenuBar {
    pub glyph : u8,
    pub fg : RGB,
    pub bg : RGB,
    bounds : Rect,
    children: Vec<String>,
    id : String
}

impl MenuBar {
    pub fn default(ctx : &mut Rltk, id : &str) -> Box<MenuBar> {
        let size = ctx.get_char_size();
        Box::new(MenuBar {
            glyph : crate::to_cp437('█'),
            fg : RGB::named(crate::LIGHT_GRAY),
            bg : RGB::named(crate::BLACK),
            bounds : Rect::new(0, 0, size.0 as i32, 1),
            children : Vec::new(),
            id : id.to_string()
        })
    }
}

impl Element for MenuBar {
    fn render(&self, ctx : &mut Rltk, parent : Rect) {
        for y in self.bounds.y1 .. self.bounds.y2 {
            for x in self.bounds.x1 .. self.bounds.x2 {
                ctx.set(x, y, self.fg, self.bg, self.glyph);
            }
        }
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
    fn render(&self, ctx : &mut Rltk, parent : Rect) {
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
        let x2 = self.bounds.x2 + parent.x2;
        let y1 = self.bounds.y1 + parent.y1;
        let y2 = self.bounds.y2 + parent.y2;
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