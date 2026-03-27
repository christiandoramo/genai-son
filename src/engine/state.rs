use crate::engine::graphics::device::GraphicsDevice;
use crate::engine::input::InputController;
use crate::engine::time::TimeState;
use crate::engine::voxel::storage::VoxelStorage;
use crate::game::combat::update_combat;
use crate::game::player::Player;
use crate::game::player::controller::handle_input;
use crate::game::player::physics::update_player;
use crate::game::world_gen::WorldGenerator;
use naga_oil::compose::{
    ComposableModuleDescriptor, Composer, NagaModuleDescriptor, ShaderLanguage,
};
use std::sync::Arc;
use winit::window::Window;

// 1. A FUNÇÃO MÁGICA: Converte a inteligência do naga_oil em WGSL puro que o wgpu 28.0 aceita
fn compile_naga_to_wgsl(module: &wgpu::naga::Module) -> String {
    let info = wgpu::naga::valid::Validator::new(
        wgpu::naga::valid::ValidationFlags::all(),
        wgpu::naga::valid::Capabilities::all(),
    ).validate(module).expect("Erro crítico de validação matemática no naga_oil");

    wgpu::naga::back::wgsl::write_string(
        module,
        &info,
        wgpu::naga::back::wgsl::WriterFlags::empty()
    ).expect("Falha ao converter o módulo Naga de volta para WGSL")
}
pub struct State<'a> {
    // ... (suas definições de struct continuam iguais)
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
        // ... (sua inicialização do device, storage, player, etc continua igual)
        let device = GraphicsDevice::new(window.clone()).await;
        let storage = VoxelStorage::new(&device.device);
        let player = Player::new([128.0, 220.0, 128.0]);
        let inputs = InputController::new();
        let time = TimeState::new();

        let mut composer = Composer::default();

        let modules = vec![
            ("constants", include_str!("../shaders/include/constant.wgsl")),
            ("structs", include_str!("../shaders/include/struct.wgsl")),
            ("globals", include_str!("../shaders/include/globals.wgsl")),
            ("biomes", include_str!("../shaders/pcg/biomes.wgsl")),
            ("math", include_str!("../shaders/include/math.wgsl")),
            ("dda", include_str!("../shaders/raytracer/dda.wgsl")),
            ("lighting", include_str!("../shaders/raytracer/lighting.wgsl")),
            ("psx", include_str!("../shaders/post_process/psx.wgsl")),
            ("physics_sand", include_str!("../shaders/physics/sand.wgsl")),
            ("physics_fluids", include_str!("../shaders/physics/fluids.wgsl")),
            ("physics_dirt", include_str!("../shaders/physics/dirt.wgsl")),
            ("physics_gas", include_str!("../shaders/physics/gas.wgsl")),
        ];

        for (name, source) in modules {
            composer
                .add_composable_module(ComposableModuleDescriptor {
                    source,
                    file_path: name,
                    language: ShaderLanguage::Wgsl,
                    as_name: None,
                    ..Default::default()
                })
                .expect(&format!("Erro de sintaxe no módulo WGSL: {}", name));
        }

        let render_naga = composer
            .make_naga_module(NagaModuleDescriptor {
                source: include_str!("../shaders/render_main.wgsl"),
                file_path: "render_main.wgsl",
                ..Default::default()
            })
            .unwrap();

        let physics_naga = composer
            .make_naga_module(NagaModuleDescriptor {
                source: include_str!("../shaders/physics/physics_main.wgsl"),
                file_path: "physics_main.wgsl",
                ..Default::default()
            })
            .unwrap();

        let gen_naga = composer
            .make_naga_module(NagaModuleDescriptor {
                source: include_str!("../shaders/pcg/planet_gen.wgsl"),
                file_path: "planet_gen.wgsl",
                ..Default::default()
            })
            .unwrap();

        // 2. A MUDANÇA PRINCIPAL: Usando a função para injetar a string pura validada
        let render_mod = device.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Render"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Owned(compile_naga_to_wgsl(&render_naga))),
        });
        
        let physics_mod = device.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Physics"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Owned(compile_naga_to_wgsl(&physics_naga))),
        });
        
        let gen_mod = device.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Gen"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Owned(compile_naga_to_wgsl(&gen_naga))),
        });

        // ... O RESTO DO SEU CÓDIGO (buffers, bind_group_layout, pipelines, etc) FICA EXATAMENTE IGUAL!

        let uniform_buffer = device.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: 128,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout =
            device
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[
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

        let pipeline_layout =
            device
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts: &[&bind_group_layout],
                    ..Default::default()
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
                &physics_mod,
                "cp_main",
            );
        let gen_pipeline =
            crate::engine::graphics::pipeline_builder::PipelineBuilder::create_compute_pipeline(
                &device.device,
                &pipeline_layout,
                &gen_mod,
                "main_gen",
            );

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

        let m_delta = self.inputs.mouse.delta;
        self.player.camera.mouse_move(m_delta.0, m_delta.1);
        self.inputs.mouse.delta = (0.0, 0.0);

        handle_input(&mut self.player, &self.inputs.keyboard);
        update_player(&mut self.player, &self.inputs.keyboard, self.time.delta);
        update_combat(
            &mut self.player,
            &self.device.queue,
            &self.storage.world_buffer,
            self.time.delta,
        );

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
            time: self.time.time_of_day,
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
