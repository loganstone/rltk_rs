use super::{Rltk, Rect, Console, Element, ElementStorage, Placement};
use std::collections::HashMap;

#[derive(Default)]
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

    pub fn add_element(&mut self, 
        key : String,
        element : Box<dyn Element>, 
        parent_id_option : Option<usize>, 
        physical_bounds : Rect, 
        placement : Placement) -> usize 
    {
        self.element_store.push(ElementStorage::new(element, physical_bounds, placement, parent_id_option));
        let vec_key = self.element_store.len()-1;
        self.element_keys.insert(key, vec_key);

        if let Some(parent_id) = parent_id_option {
            self.element_store[parent_id].children.push(vec_key);
        }

        vec_key
    }

    pub fn render(&mut self, ctx : &mut Rltk, id : usize) {
        let screen_size = ctx.get_char_size();
        let physical_bounds = Rect::new(0, 0, screen_size.0 as i32, screen_size.1 as i32);
        self.element_store[id].render(ctx, physical_bounds);
        render_element(self, ctx, id, physical_bounds);
    }

    #[allow(clippy::borrowed_box)]
    pub fn element_by_id(&mut self, id : usize) -> &mut Box<dyn Element> {
        &mut self.element_store[id].element
    }

    pub fn get_physical_bounds(&self, id: usize) -> Rect {
        self.element_store[id].physical_bounds
    }

    pub fn calc_child_width(&self, id : usize) -> i32 {
        let mut width = 0;
        for child in self.element_store[id].children.iter() {
            width += self.element_store[*child].physical_bounds.width();
        }
        width
    }

    fn get_children_of_element(&self, id : usize) -> Vec<usize> {
        self.element_store[id].children.clone()
    }

    fn by_id(&mut self, id : usize) -> &mut ElementStorage {
        &mut self.element_store[id]
    }
}

fn render_element(element_store : &mut ElementStore, ctx : &mut Rltk, id : usize, parent_bounds : Rect) {
    let child_elements = element_store.get_children_of_element(id);
    for child_id in child_elements.iter() {
        let element = element_store.by_id(*child_id);
        if let Some(bounds) = element.render(ctx, parent_bounds) {
            render_element(element_store, ctx, *child_id, bounds);
        }
    }
}

