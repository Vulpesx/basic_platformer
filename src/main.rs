use basic_platformer::math::*;
use basic_platformer::{resources::TextureMap, Scene};
use raylib::prelude::*;

mod phys_test;
use phys_test::*;
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
        .expect("could not load texture map!");
    let map = {
        let width = map.width / 10;
        let height = map.height / 6;
        TextureMap::new(map, width, height)
    };

    let player = Player::new(SWIDTH / 2.0, SHEIGHT / 2.0, &map);
    let mut scene = PhysTest::new(&thread); // let mut scene = TestScene::new(player, map);

    while !rl.window_should_close() {
        /* --- INPUT --- */
        scene.input(&mut rl);

        /* --- UPDATE --- */
        scene.update(&mut rl);

        /* --- DRAW --- */
        // let mut d = rl.begin_drawing(&thread);

        // d.clear_background(Color::GRAY);
        // scene.render(&mut d);
    }
}
