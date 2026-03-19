use crate::engine::graphics::device::GraphicsDevice;
use crate::engine::input::InputController;
use crate::engine::time::TimeState;
use crate::engine::voxel::storage::VoxelStorage;
use crate::game::combat::update_combat;
use crate::game::player::Player;
use crate::game::player::controller::handle_input;
use crate::game::player::physics::update_player;
use crate::game::world_gen::WorldGenerator;
use std::sync::Arc;
use winit::window::Window;

pub struct State<'a> {
    pub device: GraphicsDevice<'a>,
    pub storage: VoxelStorage,
    pub window: Arc<Window>,
    pub player: Player,
    pub inputs: InputController,
    pub time: TimeState,
    pub render_pipeline: wgpu::RenderPipeline,
    pub physics_pipeline: wgpu::ComputePipeline,
    pub gen_pipeline: wgpu::ComputePipeline,
    pub bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
}

impl<'a> State<'a> {
    pub async fn new(window: Arc<Window>) -> Self {
        let device = GraphicsDevice::new(window.clone()).await;
        let storage = VoxelStorage::new(&device.device);
        let player = Player::new([128.0, 200.0, 128.0]);
        let inputs = InputController::new();
        let time = TimeState::new();

        // --- CARREGAMENTO E CONCATENAÇÃO DE SHADERS ---
        // (Simula o #include lendo os arquivos do disco)
        let constants = include_str!("../shaders/include/constant.wgsl");
        let structs = include_str!("../shaders/include/struct.wgsl");
        let math = include_str!("../shaders/include/math.wgsl");
        let biome_lib = include_str!("../shaders/pcg/biomes.wgsl");
        let vertex = include_str!("../shaders/include/vertex.wgsl");

        let render_src = format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            constants,
            structs,
            biome_lib,
            math,
            vertex,
            include_str!("../shaders/raytracer/dda.wgsl"),
            include_str!("../shaders/raytracer/lighting.wgsl"), // Faltava isso na sua refatoração!!
            include_str!("../shaders/post_process/psx.wgsl"),
            include_str!("../shaders/render_main.wgsl")
        );

        let physics_src = format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            constants,
            structs,
            biome_lib,
            math,
            include_str!("../shaders/physics/sand.wgsl"),
            include_str!("../shaders/physics/fluids.wgsl"),
            include_str!("../shaders/physics/dirt.wgsl"),
            include_str!("../shaders/physics/physics_main.wgsl") // Dispatcher
        );

        let gen_src = format!(
            "{}\n{}\n{}\n{}\n{}",
            constants,
            structs,
            math,
            biome_lib, // Bioma deve vir antes do planet_gen
            include_str!("../shaders/pcg/planet_gen.wgsl")
        );

        // --- UNIFORM SETUP ---
        let uniform_buffer = device.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: 128, // Suficiente para nossa struct Uniforms
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // --- BIND GROUP ---
        let bind_group_layout =
            device
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        // Uniforms: Todos enxergam
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::VERTEX
                                | wgpu::ShaderStages::FRAGMENT
                                | wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        // World e Macro: Apenas Fragment (Raytracing) e Compute (Física)
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 3,
                            visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                    label: Some("Main Bind Group Layout"),
                });

        let bind_group = device.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: storage.world_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: storage.macro_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: storage.projectile_buffer.as_entire_binding(),
                },
            ],
            label: None,
        });

        // --- PIPELINES ---
        let pipeline_layout =
            device
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts: &[&bind_group_layout],
                    ..Default::default()
                });

        let render_mod = device
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(render_src.into()),
            });
        let render_pipeline =
            device
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &render_mod,
                        entry_point: Some("vs_main"),
                        compilation_options: Default::default(),
                        buffers: &[],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &render_mod,
                        entry_point: Some("fs_main"),
                        compilation_options: Default::default(),
                        targets: &[Some(device.config.format.into())],
                    }),
                    primitive: wgpu::PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState::default(),
                    multiview_mask: None,
                    cache: None,
                });

        let physics_pipeline =
            crate::engine::graphics::pipeline_builder::PipelineBuilder::create_compute_pipeline(
                &device.device,
                &pipeline_layout,
                &physics_src,
                "cp_main",
            );
        let gen_pipeline =
            crate::engine::graphics::pipeline_builder::PipelineBuilder::create_compute_pipeline(
                &device.device,
                &pipeline_layout,
                &gen_src,
                "main_gen",
            );

        // GERAÇÃO INICIAL
        let world_gen = WorldGenerator::new(42);
        world_gen.dispatch_initial_gen(&device.device, &device.queue, &gen_pipeline, &bind_group);

        Self {
            window,
            device,
            storage,
            player,
            inputs,
            time,
            render_pipeline,
            physics_pipeline,
            gen_pipeline,
            bind_group,
            uniform_buffer,
        }
    }

    pub fn update(&mut self) {
        self.time.update();

        // 1. APLICAR MOUSE NA CÂMERA (O elo perdido!)
        // Pegamos o delta acumulado, aplicamos e resetamos para o próximo frame
        let m_delta = self.inputs.mouse.delta;
        self.player.camera.mouse_move(m_delta.0, m_delta.1);
        self.inputs.mouse.delta = (0.0, 0.0);

        // 2. PROCESSAR TECLADO E ARSENAL
        // Passamos apenas a parte do teclado para as funções que já existem
        handle_input(&mut self.player, &self.inputs.keyboard);
        update_player(&mut self.player, &self.inputs.keyboard, self.time.delta);

        // 3. COMBATE E FÍSICA
        update_combat(
            &mut self.player,
            &self.device.queue,
            &self.storage.world_buffer,
            self.time.delta,
        );

        // Estrutura de Uniforms exata para o Shader (com padding para 16 bytes de alinhamento)
        #[repr(C)]
        #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
        struct UniformData {
            res: [f32; 2],
            time: f32,
            action: u32,
            cam_pos: [f32; 3],
            flash: u32,
            cam_front: [f32; 3],
            pad1: f32,
            cam_up: [f32; 3],
            pad2: f32,
        }

        // Transição do Dia/Noite suave baseada em física pura:
        let target_time = if self.player.is_day {
            0.0
        } else {
            std::f32::consts::PI
        };
        self.time.time_of_day += (target_time - self.time.time_of_day) * self.time.delta * 4.0;

        let data = UniformData {
            res: [
                self.device.size.width as f32,
                self.device.size.height as f32,
            ],
            time: self.time.time_of_day, // Usa o suave agora
            action: self.player.active_weapon as u32,
            cam_pos: self.player.camera.pos,
            flash: if self.player.flashlight { 1 } else { 0 },
            cam_front: self.player.camera.get_front(),
            pad1: 0.0,
            cam_up: self.player.visual_up,
            pad2: 0.0,
        };
        
        self.device
            .queue
            .write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[data]));

        // Envia os Mísseis da CPU para a Placa de Vídeo desenhar
        let mut proj_data = [crate::engine::voxel::storage::Projectile {
            pos: [0.0; 3],
            is_active: 0,
            vel: [0.0; 3],
            p_type: 0,
            mat_id: 0,
            pad1: 0,
            pad2: 0,
            pad3: 0,
        }; 64];
        for (i, p) in self.player.active_projectiles.iter().enumerate().take(64) {
            proj_data[i] = *p;
        }
        self.device.queue.write_buffer(
            &self.storage.projectile_buffer,
            0,
            bytemuck::cast_slice(&proj_data),
        );

        // Limpa os estados de 'Pressionado' que causam flutuações e pulos infinitos
        self.inputs.keyboard.tick();

    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.device.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            cpass.set_pipeline(&self.physics_pipeline);
            cpass.set_bind_group(0, &self.bind_group, &[]);
            cpass.dispatch_workgroups(64, 64, 64);
        }

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                ..Default::default()
            });
            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.draw(0..3, 0..1);
        }

        self.device.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}
