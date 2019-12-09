use super::{ElementInfo, RGB, Rltk, Element, Placement, Rect, Console, ElementStore, to_cp437, Theme};
use std::any::Any;

pub struct Window {
    pub info : ElementInfo,
    pub glyph : u8,
    pub fg : RGB,
    pub bg : RGB,
    pub border_fg : RGB,
    pub border_bg : RGB,
    pub title : String,
    pub title_bg : RGB,
    pub title_fg : RGB
}

impl Window {
    pub fn new<S : ToString>(tui : &ElementStore, theme: &Theme, parent : Option<usize>, pos : Rect, title : S) -> Box<Window> {
        let parent_bounds = tui.get_bounds(parent.unwrap());
        Box::new(Window{
            info : ElementInfo{
                placement: Placement::Absolute,
                bounds : Rect::new(parent_bounds.x1 + pos.x1, parent_bounds.y1 + pos.y1, pos.width(), pos.height()),
                parent,
                children : Vec::new()
            },
            glyph : theme.status_bar_background.glyph,
            fg : theme.window_background.fg, 
            bg : theme.window_background.bg,
            border_fg : theme.window_border.fg,
            border_bg : theme.window_border.bg,
            title : title.to_string(),
            title_bg : theme.window_title.bg,
            title_fg : theme.window_title.fg
        })
    }
}

impl Element for Window {
    fn get_info(&self) -> &ElementInfo { &self.info }
    fn get_info_mut(&mut self) -> &mut ElementInfo { &mut self.info }
    fn render(&self, ctx : &mut Rltk) {
        let bounds = self.info.bounds;
        ctx.draw_box_double(bounds.x1, bounds.y1, bounds.width(), bounds.height(), self.border_fg, self.border_bg);
        ctx.set(bounds.x1 + 2, bounds.y1, self.border_fg, self.border_bg, to_cp437('■'));
        ctx.print_color(bounds.x1 + 3, bounds.y1, self.title_fg, self.title_bg, &self.title);
        ctx.set(bounds.x1 + 3 + self.title.len() as i32, bounds.y1, self.border_fg, self.border_bg, to_cp437('■'));
        for y in bounds.y1 + 1 .. bounds.y2 {
            for x in bounds.x1 +1 .. bounds.x2 {
                ctx.set(x, y, self.fg, self.bg, self.glyph);
            }
        }
    }
    fn add_child(&mut self, id : usize) { self.info.children.push(id); }
    fn get_child_widths(&self, ui : &ElementStore) -> i32 {
        let mut width : i32 = 0;
        for child in self.info.children.iter() {
            width += ui.get_bounds(*child).width();
        }
        width
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}