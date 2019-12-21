use super::{Rltk, Rect, Console, Element, ElementStorage, Placement, ReflowType};
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

    fn get_children_of_element(&self, id : usize) -> Vec<usize> {
        self.element_store[id].children.clone()
    }

    fn by_id(&mut self, id : usize) -> &mut ElementStorage {
        &mut self.element_store[id]
    }

    pub fn update(&mut self, id : usize, ctx: &mut Rltk) {
        self.element_store[id].update(ctx);
        update_element(self, id, ctx);
    }

    pub fn set_update_function(&mut self, id : usize, updater : fn(&mut Rltk, &mut Box<dyn Element>)) {
        self.element_store[id].on_update = Some(updater);
    }
}

fn update_element(element_store : &mut ElementStore, id : usize, ctx : &mut Rltk) {
    let child_elements = element_store.get_children_of_element(id);
    for child_id in child_elements.iter() {
        let element = element_store.by_id(*child_id);
        element.update(ctx);
        update_element(element_store, *child_id, ctx);
    }
}

fn render_element(element_store : &mut ElementStore, ctx : &mut Rltk, id : usize, parent_bounds : Rect) {
    let child_elements = element_store.get_children_of_element(id);

    let reflow_type = element_store.element_by_id(id).is_container();
    match reflow_type {
        ReflowType::None => {}
        _ => {
            let mut dirty = false;
            child_elements.iter().for_each(|e| {
                let elem = element_store.element_by_id(*e);
                if elem.flow_dirty() { 
                    dirty = true;
                    elem.mark_flow_clean();
                }
            });

            if dirty {
                match reflow_type {
                    ReflowType::Horizontal => horizontal_reflow(element_store, &child_elements),
                    ReflowType::Vertical => vertical_reflow(element_store, &child_elements),
                    _ => {}
                }
            }
        }
    }

    for child_id in child_elements.iter() {
        let element = element_store.by_id(*child_id);
        if let Some(bounds) = element.render(ctx, parent_bounds) {
            render_element(element_store, ctx, *child_id, bounds);
        }
    }
}

fn horizontal_reflow(element_store : &mut ElementStore, children : &[usize]) {
    let mut x = 1;
    for child_id in children.iter() {
        let w = element_store.element_by_id(*child_id).desired_width();
        element_store.by_id(*child_id).physical_bounds = Rect::new(x, 0, w, 1);
        x += w;
    }
}

fn vertical_reflow(element_store : &mut ElementStore, children : &[usize]) {
    let mut y = 1;
    for child_id in children.iter() {
        let elem = element_store.element_by_id(*child_id);
        let h = elem.desired_height();
        let w = elem.desired_width();
        element_store.by_id(*child_id).physical_bounds = Rect::new(1, y, w, h);
        y += h;
    }
}