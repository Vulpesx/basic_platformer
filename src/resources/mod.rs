mod texture_map;
use std::ops::Deref;

use raylib::prelude::*;
pub use texture_map::*;

const INDEX_MASK: u32 = (1 << 16) - 1;
const MAGIC_MASK: u32 = !INDEX_MASK;

pub struct TextureHandle<'a> {
    handle: u32,
    manager: &'a TextureManager,
}

impl<'a> TextureHandle<'a> {
    pub fn get_ref(&self) -> Result<&Texture2D, String> {
        self.manager.get_ref(self)
    }

    pub fn get_ref_unchecked(&self) -> Result<&Texture2D, String> {
        self.manager.get_ref_unchecked(self)
    }
}

impl<'a> Drop for TextureHandle<'a> {
    fn drop(&mut self) {
        self.manager.remove(self.get_magic());
    }
}

impl<'a> Deref for TextureHandle<'a> {
    type Target = Texture2D;

    fn deref(&self) -> &Self::Target {
        self.get_ref().unwrap()
    }
}

impl<'a> ResourceHandle for TextureHandle<'a> {
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
    file_paths: Vec<String>,
    handles: Vec<u16>,
    last_handle: u16,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: vec![],
            file_paths: vec![],
            handles: vec![],
            last_handle: 0,
        }
    }
    pub fn load(
        &mut self,
        path: &str,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) -> Result<TextureHandle, String> {
        let index = self.textures.len();
        let magic = self.last_handle + 1;
        self.last_handle += 1;
        self.handles.push(magic);
        self.textures.push(rl.load_texture(thread, path)?);
        self.file_paths.push(path.to_string());
        Ok(TextureHandle {
            handle: (magic as u32) << 16 | index as u32,
            manager: self,
        })
    }

    pub fn get(&mut self, path: &str) -> Result<TextureHandle, String> {
        let path = path.to_string();
        if !self.file_paths.contains(&path) {
            return Err(String::from("file path not found in cache"));
        }

        for i in 0..self.file_paths.len() {
            if path == self.file_paths[i] {
                if self.textures.get(i).is_some() {
                    let index = i + 1;
                    let magic = self.last_handle + 1;
                    self.last_handle += 1;
                    self.handles.push(magic);
                    return Ok(TextureHandle {
                        handle: (magic as u32) << 16 | index as u32,
                        manager: self,
                    });
                } else {
                    return Err(String::from("could not find corresponding texture"));
                }
            }
        }

        Err(String::from(
            "unknown error, func should have returned before this",
        ))
    }

    pub fn get_ref(&self, handle: &TextureHandle) -> Result<&Texture2D, String> {
        if !self.handles.contains(&handle.get_magic()) {
            return Err(String::from("invalid handle"));
        }

        if let Some(t) = self.textures.get(handle.get_index() as usize) {
            return Ok(t);
        }

        Err(String::from("texture not found, invalid handle"))
    }

    pub fn get_ref_mut(&mut self, handle: &TextureHandle) -> Result<&mut Texture2D, String> {
        if !self.handles.contains(&handle.get_magic()) {
            return Err(String::from("invalid handle"));
        }

        if let Some(t) = self.textures.get_mut(handle.get_index() as usize) {
            return Ok(t);
        }

        Err(String::from("texture not found, invalid handle"))
    }

    pub fn get_ref_unchecked(&self, handle: &TextureHandle) -> Result<&Texture2D, String> {
        if let Some(t) = self.textures.get(handle.get_index() as usize) {
            return Ok(t);
        }

        Err(String::from("texture not found, invalid handle"))
    }

    pub fn get_ref_mut_unchecked(
        &mut self,
        handle: &TextureHandle,
    ) -> Result<&mut Texture2D, String> {
        if let Some(t) = self.textures.get_mut(handle.get_index() as usize) {
            return Ok(t);
        }

        Err(String::from("texture not found, invalid handle"))
    }

    pub fn new_handle(&mut self, handle: &TextureHandle) -> Result<TextureHandle, String> {
        if !self.handles.contains(&handle.get_magic()) {
            return Err(String::from("invalid handle"));
        }
        let index = handle.get_index() as u32;
        let magic = self.last_handle as u32 + 1;
        Ok(TextureHandle {
            handle: magic << 16 | index,
            manager: self,
        })
    }

    pub fn remove(&mut self, handle: u16) {
        for i in 0..self.handles.len() {
            if handle == self.handles[i] {
                self.handles.remove(i);
            }
        }
    }
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
