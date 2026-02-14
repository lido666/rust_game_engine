///
/// RawModel - structure and class used to store raw model
///

#[derive(Clone)]
pub struct RawModel {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub num_indices: u32,
}
