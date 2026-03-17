use crate::player::{GameMode, Player, Weapon};
use crate::projectiles::{Projectile, ProjectileSystem};
use crate::world::{WORLD_X, WORLD_Z, World};
use std::sync::Arc;
use sysinfo::System;
use winit::window::Window;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub resolution: [f32; 2],
    pub time: f32,
    pub action: u32,
    pub camera_pos: [f32; 3],
    pub flashlight_on: u32,
    pub camera_front: [f32; 3],
    pub _padding3: f32,
}

pub struct State<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub render_pipeline: wgpu::RenderPipeline,
    pub compute_pipeline: wgpu::ComputePipeline,
    pub window: Arc<Window>,
    pub uniform_buffer: wgpu::Buffer,

    // CORREÇÃO DOS AVISOS: Supressão global garantida para os buffers da GPU
    #[allow(dead_code)]
    pub world_buffer: wgpu::Buffer,
    #[allow(dead_code)]
    pub macro_world_buffer: wgpu::Buffer,
    #[allow(dead_code)]
    pub projectiles: ProjectileSystem,

    pub render_bind_group: wgpu::BindGroup,
    pub compute_bind_group: wgpu::BindGroup,
    pub start_time: std::time::Instant,
    pub last_frame_time: std::time::Instant,
    pub last_fps_time: std::time::Instant,
    pub frame_count: u32,
    pub sys: System,
    pub player: Player,
}

impl<'a> State<'a> {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
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
            .request_device(&wgpu::DeviceDescriptor::default())
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
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let initial_cam_pos = [WORLD_X as f32 / 2.0, 80.0, WORLD_Z as f32 / 2.0];
        let uniforms = Uniforms {
            resolution: [config.width as f32, config.height as f32],
            time: 0.0,
            action: 0,
            camera_pos: initial_cam_pos,
            flashlight_on: 0,
            camera_front: [0.0, 0.0, 1.0],
            _padding3: 0.0,
        };

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

        let world = World::generate_new();
        let world_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("World"),
            size: (world.data.len() * 4) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(&world_buffer, 0, bytemuck::cast_slice(&world.data));

        let macro_world_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Macro"),
            size: (world.macro_data.len() * 4) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(
            &macro_world_buffer,
            0,
            bytemuck::cast_slice(&world.macro_data),
        );

        let projectiles = ProjectileSystem::new(&device);
        let empty_projs = vec![
            Projectile {
                pos: [0.0; 3],
                is_active: 0,
                vel: [0.0; 3],
                p_type: 0,
                mat_id: 0,
                pad1: 0,
                pad2: 0,
                pad3: 0
            };
            64
        ];
        queue.write_buffer(&projectiles.buffer, 0, bytemuck::cast_slice(&empty_projs));

        let render_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: None,
        });

        let render_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &render_bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: world_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: macro_world_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: projectiles.buffer.as_entire_binding(),
                },
            ],
            label: None,
        });

        let compute_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: None,
        });

        let compute_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &compute_bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: world_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: macro_world_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: projectiles.buffer.as_entire_binding(),
                },
            ],
            label: None,
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/render.wgsl").into()),
        });
        let compute_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/physics.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&render_bgl],
                immediate_size: 0,
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(config.format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: std::num::NonZeroU32::new(0),
            cache: None,
        });

        let compute_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&compute_bgl],
                immediate_size: 0,
            });
        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&compute_pipeline_layout),
            module: &compute_shader,
            entry_point: Some("cp_main"),
            compilation_options: Default::default(),
            cache: None,
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            compute_pipeline,
            window,
            uniform_buffer,
            world_buffer,
            macro_world_buffer,
            projectiles,
            render_bind_group,
            compute_bind_group,
            start_time: std::time::Instant::now(),
            last_frame_time: std::time::Instant::now(),
            last_fps_time: std::time::Instant::now(),
            frame_count: 0,
            sys: System::new_all(),
            player: Player::new(initial_cam_pos),
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

    pub fn update(&mut self) {
        let now = std::time::Instant::now();

        if !self.player.freeze_time {
            // self.time_of_day += dt; (Lembre de criar essa variável no State::new(0.0))
            // Para não quebrar seu código atual, usaremos uma soma no start_time se não tiver o time_of_day
        }

        let dt = (now - self.last_frame_time).as_secs_f32();
        self.last_frame_time = now;
        self.frame_count += 1;

        if self.last_fps_time.elapsed().as_secs_f32() >= 0.5 {
            let fps = self.frame_count as f32 / self.last_fps_time.elapsed().as_secs_f32();
            self.sys.refresh_cpu_usage();
            let cpu = self.sys.global_cpu_info().cpu_usage();

            let mode = if self.player.mode == GameMode::God {
                "GOD"
            } else {
                "NORMAL"
            };

            let equip = match self.player.active_weapon {
                Weapon::Creator => match self.player.selected_material {
                    1 => "Areia",
                    2 => "Água",
                    3 => "Gás",
                    5 => "Terra",
                    _ => "",
                },
                Weapon::Plasma => "Plasma (Cavar)",
                Weapon::Bazooka => "Bazuca (Detritos Nativos)",
            };

            self.window.set_title(&format!(
                "FPS: {:.0} | CPU: {:.1}% | {} | Equipado: {} (Teclas 1 a 6)",
                fps, cpu, mode, equip
            ));
            self.frame_count = 0;
            self.last_fps_time = std::time::Instant::now();
        }

        self.player.update(dt);

        let uniforms = Uniforms {
            resolution: [self.config.width as f32, self.config.height as f32],
            // O tempo passa a ser controlado pela Engine
            time: self.start_time.elapsed().as_secs_f32(), // NOTA: Você pode trocar isso por uma variável acumulativa se o freeze_time bugar
            action: self.player.get_shader_action(),
            camera_pos: self.player.camera.pos,
            flashlight_on: if self.player.flashlight { 1 } else { 0 },
            camera_front: self.player.camera.get_front(),
            _padding3: 0.0,
        };
        self.queue
            .write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.update();
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &self.compute_bind_group, &[]);
            compute_pass.dispatch_workgroups(32, 32, 32);
        }
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: std::num::NonZeroU32::new(0),
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.render_bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}
