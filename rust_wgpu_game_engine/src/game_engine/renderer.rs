use super::entity::Entity;
use super::textured_model::TexturedModel;
use glam::Mat4;

pub struct Renderer {
    pub transform_buffer: wgpu::Buffer,
    pub transform_bind_group: wgpu::BindGroup,
}

impl Renderer {
    pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> Self {
        let transform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Transform Buffer"),
            size: std::mem::size_of::<[[f32; 4]; 4]>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let transform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: transform_buffer.as_entire_binding(),
            }],
            label: Some("transform_bind_group"),
        });

        Self {
            transform_buffer,
            transform_bind_group,
        }
    }

    /// Binds resources shared by all entities of this model type
    pub fn bind_textured_model<'a>(
        &self,
        render_pass: &mut wgpu::RenderPass<'a>,
        textured_model: &'a TexturedModel,
    ) {
        render_pass.set_vertex_buffer(0, textured_model.model.vertex_buffer.slice(..));
        render_pass.set_index_buffer(
            textured_model.model.index_buffer.slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.set_bind_group(0, &textured_model.texture.diffuse_bind_group, &[]);
    }

    pub fn render_entities<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        entities: &'a Vec<Entity>,
        queue: &wgpu::Queue,
        view_matrix: &Mat4,
    ) {
        let aspect = 1.77; // Ensure this matches your window aspect ratio

        // Switch to Right-Handed Perspective
        // fov: 45 degrees, near plane: 0.1, far plane: 100.0
        let projection = Mat4::perspective_rh(45.0f32.to_radians(), aspect, 0.1, 100.0);

        let pv_matrix = projection * *view_matrix;

        for entity in entities {
            let matrix = pv_matrix * entity.create_transformation_matrix();
            let matrix_array: [[f32; 4]; 4] = matrix.to_cols_array_2d();

            queue.write_buffer(
                &self.transform_buffer,
                0,
                bytemuck::cast_slice(&[matrix_array]),
            );

            render_pass.set_bind_group(1, &self.transform_bind_group, &[]);
            render_pass.draw_indexed(0..entity.model.model.num_indices, 0, 0..1);
        }
    }
}
