rltk::add_wasm_support!();

use rltk::{Console, GameState, Rltk, element, Rect, RGB};
use rltk::gui::*;

struct State {
    pub ui : Option<TextUI>,
    pub iteration : u64
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        if let Some(ui) = &mut self.ui {

            ui.render(ctx);
            self.iteration += 1;

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
                .add(ctx, WidgetType::Window{ pos : Rect::new(10,15,40,5), title: "Second Window".to_string() }, "win2", "background")
                .add(ctx, WidgetType::PlainText{ text : "This is another plain text line.".to_string(), fg : RGB::named(rltk::YELLOW), bg : RGB::named(rltk::BLACK) }, "b4", "win2")
                .bind_update("frametime", |ctx, e| {
                    e.as_any().downcast_mut::<StatusBarText>().unwrap().set_text(format!("Frame Time: {}", ctx.frame_time_ms));
                })
                .bind_update("fps", |ctx, e| {
                    e.as_any().downcast_mut::<StatusBarText>().unwrap().set_text(format!("FPS: {}", ctx.fps));
                })
                .bind_update("b4", |ctx, e| {
                    e.as_any().downcast_mut::<PlainText>().unwrap().set_text(format!("Mouse Position: {}, {}", ctx.mouse_pos().0, ctx.mouse_pos().1));
                })
                ;

            //ui.(*ui.get_id("b4").unwrap()).on_update = Some(|| {});

            self.ui = Some(ui);
        }
    }
}

impl State {
}

fn main() {
    let context = Rltk::init_simple8x8(80, 50, "Hello GUI", "resources");

    let gs: State = State { ui : None, iteration : 0 };
    rltk::main_loop(context, gs);
}
