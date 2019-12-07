use super::{Rltk, Rect, Event};
use std::any::Any;

pub trait Element {
    fn render(&self, ctx : &mut Rltk, parent : Rect, events : &mut Vec<Event>);
    fn get_bounds(&self) -> Rect;
    fn get_children(&self) -> &[String];
    fn get_id(&self) -> &str;
    fn add_child(&mut self, id : &str);
    fn as_any(&mut self) -> &mut dyn Any;
}