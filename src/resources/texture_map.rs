use raylib::prelude::*;
use std::rc::Rc;

pub struct TextureMap {
    map: Rc<Texture2D>,
    tile_width: i32,
    tile_height: i32,
}

impl TextureMap {
    pub fn new(map: Rc<Texture2D>, tile_width: i32, tile_height: i32) -> Self {
        Self {
            map,
            tile_width,
            tile_height,
        }
    }

    pub fn texture(&self) -> &Texture2D {
        &self.map
    }

    pub fn tile(&self, x: i32, y: i32) -> Rectangle {
        if x > self.map.width / self.tile_width {
            panic!("x is out of bounds")
        }
        if y > self.map.height / self.tile_height {
            panic!("y is out of bounds")
        }
        Rectangle {
            x: (x * self.tile_width) as f32,
            y: (y * self.tile_height) as f32,
            width: self.tile_width as f32,
            height: self.tile_height as f32,
        }
    }

    pub fn tile_width(&self) -> i32 {
        self.tile_width
    }

    pub fn tile_height(&self) -> i32 {
        self.tile_height
    }
}
