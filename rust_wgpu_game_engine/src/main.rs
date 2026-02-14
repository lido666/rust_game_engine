use rust_wgpu_game_engine::game_engine::{
    camera::Camera, // Added
    entity::Entity, // Import your entity
    light::Light,   // Added
    loader,
    main_shader,
    master_renderer::MasterRenderer, // Added
    model_texture,
    raw_model,
    renderer::Renderer, // Import your new renderer
    textured_model::TexturedModel,
};
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

// 1. The "State" holds all GPU resources (Device, Queue, Surface)
struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Arc<Window>,
    my_loader: loader::Loader,
    my_shader: main_shader::MainShader,
    // Note: my_renderer is usually managed by MasterRenderer
    my_master_renderer: MasterRenderer,
    camera: Camera,
    sun: Light,
    entities: Vec<Entity>, // Removed duplicate declaration
}

impl<'a> State<'a> {
    async fn new(window: Arc<Window>) -> State<'a> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
                memory_hints: Default::default(),
                ..Default::default()
            })
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        // Create the Layouts required by your shader.wgsl
        let texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let uniform_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("uniform_layout"),
        });

        let mut my_loader = loader::Loader::new(&device, queue.clone());
        let my_shader =
            main_shader::MainShader::new(&device, &config, &texture_layout, &uniform_layout);
        let my_master_renderer = MasterRenderer::new(&device, &uniform_layout); //

        let mut camera = Camera::new();
        camera.position.z = 10.0; // Move the camera back 5 units

        let sun = Light::new();

        let raw_model = my_loader.load_3d_model("cube.obj");
        let texture = my_loader.load_texture();
        let textured_model = TexturedModel::new(&raw_model, &texture);

        let entity = Entity::new(
            textured_model,
            glam::Vec3::new(0.0, 0.0, 0.0),
            0.0,
            0.0,
            0.0,
            1.0,
            0,
        );

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            my_loader,
            my_shader,
            my_master_renderer,
            camera,
            sun,
            entities: vec![entity],
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // 1. Prepare MasterRenderer
        self.my_master_renderer.clear_entities();
        for entity in &self.entities {
            self.my_master_renderer.add_entity(entity.clone());
        }

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                ..Default::default()
            });

            // 2. Execute Batch Rendering
            render_pass.set_pipeline(&self.my_shader.render_pipeline);
            self.my_master_renderer
                .render(&mut render_pass, &self.queue, &self.sun, &self.camera);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

struct App<'a> {
    state: Option<State<'a>>,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_none() {
            let window_attributes = Window::default_attributes().with_title("Rust Game Engine");

            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            let state = pollster::block_on(State::new(window));
            self.state = Some(state);
        }
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let state = match self.state.as_mut() {
            Some(state) => state,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                state.resize(physical_size);
            }
            WindowEvent::RedrawRequested => {
                state.window.request_redraw();
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(key_code),
                        ..
                    },
                ..
            } => {
                match key_code {
                    KeyCode::Escape => event_loop.exit(),
                    KeyCode::KeyA => {
                        for entity in &mut state.entities {
                            // Rotate by 0.5 degrees in radians
                            entity.increase_rotation(0.05, 0.0872665, 0.05);
                        }
                    }
                    _ => (),
                }
            } // Added missing closing brace for KeyboardInput
            _ => (),
        }
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App { state: None };
    event_loop.run_app(&mut app).unwrap();
}
