use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(1280, 720).title("hello world").build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::DARKGRAY);
        d.draw_text("Hello World!", 12, 12, 20, Color::WHITE);
    }
}
