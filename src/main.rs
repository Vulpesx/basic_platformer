use raylib::prelude::*;

fn main() {
    const SWIDTH: f32 = 640.0;
    const SHEIGHT: f32 = 480.0;

    let mut quit = false;

    let (mut rl, thread) = raylib::init()
        .size(SWIDTH as i32, SHEIGHT as i32)
        .title("BAsic Platformer")
        .build();

    let map = rl
        .load_texture(&thread, "assets/tilemap_packed.png")
        .expect("could not load texture map!");
    let map = {
        let width = map.width() / 10;
        let height = map.height() / 6;
        TextureMap::new(map, width, height)
    };

    while !rl.window_should_close() && !quit {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::GRAY);
        d.draw_texture_rec(map.texture(), map.tile(0, 4), rvec2(0.0, 0.0), Color::WHITE);
        d.draw_text("WIP", 0, 0, 20, Color::SKYBLUE);
    }
}

struct TextureMap {
    map: Texture2D,
    tile_width: i32,
    tile_height: i32,
}

impl TextureMap {
    fn new(map: Texture2D, tile_width: i32, tile_height: i32) -> Self {
        Self {
            map,
            tile_width,
            tile_height,
        }
    }

    fn texture(&self) -> &Texture2D {
        &self.map
    }

    fn tile(&self, x: i32, y: i32) -> Rectangle {
        if x > self.map.width() / self.tile_width {
            panic!("x is out of bounds")
        }
        if y > self.map.height() / self.tile_height {
            panic!("y is out of bounds")
        }
        Rectangle {
            x: (x * self.tile_width) as f32,
            y: (y * self.tile_height) as f32,
            width: self.tile_width as f32,
            height: self.tile_height as f32,
        }
    }
}
