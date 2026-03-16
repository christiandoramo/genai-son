use std::sync::Arc;
use winit::window::Window;
use winit::keyboard::KeyCode;
use sysinfo::System;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameMode { God, Normal }

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub resolution: [f32; 2],
    pub time: f32,
    pub _padding1: f32,
    pub camera_pos: [f32; 3],
    pub _padding2: f32,
    pub camera_front: [f32; 3],
    pub _padding3: f32,
}

// Mundo maior! 512x128x512 = ~134MB de VRAM
const WORLD_X: usize = 512;
const WORLD_Y: usize = 128;
const WORLD_Z: usize = 512;

pub struct State<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub render_pipeline: wgpu::RenderPipeline,
    pub window: Arc<Window>,
    pub uniform_buffer: wgpu::Buffer,
    
    #[allow(dead_code)] // Avisa ao Rust que a GPU usa isso, não a CPU
    pub world_buffer: wgpu::Buffer, 
    
    pub bind_group: wgpu::BindGroup,
    
    pub start_time: std::time::Instant,
    pub last_frame_time: std::time::Instant,
    pub last_fps_time: std::time::Instant,
    pub frame_count: u32,

    pub sys: System, // Monitor de CPU

    pub mode: GameMode,
    pub camera_pos: [f32; 3],
    pub yaw: f32,
    pub pitch: f32,
    pub keys_pressed: [bool; 6], 
}

impl<'a> State<'a> {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await.unwrap();
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter().copied().find(|f| f.is_srgb()).unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width.max(1), height: size.height.max(1),
            present_mode: wgpu::PresentMode::Fifo, 
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![], desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let initial_cam_pos = [WORLD_X as f32 / 2.0, 80.0, WORLD_Z as f32 / 2.0];
        let uniforms = Uniforms {
            resolution: [config.width as f32, config.height as f32],
            time: 0.0, _padding1: 0.0,
            camera_pos: initial_cam_pos, _padding2: 0.0,
            camera_front: [0.0, 0.0, 1.0], _padding3: 0.0,
        };

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"), size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false,
        });
        queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

        let mut world_data = vec![0u32; WORLD_X * WORLD_Y * WORLD_Z];
        println!("Gerando Terreno na CPU (Aguarde, mundo maior)...");
        for x in 0..WORLD_X {
            for z in 0..WORLD_Z {
                let px = x as f32 * 0.05;
                let pz = z as f32 * 0.05;
                // Deixei o terreno mais acidentado para preencher a tela
                let ground = (px * 0.1).sin() * 15.0 + (pz * 0.1).cos() * 15.0;
                let detail = (px * 2.0).sin() * 2.0 + (pz * 2.0).cos() * 2.0;
                
                let height = (ground + detail + 50.0) as usize; 
                
                for y in 0..WORLD_Y {
                    if y <= height.clamp(0, WORLD_Y - 1) {
                        let index = x + y * WORLD_X + z * WORLD_X * WORLD_Y;
                        world_data[index] = 1; 
                    }
                }
            }
        }

        let world_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("World Storage Buffer"),
            size: (world_data.len() * std::mem::size_of::<u32>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false,
        });
        queue.write_buffer(&world_buffer, 0, bytemuck::cast_slice(&world_data));

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None }
            ], label: Some("bind_group_layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: uniform_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: world_buffer.as_entire_binding() }
            ], label: Some("bind_group"),
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Voxel Shader"), source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"), bind_group_layouts: &[&bind_group_layout], immediate_size: 0,
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"), layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some("vs_main"), compilation_options: Default::default(), buffers: &[] },
            fragment: Some(wgpu::FragmentState {
                module: &shader, entry_point: Some("fs_main"), compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState { format: config.format, blend: Some(wgpu::BlendState::REPLACE), write_mask: wgpu::ColorWrites::ALL })],
            }),
            primitive: wgpu::PrimitiveState { topology: wgpu::PrimitiveTopology::TriangleList, front_face: wgpu::FrontFace::Ccw, cull_mode: None, polygon_mode: wgpu::PolygonMode::Fill, ..Default::default() },
            depth_stencil: None, multisample: wgpu::MultisampleState::default(), multiview_mask: std::num::NonZeroU32::new(0), cache: None,
        });

        Self {
            surface, device, queue, config, size, render_pipeline, window, uniform_buffer, world_buffer, bind_group,
            start_time: std::time::Instant::now(), last_frame_time: std::time::Instant::now(), last_fps_time: std::time::Instant::now(), frame_count: 0,
            sys: System::new_all(), // Inicia o monitor do sistema
            mode: GameMode::God, camera_pos: initial_cam_pos, yaw: std::f32::consts::FRAC_PI_2, pitch: 0.0, keys_pressed: [false; 6],
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size; self.config.width = new_size.width; self.config.height = new_size.height; self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, keycode: KeyCode, pressed: bool) {
        match keycode {
            KeyCode::KeyW => self.keys_pressed[0] = pressed, KeyCode::KeyS => self.keys_pressed[1] = pressed,
            KeyCode::KeyA => self.keys_pressed[2] = pressed, KeyCode::KeyD => self.keys_pressed[3] = pressed,
            KeyCode::Space => self.keys_pressed[4] = pressed, KeyCode::ShiftLeft => self.keys_pressed[5] = pressed,
            KeyCode::KeyG => if pressed { self.mode = match self.mode { GameMode::God => GameMode::Normal, GameMode::Normal => GameMode::God, }; },
            _ => {}
        }
    }

    pub fn mouse_move(&mut self, dx: f64, dy: f64) {
        let sensitivity = 0.003;
        self.yaw -= (dx as f32) * sensitivity; self.pitch -= (dy as f32) * sensitivity;
        self.pitch = self.pitch.clamp(-1.56, 1.56);
    }

    pub fn update(&mut self) {
        let now = std::time::Instant::now(); let dt = (now - self.last_frame_time).as_secs_f32(); self.last_frame_time = now;
        self.frame_count += 1;
        let elapsed_fps = self.last_fps_time.elapsed().as_secs_f32();
        
        // Atualiza a barra de título a cada meio segundo
        if elapsed_fps >= 0.5 {
            let fps = self.frame_count as f32 / elapsed_fps; 
            let ms = 1000.0 / fps; // Tempo de estresse da GPU por frame
            
            self.sys.refresh_cpu_usage(); // Lê os sensores da placa mãe
            let cpu_usage = self.sys.global_cpu_info().cpu_usage();
            
            let mode_str = if self.mode == GameMode::God { "GOD" } else { "NORMAL" };
            self.window.set_title(&format!("Engine | FPS: {:.0} | GPU-Time: {:.1}ms | CPU: {:.1}% | Modo: {}", fps, ms, cpu_usage, mode_str));
            self.frame_count = 0; self.last_fps_time = std::time::Instant::now();
        }

        let speed = 40.0 * dt; 
        let front_x = self.yaw.cos() * self.pitch.cos(); let front_y = self.pitch.sin(); let front_z = self.yaw.sin() * self.pitch.cos();
        let front = [front_x, front_y, front_z];
        let right = [front_z, 0.0, -front_x]; let right_len = (right[0] * right[0] + right[2] * right[2]).sqrt().max(0.001); let right = [right[0] / right_len, 0.0, right[2] / right_len];

        if self.keys_pressed[0] { self.camera_pos[0] += front[0] * speed; if self.mode == GameMode::God { self.camera_pos[1] += front[1] * speed; } self.camera_pos[2] += front[2] * speed; }
        if self.keys_pressed[1] { self.camera_pos[0] -= front[0] * speed; if self.mode == GameMode::God { self.camera_pos[1] -= front[1] * speed; } self.camera_pos[2] -= front[2] * speed; }
        if self.keys_pressed[2] { self.camera_pos[0] -= right[0] * speed; self.camera_pos[2] -= right[2] * speed; }
        if self.keys_pressed[3] { self.camera_pos[0] += right[0] * speed; self.camera_pos[2] += right[2] * speed; }
        if self.mode == GameMode::God { if self.keys_pressed[4] { self.camera_pos[1] += speed; } if self.keys_pressed[5] { self.camera_pos[1] -= speed; } }

        let uniforms = Uniforms {
            resolution: [self.config.width as f32, self.config.height as f32], time: self.start_time.elapsed().as_secs_f32(), _padding1: 0.0,
            camera_pos: self.camera_pos, _padding2: 0.0, camera_front: front, _padding3: 0.0,
        };
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.update(); 
        let output = self.surface.get_current_texture()?; let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Render") });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Pass"), color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store }, depth_slice: None })],
                depth_stencil_attachment: None, occlusion_query_set: None, timestamp_writes: None, multiview_mask: std::num::NonZeroU32::new(0),
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]); render_pass.draw(0..3, 0..1);
        }
        self.queue.submit(std::iter::once(encoder.finish())); output.present(); Ok(())
    }
}