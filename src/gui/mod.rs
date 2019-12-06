use crate::{RGB, Rltk, console::Console, Rect};

pub struct UI {
    pub base_element : Box<dyn Element>
}

impl UI {
    pub fn new(base_element : Box<dyn Element>) -> UI {
        UI {
            base_element
        }
    }

    pub fn render(&self, ctx : &mut Rltk) {
        let size = ctx.get_char_size();
        let b = Rect::new(0, 0, size.0 as i32, size.1 as i32);
        self.base_element.render(ctx, b);
        self.base_element.render_children(ctx, b);
    }
}

pub trait Element {
    fn render(&self, ctx : &mut Rltk, parent : Rect);
    fn get_bounds(&self) -> Rect;
    fn get_children(&self) -> &[Box<dyn Element>];
    fn render_children(&self, ctx : &mut Rltk, _parent : Rect) {
        let b = self.get_bounds();
        self.get_children().iter().for_each(|e| {            
            e.render(ctx, b);
            e.render_children(ctx, b);
        });
    }
    fn add_child(&mut self, element : Box<dyn Element>);
}

pub struct Background {
    pub glyph : u8,
    pub fg : RGB,
    pub bg : RGB,
    bounds : Rect,
    children : Vec<Box<dyn Element>>
}

impl Background {
    pub fn default(ctx : &mut Rltk) -> Box<Background> {
        let size = ctx.get_char_size();
        Box::new(Background {
            glyph : crate::to_cp437('▒'),
            fg : RGB::named(crate::LIGHT_BLUE),
            bg : RGB::named(crate::DARK_BLUE),
            bounds : Rect::new(0, 0, size.0 as i32, size.1 as i32),
            children : Vec::new()
        })
    }
}

impl Element for Background {
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

    fn get_children(&self) -> &[Box<dyn Element>] {
        &self.children
    }

    fn add_child(&mut self, element : Box<dyn Element>) {
        self.children.push(element);
    }
}

pub struct StatusBar {
    pub glyph : u8,
    pub fg : RGB,
    pub bg : RGB,
    bounds : Rect,
    children : Vec<Box<dyn Element>>
}

impl StatusBar {
    pub fn default(ctx : &mut Rltk) -> Box<Background> {
        let size = ctx.get_char_size();
        Box::new(Background {
            glyph : crate::to_cp437('█'),
            fg : RGB::named(crate::LIGHT_GRAY),
            bg : RGB::named(crate::BLACK),
            bounds : Rect::new(0, size.1 as i32 - 1, size.0 as i32, 1),
            children : Vec::new()
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

    fn get_children(&self) -> &[Box<dyn Element>] {
        &self.children
    }

    fn add_child(&mut self, element : Box<dyn Element>) {
        self.children.push(element);
    }
}

pub struct MenuBar {
    pub glyph : u8,
    pub fg : RGB,
    pub bg : RGB,
    bounds : Rect,
    children : Vec<Box<dyn Element>>
}

impl MenuBar {
    pub fn default(ctx : &mut Rltk) -> Box<MenuBar> {
        let size = ctx.get_char_size();
        Box::new(MenuBar {
            glyph : crate::to_cp437('█'),
            fg : RGB::named(crate::LIGHT_GRAY),
            bg : RGB::named(crate::BLACK),
            bounds : Rect::new(0, 0, size.0 as i32, 1),
            children : Vec::new()
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

    fn get_children(&self) -> &[Box<dyn Element>] {
        &self.children
    }

    fn add_child(&mut self, element : Box<dyn Element>) {
        self.children.push(element);
    }
}

pub struct PlainText {
    pub text : String,
    pub fg : RGB,
    pub bg : RGB,
    bounds : Rect,
    children : Vec<Box<dyn Element>>
}

impl PlainText {
    pub fn default(_ctx : &mut Rltk, text : &str, x : i32, y : i32, fg : RGB, bg : RGB) -> Box<PlainText> {
        Box::new(PlainText {
            text : text.to_string(),
            fg,
            bg,
            bounds : Rect::new(x, y, text.len() as i32, 1),
            children : Vec::new()
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

    fn get_children(&self) -> &[Box<dyn Element>] {
        &self.children
    }

    fn add_child(&mut self, element : Box<dyn Element>) {
        self.children.push(element);
    }
}

pub struct Window {
    pub fg : RGB,
    pub bg : RGB,
    bounds : Rect,
    title : String,
    children : Vec<Box<dyn Element>>
}

impl Window {
    pub fn new(_ctx : &mut Rltk, x : i32, y : i32, w: i32, h: i32, title : &str) -> Box<Window> {
        Box::new(Window {
            fg : RGB::named(crate::NAVY),
            bg : RGB::named(crate::LIGHT_GRAY),
            bounds : Rect::new(x, y, w, h),
            children : Vec::new(),
            title : title.to_string()
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

    fn get_children(&self) -> &[Box<dyn Element>] {
        &self.children
    }

    fn add_child(&mut self, element : Box<dyn Element>) {
        self.children.push(element);
    }
}