use crate::raylib::TextureError;

pub struct Assets {
    // TODO: Fill your romfs assets
    // for example, Textures
}

impl Assets {
    pub fn load() -> Result<Self, TextureError> {
        Ok(Self {
            // TODO: Load your assets from romfs
            // Texture::load(c"romfs:/...")?
        })
    }
}
