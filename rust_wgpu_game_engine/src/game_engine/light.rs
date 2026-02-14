use bytemuck::{Pod, Zeroable};
use glam::Vec3;

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Light {
    pub position: Vec3,
    // Note: Some wgpu setups require 4-byte padding after a Vec3
    // to satisfy WGSL's 16-byte alignment rules for uniforms.
    pub _padding: f32,
    pub colour: Vec3,
    pub _padding2: f32,
}

impl Light {
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO, // Shorthand for (0.0, 0.0, 0.0)
            _padding: 0.0,
            colour: Vec3::ONE, // Shorthand for (1.0, 1.0, 1.0)
            _padding2: 0.0,
        }
    }

    // In Rust, we usually just access public fields,
    // but if you want getters:
    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    pub fn get_colour(&self) -> Vec3 {
        self.colour
    }
}
