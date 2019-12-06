rltk::add_wasm_support!();

use rltk::{Console, GameState, Rltk, RGB};
use rltk::gui::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        let mut bg = Background::default(ctx);
        let mut sb = StatusBar::default(ctx);
        sb.add_child(PlainText::default(ctx, "Alt-X", 1, 0, RGB::named(rltk::RED), RGB::named(rltk::LIGHT_GRAY)));
        sb.add_child(PlainText::default(ctx, "Exit", 7, 0, RGB::named(rltk::NAVY), RGB::named(rltk::LIGHT_GRAY)));
        bg.add_child(sb);
        let mut mb = MenuBar::default(ctx);
        mb.add_child(PlainText::default(ctx, "F", 1, 0, RGB::named(rltk::RED), RGB::named(rltk::LIGHT_GRAY)));
        mb.add_child(PlainText::default(ctx, "ile", 2, 0, RGB::named(rltk::NAVY), RGB::named(rltk::LIGHT_GRAY)));
        bg.add_child(mb);

        let mut window = Window::new(ctx, 10, 10, 50, 30, "This is a window");
        window.add_child(PlainText::default(ctx, "Body text", 0, 0, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK)));
        bg.add_child(window);
        let ui = UI::new(bg);
        ui.render(ctx);
    }
}

fn main() {
    let context = Rltk::init_simple8x8(80, 50, "Hello GUI", "resources");

    let gs: State = State {};
    rltk::main_loop(context, gs);
}
