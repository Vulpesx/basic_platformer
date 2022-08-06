use raylib::prelude::*;

fn main() {
    const SWIDTH: f32 = 640.0;
    const SHEIGHT: f32 = 480.0;

    let mut quit = false;

    let (mut rl, thread) = raylib::init()
        .size(SWIDTH as i32, SHEIGHT as i32)
        .title("BAsic Platformer")
        .build();

    while !rl.window_should_close() && !quit {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::GRAY);
        d.draw_text("WIP", 0, 0, 20, Color::SKYBLUE);
    }
}
