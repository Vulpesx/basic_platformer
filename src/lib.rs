use std::collections::HashMap;

use raylib::consts::KeyboardKey as Key;
use raylib::prelude::*;

pub struct TextureMap {
    map: Texture2D,
    tile_width: i32,
    tile_height: i32,
}

impl TextureMap {
    pub fn new(map: Texture2D, tile_width: i32, tile_height: i32) -> Self {
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

pub struct InputMap {
    map: HashMap<Key, String>,
}

impl InputMap {
    pub fn get_mapping(&self, key: Key) -> String {
        if let Some(m) = self.map.get(&key) {
            m.to_string()
        } else {
            String::from("unmapped")
        }
    }

    pub fn add_mapping(&mut self, key: Key, map: &str) -> Option<String> {
        self.map.insert(key, map.to_string())
    }

    pub fn unmap_key(&mut self, key: Key) -> Option<(Key, String)> {
        self.map.remove_entry(&key)
    }

    pub fn force_mapping(&mut self, key: Key, map: &str) -> Option<String> {
        if self.map.contains_key(&key) {
            let prev = self.map.get(&key).unwrap().to_string();
            self.map.insert(key, map.to_string());
            Some(prev)
        } else {
            None
        }
    }
}

pub trait Scene {
    fn update(&mut self, rl: &mut RaylibHandle);
    fn render(&self, d: &mut RaylibDrawHandle);
}
