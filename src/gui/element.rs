use super::{Rltk, Rect};
use std::any::Any;

pub enum ReflowType {
    None, Horizontal, Vertical
}

pub trait Element {
    fn render(&self, _ctx : &mut Rltk, _physical_bounds : Rect) {}
    fn as_any(&mut self) -> &mut dyn Any;
    fn is_container(&self) -> ReflowType { ReflowType::None }
    fn flow_dirty(&self) -> bool { false }
    fn mark_flow_clean(&mut self) {}
    fn desired_width(&self) -> i32 { 0 }
    fn desired_height(&self) -> i32 { 1 }
}