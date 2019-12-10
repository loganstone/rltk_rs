use super::{Rltk, Rect, Element};

pub enum Placement {
    Absolute,
    Relative
}

pub struct ElementStorage {
    deleted : bool,
    visible : bool,
    pub physical_bounds : Rect,
    pub element : Box<dyn Element>,
    pub placement : Placement,
    pub parent : Option<usize>,
    pub children : Vec<usize>
}

impl ElementStorage {
    pub fn new(element : Box<dyn Element>, physical_bounds : Rect, placement : Placement, parent : Option<usize>) -> ElementStorage {
        ElementStorage {
            deleted : false,
            visible : true,
            element,
            physical_bounds,
            placement,
            parent,
            children : Vec::new()
        }
    }

    pub fn render(&self, ctx : &mut Rltk, parent_bounds : Rect) -> Option<Rect> {
        if !self.deleted && self.visible {
            let bounds = match self.placement {
                Placement::Absolute => self.physical_bounds,
                Placement::Relative => self.physical_bounds + parent_bounds
            };
            self.element.render(ctx, bounds);
            return Some(bounds);
        }
        None
    }
}