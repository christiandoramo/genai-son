use std::sync::Arc;
use sysinfo::System;
use winit::window::Window;

use crate::entities::player::{GameMode, Player, Weapon};
use crate::entities::projectiles::{Projectile, ProjectileSystem};
use crate::graphics::buffers::Uniforms;
use crate::graphics::core::GpuCore;
use crate::graphics::pipelines::GpuPipelines;

// REMOVIDO: importações do crate::world antigas. Elas não existem mais!

pub struct State<'a> {
    pub gpu: GpuCore<'a>,
    pub pipelines: GpuPipelines,
    pub window: Arc<Window>,

    pub uniform_buffer: wgpu::Buffer,
    #[allow(dead_code)]
    pub world_buffer: wgpu::Buffer,
    #[allow(dead_code)]
    pub macro_world_buffer: wgpu::Buffer,
    #[allow(dead_code)]
    pub projectiles: ProjectileSystem,

    pub last_frame_time: std::time::Instant,
    pub last_fps_time: std::time::Instant,
    pub frame_count: u32,
    pub sys: System,
    pub player: Player,
    pub time_of_day: f32,
}

impl<'a> State<'a> {
    pub async fn new(window: Arc<Window>) -> Self {
        let gpu = GpuCore::new(window.clone()).await;

        let initial_cam_pos = [128.0, 220.0, 128.0];

        let uniforms = Uniforms {
            resolution: [gpu.config.width as f32, gpu.config.height as f32],
            time: 0.0,
            action: 0,
            camera_pos: initial_cam_pos,
            flashlight_on: 0,
            camera_front: [0.0, 0.0, 1.0],
            _padding3: 0.0,
        };

        let uniform_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        gpu.queue
            .write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

        // Buffer do Mundo ultraleve (64MB)
        let world_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("World"),
            size: (256 * 256 * 256 * 4) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let macro_world_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Macro"),
            size: ((256 / 8) * (256 / 8) * (256 / 8) * 4) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let projectiles = ProjectileSystem::new(&gpu.device);
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
        gpu.queue
            .write_buffer(&projectiles.buffer, 0, bytemuck::cast_slice(&empty_projs));

        let pipelines = GpuPipelines::new(
            &gpu.device,
            gpu.config.format,
            &uniform_buffer,
            &world_buffer,
            &macro_world_buffer,
            &projectiles.buffer,
        );

        // ====================================================================
        // A MÁGICA FOI MOVIDA PARA CÁ: FORJANDO O PLANETA APENAS 1 VEZ
        // ====================================================================
        let mut encoder = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Gen Encoder"),
            });
        {
            let mut c_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            c_pass.set_pipeline(&pipelines.world_gen_pipeline);
            c_pass.set_bind_group(0, &pipelines.compute_bind_group, &[]);
            // Dispara para 256x256x256 = Workgroups de (64, 64, 64) porque 64 * 4 = 256!
            c_pass.dispatch_workgroups(64, 64, 64); // <--- MUDANÇA AQUI
        }
        gpu.queue.submit(std::iter::once(encoder.finish()));

        // ====================================================================

        Self {
            gpu,
            pipelines,
            window,
            uniform_buffer,
            world_buffer,
            macro_world_buffer,
            projectiles,
            last_frame_time: std::time::Instant::now(),
            last_fps_time: std::time::Instant::now(),
            frame_count: 0,
            sys: System::new_all(),
            player: Player::new(initial_cam_pos),
            time_of_day: 0.0,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.gpu.resize(new_size);
    }

    pub fn update(&mut self) {
        let now = std::time::Instant::now();
        let mut dt = (now - self.last_frame_time).as_secs_f32();
        self.last_frame_time = now;
        self.frame_count += 1;

        // O FREIO QUÂNTICO
        if dt > 0.05 {
            dt = 0.05;
        }

        // O SISTEMA DE ILUMINAÇÃO (Dia e Noite como Toggle)
        // 0.0 = Sol no Topo (Dia) | 3.14 (PI) = Sol embaixo (Noite Escura)
        let target_time = if self.player.is_day {
            0.0
        } else {
            std::f32::consts::PI
        };

        // Interpola suavemente. A transição dura cerca de 1 segundo.
        self.time_of_day += (target_time - self.time_of_day) * dt * 4.0;

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
            resolution: [self.gpu.config.width as f32, self.gpu.config.height as f32],
            time: self.time_of_day,
            action: self.player.get_shader_action(),
            camera_pos: self.player.camera.pos,
            flashlight_on: if self.player.flashlight { 1 } else { 0 },
            camera_front: self.player.camera.get_front(),
            _padding3: 0.0,
        };
        self.gpu
            .queue
            .write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.update();
        let output = self.gpu.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // PASSE 1: RODA A FÍSICA E AS ARMAS (Compute Shader)
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.pipelines.compute_pipeline);
            compute_pass.set_bind_group(0, &self.pipelines.compute_bind_group, &[]);
            compute_pass.dispatch_workgroups(32, 32, 32);
        }

        // PASSE 2: DESENHA OS VOXELS NA TELA (Fragment Shader de Raytracing)
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
            render_pass.set_pipeline(&self.pipelines.render_pipeline);
            render_pass.set_bind_group(0, &self.pipelines.render_bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        self.gpu.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}
