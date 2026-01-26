use crate::handles::TextureHandle;
use crate::texture::Texture;
use glow::Context;
use image::GenericImageView;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::hash::{Hash, Hasher}; // cargo add image

#[derive(Default)]
pub struct TextureResourceManager {
    pub textures: HashMap<TextureHandle, Texture>,
}

impl TextureResourceManager {
    pub fn add_texture(&mut self, texture: Texture) -> TextureHandle {
        let id = texture.id;
        self.textures.insert(id, texture);
        id
    }

    pub fn load_from_file(&mut self, gl: &Context, path: &OsStr) -> TextureHandle {
        // Load image with the `image` crate
        let img = image::open(path)
            .expect("Failed to open texture image")
            .flipv();
        let rgba = img.to_rgba8();
        let (width, height) = img.dimensions();
        let id: TextureHandle = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            path.hash(&mut hasher);
            TextureHandle(hasher.finish() as u32)
        };

        let mut tex = Texture::new(id, width, height);
        tex.upload_to_gpu(gl, &rgba);
        self.add_texture(tex)
    }

    pub fn get_texture(&self, id: TextureHandle) -> Option<&Texture> {
        self.textures.get(&id)
    }
}
