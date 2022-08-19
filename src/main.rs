use basic_platformer::Scene;
use raylib::prelude::*;

mod player;
use player::*;

pub mod consts {
    pub const SWIDTH: f32 = 640.0;
    pub const SHEIGHT: f32 = 480.0;
}
use consts::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SWIDTH as i32, SHEIGHT as i32)
        .title("BAsic Platformer")
        .build();

    let map = rl
        .load_texture(&thread, "assets/tilemap_packed.png")
        .unwrap();

    let player = Player::new(SWIDTH / 2.0, SHEIGHT / 2.0, &map);
    let mut scene = TestScene::new(player, rmanager.get("assets/tilemap_packed").unwrap());

    while !rl.window_should_close() {
        /* --- INPUT --- */
        scene.input(&mut rl);

        /* --- UPDATE --- */
        scene.update(&mut rl);

        /* --- DRAW --- */
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::GRAY);
        scene.render(&mut d);
    }
}
