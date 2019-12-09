use super::{ElementInfo, RGB, Rltk, Element, TextUI, Placement, Rect, Console};
use std::any::Any;

pub struct StatusBarText {
    pub info : ElementInfo,
    pub text : String,
    pub fg : RGB,
    pub bg : RGB
}

impl StatusBarText {
    pub fn new<S: ToString>(ui : &TextUI, parent : Option<usize>, text : S) -> Box<StatusBarText> {
        let t = text.to_string();
        let length = t.len() as i32 + 1;
        let parent_id = parent.unwrap();
        let parent_bounds = ui.get_bounds(parent_id);
        let occupied_width = ui.get_child_widths(parent_id);
        Box::new(StatusBarText{            
            info : ElementInfo{
                placement: Placement::Relative,
                bounds : Rect::new(parent_bounds.x1 + occupied_width, parent_bounds.y1, length, 1),
                parent,
                children : Vec::new()
            },
            text : t,
            fg : ui.theme.status_bar_text.fg, 
            bg : ui.theme.status_bar_text.bg
        })
    }

    pub fn set_text<S: ToString>(&mut self, text : S) {
        self.text = text.to_string();
    }
}

impl Element for StatusBarText {
    fn get_info(&self) -> &ElementInfo { &self.info }
    fn get_info_mut(&mut self) -> &mut ElementInfo { &mut self.info }
    fn render(&self, ctx : &mut Rltk) {
        ctx.print_color(self.info.bounds.x1, self.info.bounds.y1, self.fg, self.bg, &self.text);
    }
    fn add_child(&mut self, id : usize) { self.info.children.push(id); }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}