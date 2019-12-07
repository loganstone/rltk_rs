use crate::{Rltk, console::Console, Rect};
use std::collections::HashMap;
use super::*;

pub struct UI {
    pub base_element : String,
    pub theme : Theme,
    elements : HashMap<String, Box<dyn Element>>
}

impl UI {
    pub fn new(theme : Theme) -> UI {
        UI {
            base_element : "".to_string(),
            elements : HashMap::new(),
            theme
        }
    }

    pub fn set_base(&mut self, id : &str) -> &mut UI {
        self.base_element = id.to_string();
        self
    }

    pub fn add(&mut self, ctx: &mut Rltk, id : &str, parent : &str, widget : WidgetType) -> &mut UI {
        match widget {
            WidgetType::Background => self.add_explicit(parent, Background::default(ctx, id, self.theme)),
            WidgetType::MenuBar => self.add_explicit(parent, MenuBar::default(ctx, id, self.theme)),
            WidgetType::StatusBar => self.add_explicit(parent, StatusBar::default(ctx, id, self.theme))
        };
        self
    }

    pub fn add_explicit(&mut self, parent: &str, element : Box<dyn Element>) -> &mut UI {
        let id = element.get_id().to_string();
        self.elements.insert(id.clone(), element);
        if let Some(p) = self.elements.get_mut(parent) {
            p.add_child(&id);            
        }
        self
    }

    pub fn render(&self, ctx : &mut Rltk) -> Vec<Event> {
        let size = ctx.get_char_size();
        let b = Rect::new(0, 0, size.0 as i32, size.1 as i32);
        let mut events : Vec<Event> = Vec::new();
        self.render_element(ctx, &self.base_element, b, &mut events);
        events
    }

    fn render_element(&self, ctx : &mut Rltk, id : &str, parent : Rect, events : &mut Vec<Event>) {
        if let Some(e) = self.elements.get(id) {
            e.render(ctx, parent, events);

            let b = e.get_bounds();
            for child in e.get_children().iter() {
                self.render_element(ctx, child, b, events);
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

pub enum WidgetType {
    Background,
    MenuBar,
    StatusBar
}