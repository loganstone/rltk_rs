use crate::{Rltk, console::Console, Rect};
use std::collections::HashMap;
use super::{Element, Event};

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
