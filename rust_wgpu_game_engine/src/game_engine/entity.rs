use super::textured_model::TexturedModel;
use glam::{Mat4, Quat, Vec2, Vec3};

#[derive(Clone)] // Added Clone derivation
pub struct Entity {
    pub model: TexturedModel,
    pub position: Vec3,
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub scale: f32,
    pub texture_index: u32, // Corrected typo from 'texure'
}

impl Entity {
    pub fn new(
        model: TexturedModel,
        position: Vec3,
        rot_x: f32,
        rot_y: f32,
        rot_z: f32,
        scale: f32,
        texture_index: u32,
    ) -> Self {
        Self {
            model,
            position,
            rot_x,
            rot_y,
            rot_z,
            scale,
            texture_index,
        }
    }

    pub fn set_texture_index(&mut self, new_index: u32) {
        self.texture_index = new_index;
    }

    pub fn increase_position(&mut self, dx: f32, dy: f32, dz: f32) {
        self.position += Vec3::new(dx, dy, dz);
    }

    pub fn increase_rotation(&mut self, dx: f32, dy: f32, dz: f32) {
        self.rot_x += dx;
        self.rot_y += dy;
        self.rot_z += dz;
    }

    pub fn get_model(&self) -> TexturedModel {
        // Assuming TexturedModel has a Clone derive or similar helper
        self.model.clone()
    }

    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    // Simplified texture offset logic
    pub fn get_texture_offset(&self) -> Vec2 {
        let rows = self.model.get_texture().number_of_rows as f32;
        let column = (self.texture_index as f32 % rows) / rows;
        let row = (self.texture_index as f32 / rows).floor() / rows;

        Vec2::new(column, row)
    }

    pub fn create_transformation_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            Vec3::splat(self.scale),
            Quat::from_euler(glam::EulerRot::XYZ, self.rot_x, self.rot_y, self.rot_z),
            self.position,
        )
    }
}
