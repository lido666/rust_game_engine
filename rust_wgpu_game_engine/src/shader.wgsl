struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normals: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

// Group 0: Texture and Sampler
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

// 1. Updated Uniform struct to match renderer.rs
struct EntityUniforms {
    transform_matrix: mat4x4<f32>,
    tex_offset: vec2<f32>,
    num_rows: f32,
};

@group(1) @binding(0)
var<uniform> uniforms: EntityUniforms;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // 2. Calculate Atlas UV coordinates:
    // Shrink the UV to the size of one tile, then add the entity's offset
    out.tex_coords = (model.tex_coords / uniforms.num_rows) + uniforms.tex_offset;
    
    out.clip_position = uniforms.transform_matrix * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}