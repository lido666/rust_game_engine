use super::{model_texture::ModelTexture, raw_model::RawModel};
use std::hash::Hash;

/// Textured model combines a texture and a 3d model
#[derive(Clone)]
pub struct TexturedModel {
    pub model: RawModel,
    pub texture: ModelTexture,
}

impl Hash for TexturedModel {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.texture.id + self.texture.number_of_rows).hash(state)
    }
}

impl std::cmp::Eq for TexturedModel {}

impl PartialEq for TexturedModel {
    fn eq(&self, other: &Self) -> bool {
        self.texture.id == other.texture.id
    }
}

impl TexturedModel {
    pub fn new(model: &RawModel, text_id: &ModelTexture) -> Self {
        Self {
            model: model.clone(),
            texture: text_id.clone(),
        }
    }

    pub fn get_texture(&self) -> ModelTexture {
        self.texture.clone()
    }

    pub fn get_model(&self) -> RawModel {
        self.model.clone()
    }
}
