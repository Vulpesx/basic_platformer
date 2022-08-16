mod texture_map;
use raylib::texture::Texture2D;
pub use texture_map::*;

const INDEX_MASK: u32 = (1 << 16) - 1;
const MAGIC_MASK: u32 = !INDEX_MASK;

pub struct TextureHandle {
    handle: u32,
}

impl ResourceHandle for TextureHandle {
    fn handle(&self) -> u32 {
        self.handle
    }

    fn handle_mut(&mut self) -> &mut u32 {
        &mut self.handle
    }
}

trait ResourceHandle {
    fn handle(&self) -> u32;
    fn handle_mut(&mut self) -> &mut u32;

    fn get_index(&self) -> u16 {
        (self.handle() & INDEX_MASK) as u16
    }

    fn get_magic(&self) -> u16 {
        ((self.handle() & MAGIC_MASK) >> 16) as u16
    }
}

pub struct TextureManager {
    textures: Vec<Texture2D>,
}

#[cfg(test)]
mod tests {
    use crate::resources::ResourceHandle;

    use super::TextureHandle;

    fn handle() -> TextureHandle {
        TextureHandle {
            handle: 0b01101101101101101001001001001001,
        }
    }

    #[test]
    fn test_get_index() {
        let handle = handle();
        assert_eq!(handle.get_index(), 0b1001001001001001);
    }

    #[test]
    fn test_get_magic() {
        let handle = handle();
        assert_eq!(handle.get_magic(), 0b0110110110110110);
    }
}
