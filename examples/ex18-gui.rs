rltk::add_wasm_support!();

use rltk::{Console, GameState, Rltk, element, Rect, RGB};
use rltk::gui::*;

struct State {
    pub ui : Option<TextUI>
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        if let Some(ui) = &mut self.ui {

            if let Some(fps) = element!(ui, *ui.get_id("fps").unwrap(), StatusBarText) {
                fps.set_text(format!("FPS: {}", ctx.fps));
            }
            if let Some(ft) = element!(ui, *ui.get_id("frametime").unwrap(), StatusBarText) {
                ft.set_text(format!("Frame Time: {}", ctx.frame_time_ms));
            }

            ui.render(ctx);
        } else {
            let mut ui = TextUI::new(Theme::turbo_vision());
            ui
                .add(ctx, WidgetType::ScreenBackground, "background", "")
                .set_base("background")
                .add(ctx, WidgetType::StatusBar, "statusbar", "background")
                .add(ctx, WidgetType::StatusText{text : "FPS: 00".to_string()}, "fps", "statusbar")
                .add(ctx, WidgetType::StatusText{text : "Frame Time: 00".to_string()}, "frametime", "statusbar")
                .add(ctx, WidgetType::Window{ pos : Rect::new(5,5,40,5), title: "Hello Window".to_string() }, "win1", "background")
                .add(ctx, WidgetType::PlainText{ text : "This is a plain text line.".to_string(), fg : RGB::named(rltk::YELLOW), bg : RGB::named(rltk::BLACK) }, "b1", "win1")
                .add(ctx, WidgetType::PlainText{ text : "It doesn't do wrapping yet".to_string(), fg : RGB::named(rltk::CYAN), bg : RGB::named(rltk::BLACK) }, "b2", "win1")
                .add(ctx, WidgetType::PlainText{ text : "but vertical re-flow is working".to_string(), fg : RGB::named(rltk::CYAN), bg : RGB::named(rltk::BLACK) }, "b3", "win1")
                ;
                
            self.ui = Some(ui);
        }
    }    
}

impl State {
}

fn main() {
    let context = Rltk::init_simple8x8(80, 50, "Hello GUI", "resources");

    let gs: State = State { ui : None };
    rltk::main_loop(context, gs);
}
