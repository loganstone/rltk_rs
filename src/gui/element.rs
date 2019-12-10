use super::{Rltk, Rect};
use std::any::Any;

pub trait Element {
    fn render(&self, _ctx : &mut Rltk, _physical_bounds : Rect) {}
    fn as_any(&mut self) -> &mut dyn Any;
}