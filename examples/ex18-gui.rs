rltk::add_wasm_support!();

use rltk::{Console, GameState, Rltk, RGB};
use rltk::gui::*;
use rltk::element;

struct State {
    gui : Option<UI>
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        if let Some(gui) = &mut self.gui {
            ctx.cls();

            if let Some(fps) = element!(gui, "fps", StatusText) {
                fps.text = format!("FPS: {}", ctx.fps);
            }            

            let events = gui.render(ctx);
            if let Some(body) = element!(gui, "body", StatusText) {
                body.text = "Body text goes here. One day, it will be pretty.".to_string();
            }
            for e in events.iter().filter(|e| e.widget == "quit") {
                match e.event_type {
                    EventType::MouseOver => {
                        if let Some(body) = element!(gui, "body", StatusText) {
                            body.text = "Hovering over the exit".to_string();
                        }
                    }
                    EventType::Clicked => {
                        ctx.quit();
                    }
                }
            }
        } else {
            self.gui = Some(self.build_gui(ctx));
        }
    }    
}

impl State {
    fn build_gui(&self, ctx : &mut Rltk) -> UI {
        let theme = Theme::turbo_vision();
        let mut ui = UI::new(theme);
        ui.add(ctx, "bg", "", WidgetType::Background)
            .set_base("bg")
            .add(ctx, "menubar", "bg", WidgetType::MenuBar)
            .add(ctx, "statusbar", "bg", WidgetType::StatusBar)
            .add(ctx, "quit", "statusbar", WidgetType::StatusButton{ text : "Exit".to_string(), hotkey: "Alt-X".to_string() })
            .add(ctx, "fps", "statusbar", WidgetType::StatusText{ text: "FPS: N/A".to_string()})
            .add_explicit("menubar", PlainText::default(ctx, "F", "F", 1, 0, RGB::named(rltk::RED), RGB::named(rltk::LIGHT_GRAY)))
            .add_explicit("menubar", PlainText::default(ctx, "file", "ile", 2, 0, RGB::named(rltk::NAVY), RGB::named(rltk::LIGHT_GRAY)))
            .add_explicit("bg", Window::new(ctx, "win", 10, 10, 50, 30, "This is a window"))
            .add_explicit("win", PlainText::default(ctx, "body", "Body text", 0, 0, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK)));
        ui
    }
}

fn main() {
    let context = Rltk::init_simple8x8(80, 50, "Hello GUI", "resources");

    let gs: State = State { gui : None };
    rltk::main_loop(context, gs);
}
