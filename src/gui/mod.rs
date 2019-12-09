use crate::{Rltk, Rect, RGB, to_cp437, Console};
mod theme;
pub use theme::*;
mod widgets;
pub use widgets::{SolidBackground, StatusBar, StatusBarText, Window};
mod element_store;
use element_store::*;

pub struct TextUI {
    element_store : ElementStore,
    theme : Theme,
    base_element : usize
}

impl TextUI {
    pub fn new(theme : Theme) -> TextUI {
        TextUI{
            element_store : ElementStore::new(),
            theme,
            base_element : 0
        }
    }

    pub fn get_id<S : ToString>(&self, id : S) -> Option<&usize> {
        self.element_store.get_id(id)
    }

    pub fn add_explicit<S : ToString>(&mut self, key: S, element : Box<dyn Element>) -> usize {
        let parent_id_option = element.get_info().parent;
        self.element_store.add_element(key.to_string(), element, parent_id_option)
    }

    pub fn add<S : ToString>(&mut self, ctx : &mut Rltk, widget : WidgetType, key : S, parent : S) -> &mut Self {
        let parent_id = self.element_store.get_id(parent.to_string());
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
                self.add_explicit(key, StatusBar::new(ctx, &self.element_store, &self.theme, parent))
            }
            WidgetType::StatusText{text} => {
                self.add_explicit(key, StatusBarText::new(&self.element_store, &self.theme, parent, text))
            }
            WidgetType::Window{pos, title} => {
                self.add_explicit(key, Window::new(&self.element_store, &self.theme, parent, pos, title))
            }
        }
    }

    pub fn set_base<S : ToString>(&mut self, id : S) -> &mut Self {
        let key = self.element_store.get_id(id).unwrap();
        self.base_element = *key;
        self
    }

    pub fn render(&self, ctx : &mut Rltk) {
        self.element_store.render(ctx, self.base_element);
    }    

    pub fn element_by_id(&mut self, id : usize) -> &mut Box<dyn Element> {
        self.element_store.element_by_id(id)
    }
}

pub enum WidgetType {
    ScreenBackground,
    StatusBar,
    StatusText{ text : String },
    Window { pos : Rect, title : String }
}
