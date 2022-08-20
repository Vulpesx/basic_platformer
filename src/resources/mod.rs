use raylib::prelude::*;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

mod texture_map;
pub use texture_map::TextureMap;

thread_local! {
    static TEXTURES: RefCell<TextureSet> = RefCell::new(TextureSet::default());
}

pub struct TextureSet {
    textures: HashMap<String, Rc<Texture2D>>,
}

impl Default for TextureSet {
    fn default() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }
}

impl TextureSet {
    pub fn texture(&self, id: &str) -> Option<Rc<Texture2D>> {
        self.textures.get(id).map(|t| Rc::clone(t))
    }

    pub fn load_texture(
        path: &str,
        id: Option<&str>,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) -> Result<(), String> {
        let texture = Rc::new(rl.load_texture(thread, path)?);

        TEXTURES.with(|t| {
            let mut t = t.borrow_mut();
            t.textures.insert(
                if let Some(i) = id {
                    i.to_string()
                } else {
                    path.to_string()
                },
                texture,
            )
        });

        Ok(())
    }

    pub fn get_texture(id: &str) -> Option<Rc<Texture2D>> {
        TEXTURES.with(|t| {
            let t = t.borrow();
            t.texture(id)
        })
    }
}
