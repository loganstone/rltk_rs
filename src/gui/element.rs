use super::{Rltk, Rect, Event, UI};
use std::any::Any;

pub trait Element {
    fn render(&self, ctx : &mut Rltk, parent : Rect, events : &mut Vec<Event>);
    fn get_bounds(&self) -> Rect;
    fn set_bounds(&mut self, new_bounds : Rect);
    fn get_children(&self) -> &[String];
    fn get_id(&self) -> &str;
    fn add_child(&mut self, id : &str);
    fn as_any(&mut self) -> &mut dyn Any;
    fn should_reflow(&self) -> bool { false }
    fn calc_width(&self) -> i32 {
        let bounds = self.get_bounds();
        i32::abs(bounds.x2 - bounds.x1)
    }
    fn calc_height(&self) -> i32 {
        let bounds = self.get_bounds();
        i32::abs(bounds.y2 - bounds.y1)
    }
}