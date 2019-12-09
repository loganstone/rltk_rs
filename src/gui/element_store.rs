use super::{Rltk, Rect, TextUI};
use std::any::Any;
use std::collections::HashMap;

pub struct ElementStore {
    element_store : Vec<ElementStorage>,
    element_keys : HashMap<String, usize>,
}

impl ElementStore {
    pub fn new() -> ElementStore {
        ElementStore{
            element_store : Vec::new(),
            element_keys : HashMap::new()
        }
    }

    pub fn get_id<S: ToString>(&self, id : S) -> Option<&usize> {
        self.element_keys.get(&id.to_string())
    }

    pub fn add_element(&mut self, key : String, element : Box<dyn Element>, parent_id_option : Option<usize>) -> usize {
        self.element_store.push(ElementStorage::new(element));
        let vec_key = self.element_store.len()-1;
        self.element_keys.insert(key, vec_key);

        if let Some(parent_id) = parent_id_option {
            self.element_store[parent_id].add_child(vec_key);
        }

        vec_key
    }

    pub fn render(&self, ctx : &mut Rltk, id : usize) {
        self.element_store[id].render(ctx);
        self.render_element(ctx, id);
    }

    fn render_element(&self, ctx : &mut Rltk, id : usize) {
        for child_id in self.element_store[id].get_children().iter() {
            let element = &self.element_store[*child_id];
            if element.render(ctx) {
                self.render_element(ctx, *child_id);
            }
        }
    }

    #[allow(clippy::borrowed_box)]
    pub fn element_by_id(&mut self, id : usize) -> &mut Box<dyn Element> {
        &mut self.element_store[id].element
    }

    pub fn get_bounds(&self, id : usize) -> Rect {
        self.element_store[id].element.get_info().bounds
    }

    pub fn get_child_widths(&self, id : usize) -> i32 {
        self.element_store[id].element.get_child_widths(&self)
    }
}

pub struct ElementStorage {
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
    fn get_child_widths(&self, _ui : &ElementStore) -> i32 { 0 }
    fn as_any(&mut self) -> &mut dyn Any;
    fn supports_focus(&self) -> bool { false }
}
