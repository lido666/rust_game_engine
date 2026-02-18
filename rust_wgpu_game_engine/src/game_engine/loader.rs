use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::game_engine::{model_texture::ModelTexture, raw_model::RawModel};
///
/// Loader - loads models and textures
///
use wgpu::util::DeviceExt;

/// vertext data struct implementation
/// this is used for loading in to arrays
/// from .obj file so that they can be used to create a
/// rawmodel
///
pub struct ObjData {
    pub vertices: Vec<f32>,
    pub normal: Vec<f32>,
    pub texture: Vec<f32>,
    pub indice: Vec<u32>,
}

///
/// stringmap implementation
/// simple key based lookup -

pub struct StringMap {
    internal_map: HashMap<String, usize>,
}

impl StringMap {
    pub fn new() -> Self {
        Self {
            internal_map: HashMap::new(),
        }
    }

    /// adds a string to the map and returns its index
    /// if the string is not in the map, it adds it

    pub fn add(&mut self, index: &String) -> usize {
        let result = self.internal_map.get_key_value(index);
        match result {
            Some(t) => {
                return *t.1;
            }
            None => {
                let map_size = self.internal_map.len();
                self.internal_map.insert(index.clone(), map_size.clone());
                return map_size;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.internal_map.len()
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        tex_coords: [0.5, 0.0],
        normal: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        tex_coords: [0.0, 0.4],
        normal: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        tex_coords: [1.0, 0.4],
        normal: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        tex_coords: [0.0, 0.4],
        normal: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        tex_coords: [0.2, 1.0],
        normal: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        tex_coords: [1.0, 0.4],
        normal: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        tex_coords: [0.2, 1.0],
        normal: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        tex_coords: [0.8, 1.0],
        normal: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        tex_coords: [1.0, 0.4],
        normal: [1.0, 1.0, 1.0],
    },
];

const INDICES: &[u16] = &[0, 1, 2, 3, 4, 5, 6, 7, 8];

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    //vertices
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    //texture coords
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    //normals
                    offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub struct Loader {
    device: wgpu::Device,
    queue: wgpu::Queue,
    texture_counter: u32,
}

impl Loader {
    pub fn new(device: &wgpu::Device, queue: wgpu::Queue) -> Self {
        Self {
            device: device.clone(),
            queue,
            texture_counter: 0,
        }
    }

    pub fn triangle(&mut self) -> RawModel {
        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            });

        RawModel {
            vertex_buffer,
            index_buffer,
            num_vertices: VERTICES.len() as u32,
            num_indices: INDICES.len() as u32,
        }
    }

    pub fn load_texture(&mut self) -> ModelTexture {
        let diffuse_bytes = include_bytes!("../../res/maze.png");
        let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
        let diffuse_rgba = diffuse_image.to_rgba8();
        let dimensions = image::GenericImageView::dimensions(&diffuse_image);

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let diffuse_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("diffuse_texture"),
            view_formats: &[],
        });

        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &diffuse_rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size,
        );

        let diffuse_texture_view =
            diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let texture_bind_group_layout =
            self.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled: false,
                                view_dimension: wgpu::TextureViewDimension::D2,
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                    label: Some("texture_bind_group_layout"),
                });

        let diffuse_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        self.texture_counter += 1;
        ModelTexture::new(self.texture_counter, diffuse_bind_group)
    }

    ///
    /// helper function to split strings into a vector by whitespace
    ///
    ///

    fn split_string_to_vec(input: &str) -> Vec<String> {
        input.split_whitespace().map(|s| s.to_string()).collect()
    }

    fn split_string_to_vec_using_slash(input: &str) -> Vec<String> {
        input.split('/').map(|s| s.to_string()).collect()
    }

    ///
    /// Object loader reads in a wacefront object file and creates a raw model using the data
    ///

    pub fn load_obj_file(filename: &str) -> ObjData {
        let file = File::open(filename);
        match file {
            Err(_) => {
                panic!("missing object file - can not continue: {}", filename);
            }
            Ok(file) => {
                // lets open our file and start reading through it.
                let reader = BufReader::new(file);

                // create our vectors for final model data
                let mut our_model = ObjData {
                    vertices: Vec::new(),
                    texture: Vec::new(),
                    normal: Vec::new(),
                    indice: Vec::new(),
                };

                // create temporrary structures for holding data we read in from
                // model file (obj)

                let mut vertices: Vec<glam::Vec3> = Vec::new();
                let mut textures: Vec<glam::Vec2> = Vec::new();
                let mut normals: Vec<glam::Vec3> = Vec::new();
                let mut indicies_to_process: Vec<String> = Vec::new();

                // Iterate over the lines of the file
                for line_from_file in reader.lines() {
                    //                    println!("{:?}", line_from_file);
                    let line = line_from_file.unwrap();
                    if line.is_empty() {
                        continue;
                    }
                    let current_line = Loader::split_string_to_vec(&line);

                    // check to see if we have vector co-ordinates
                    if line.starts_with("v ") {
                        let x: f32 = current_line[1].parse().unwrap();
                        let y: f32 = current_line[2].parse().unwrap();
                        let z: f32 = current_line[3].parse().unwrap();

                        //todo : store vertex coorrdinates
                        vertices.push(glam::Vec3::new(x, y, z));
                    }
                    if line.starts_with("vt") {
                        let x: f32 = current_line[1].parse().unwrap();
                        let y: f32 = current_line[2].parse().unwrap();
                        //todo : store texture coordinates
                        textures.push(glam::Vec2::new(x, y));
                    }

                    if line.starts_with("vn") {
                        let x: f32 = current_line[1].parse().unwrap();
                        let y: f32 = current_line[2].parse().unwrap();
                        let z: f32 = current_line[3].parse().unwrap();
                        //todo : store normals coorrdinates
                        normals.push(glam::Vec3::new(x, y, z));
                    }
                    if line.starts_with("f ") {
                        let mut x = 1;
                        while x < current_line.len() {
                            let entry = current_line[x].clone();
                            indicies_to_process.push(entry.clone());
                            x += 1;
                        }
                    }
                }

                // now we start mapping
                // now process indexices.  for each unique combination of indexices
                // we will build the vertices/normals and texture coordinates
                // step one is to get a lsit of unique cobinations

                let mut my_map = StringMap::new();
                for x in indicies_to_process.clone() {
                    // build a unique list of vertices / texture / normals combinations
                    // these will be used to build our final matrices below
                    my_map.add(&x);
                }

                // now we store results into indidvual arrays in the correct order
                for item in indicies_to_process.clone() {
                    let current_indice = Loader::split_string_to_vec_using_slash(&item);
                    // first item = vertex, 2nd is texture and third is normal
                    let vertex_index = current_indice[0].parse::<usize>().unwrap();
                    let texture_index = current_indice[1].parse::<usize>().unwrap();
                    let normal_index = current_indice[2].parse::<usize>().unwrap(); //println!("{}  {:?}", vertex_index, vertices[vertex_index - 1]);
                    our_model.vertices.push(vertices[vertex_index - 1].x);
                    our_model.vertices.push(vertices[vertex_index - 1].y);
                    our_model.vertices.push(vertices[vertex_index - 1].z);
                    our_model.texture.push(textures[texture_index - 1].x);
                    our_model.texture.push(textures[texture_index - 1].y);
                    our_model.normal.push(normals[normal_index - 1].x);
                    our_model.normal.push(normals[normal_index - 1].y);
                    our_model.normal.push(normals[normal_index - 1].z);
                    let indice_count = (our_model.vertices.len() / 3) - 1;
                    our_model.indice.push(indice_count as u32);
                }
                return our_model;
            }
        }
    }

    /// externally visible loader
    ///

    /// Externally visible loader that converts ObjData into a RawModel
    pub fn load_3d_model(&mut self, filename: &str) -> RawModel {
        let model_data: ObjData = Loader::load_obj_file(filename);

        // model_data has flat Vec<f32> for vertices, texture coords, and normals.
        // We need to interlace them into a single Vec<Vertex>.
        let mut vertices: Vec<Vertex> = Vec::new();
        let num_vertices = model_data.vertices.len() / 3;

        for i in 0..num_vertices {
            vertices.push(Vertex {
                position: [
                    model_data.vertices[i * 3],
                    model_data.vertices[i * 3 + 1],
                    model_data.vertices[i * 3 + 2],
                ],
                tex_coords: [model_data.texture[i * 2], model_data.texture[i * 2 + 1]],
                normal: [
                    model_data.normal[i * 3],
                    model_data.normal[i * 3 + 1],
                    model_data.normal[i * 3 + 2],
                ],
            });
        }

        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices), // Pass a reference to the slice
                usage: wgpu::BufferUsages::VERTEX,
            });

        // Convert indices to u16 to match the format used in renderer.rs
        let indices_u16: Vec<u16> = model_data.indice.iter().map(|&v| v as u16).collect();

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices_u16), // Pass a reference to the slice
                usage: wgpu::BufferUsages::INDEX,
            });

        RawModel {
            vertex_buffer,
            index_buffer,
            num_vertices: vertices.len() as u32,
            num_indices: indices_u16.len() as u32,
        }
    }
}
