use std::collections::HashMap;
use crate::{Rltk, Rect, RGB, to_cp437, Console};
mod theme;
pub use theme::*;
mod widgets;
pub use widgets::{SolidBackground, StatusBar, StatusBarText};
use std::any::Any;

pub struct TextUI {
    element_store : Vec<ElementStorage>,
    element_keys : HashMap<String, usize>,
    theme : Theme,
    base_element : usize
}

impl TextUI {
    pub fn new(theme : Theme) -> TextUI {
        TextUI{
            element_keys : HashMap::new(),
            element_store : Vec::new(),
            theme,
            base_element : 0
        }
    }

    pub fn get_id<S: ToString>(&self, id : S) -> Option<&usize> {
        self.element_keys.get(&id.to_string())
    }

    pub fn add_explicit<S : ToString>(&mut self, key: S, element : Box<dyn Element>) -> usize {
        let parent_id_option = element.get_info().parent;        

        self.element_store.push(ElementStorage::new(element));
        let vec_key = self.element_store.len()-1;
        self.element_keys.insert(key.to_string(), vec_key);

        if let Some(parent_id) = parent_id_option {
            self.element_store[parent_id].add_child(vec_key);
        }

        vec_key
    }

    pub fn add<S : ToString>(&mut self, ctx : &mut Rltk, widget : WidgetType, key : S, parent : S) -> &mut Self {
        let parent_id = self.element_keys.get(&parent.to_string());
        let parent_v = match parent_id {
            None => None,
            Some(pid) => Some(*pid)
        };
        self.add_return_id(ctx, widget, key, parent_v);
        self
    }

    pub fn add_return_id<S : ToString>(&mut self, ctx : &mut Rltk, widget : WidgetType, key : S, parent : Option<usize>) -> usize {
        match widget {
            WidgetType::ScreenBackground => {
                self.add_explicit(key, SolidBackground::screen_background(ctx, &self.theme, parent))
            }
            WidgetType::StatusBar => {
                self.add_explicit(key, StatusBar::new(ctx, &self, parent))
            }
            WidgetType::StatusText{text} => {
                self.add_explicit(key, StatusBarText::new(&self, parent, text))
            }
        }
    }

    pub fn set_base<S : ToString>(&mut self, id : S) -> &mut Self {
        let key = self.element_keys.get(&id.to_string()).unwrap();
        self.base_element = *key;
        self
    }

    pub fn render(&self, ctx : &mut Rltk) {
        self.element_store[self.base_element].render(ctx);
        self.render_element(ctx, self.base_element);
    }

    fn render_element(&self, ctx : &mut Rltk, id : usize) {
        for child_id in self.element_store[id].get_children().iter() {
            let element = &self.element_store[*child_id];
            if element.render(ctx) {
                self.render_element(ctx, *child_id);
            }
        }
    }

    fn get_bounds(&self, id : usize) -> Rect {
        self.element_store[id].element.get_info().bounds
    }

    fn get_child_widths(&self, id : usize) -> i32 {
        self.element_store[id].element.get_child_widths(&self)
    }

    #[allow(clippy::borrowed_box)]
    pub fn element_by_id(&mut self, id : usize) -> &mut Box<dyn Element> {
        &mut self.element_store[id].element
    }
}

pub enum WidgetType {
    ScreenBackground,
    StatusBar,
    StatusText{ text : String }
}

struct ElementStorage {
    deleted : bool,
    visible : bool,
    element : Box<dyn Element>
}

impl ElementStorage {
    pub fn new(element : Box<dyn Element>) -> ElementStorage {
        ElementStorage {
            deleted : false,
            visible : true,
            element
        }
    }

    pub fn render(&self, ctx : &mut Rltk) -> bool {
        if !self.deleted && self.visible {
            self.element.render(ctx);
            return true;
        }
        false
    }

    pub fn get_children(&self) -> &[usize] {
        self.element.get_children()
    }

    pub fn add_child(&mut self, id : usize) {
        self.element.add_child(id);
    }
}

pub enum Placement {
    Absolute,
    Relative
}

pub struct ElementInfo {
    pub placement : Placement,
    pub bounds : Rect,
    pub parent : Option<usize>,
    pub children : Vec<usize>
}

pub trait Element {
    fn get_info(&self) -> &ElementInfo;
    fn get_info_mut(&mut self) -> &mut ElementInfo;
    fn render(&self, _ctx : &mut Rltk) {}
    fn get_children(&self) -> &[usize] { &self.get_info().children }
    fn add_child(&mut self, id : usize);
    fn get_child_widths(&self, _ui : &TextUI) -> i32 { 0 }
    fn as_any(&mut self) -> &mut dyn Any;
}


