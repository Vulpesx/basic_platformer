use raylib::{RaylibHandle, RaylibThread};

pub struct TileMap {
    width: u32,
    height: u32,
    tiles: Vec<u8>,
}

impl TileMap {
    pub fn new(path: &str, rl: &mut RaylibHandle, thread: &RaylibThread) -> TileMap {
        todo!()
    }
}
