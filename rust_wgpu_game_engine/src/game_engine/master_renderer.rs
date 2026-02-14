use super::{
    camera::Camera, entity::Entity, light::Light, renderer::Renderer, textured_model::TexturedModel,
};

use std::collections::HashMap;

pub struct MasterRenderer {
    renderer: Renderer,
    entities: HashMap<TexturedModel, Vec<Entity>>,
}

impl MasterRenderer {
    pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> Self {
        Self {
            entities: HashMap::new(),
            renderer: Renderer::new(device, layout),
        }
    }

    pub fn clear_entities(&mut self) {
        self.entities.clear();
    }

    pub fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        queue: &wgpu::Queue,
        _sun: &Light,
        camera: &Camera,
    ) {
        let view_matrix = camera.build_view_matrix();

        for (textured_model, entity_list) in &self.entities {
            // Bind model resources once
            self.renderer
                .bind_textured_model(render_pass, textured_model);
            // Render all entities using this model
            self.renderer
                .render_entities(render_pass, entity_list, queue, &view_matrix);
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        // Use the public model field from textured_model.rs
        self.entities
            .entry(entity.model.clone())
            .or_insert_with(Vec::new)
            .push(entity);
    }
}
