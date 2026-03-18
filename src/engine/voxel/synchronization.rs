use std::collections::HashMap;
use crate::engine::voxel::material::WORLD_SIZE;

pub struct WorldSync;

impl WorldSync {
    pub fn update_voxel(
        queue: &wgpu::Queue,
        buffer: &wgpu::Buffer,
        edits: &mut HashMap<[i32; 3], u32>,
        pos: [i32; 3],
        material: u32
    ) {
        if pos[0] < 0 || pos[0] >= WORLD_SIZE as i32 || pos[1] < 0 || pos[1] >= WORLD_SIZE as i32 || pos[2] < 0 || pos[2] >= WORLD_SIZE as i32 {
            return;
        }

        // 1. Atualiza a verdade da CPU (Colisão)
        edits.insert(pos, material);

        // 2. Escreve direto na VRAM da GPU (Visual)
        let offset = (pos[0] as u32 + (pos[1] as u32 * WORLD_SIZE) + (pos[2] as u32 * WORLD_SIZE * WORLD_SIZE)) * 4;
        queue.write_buffer(buffer, offset as u64, bytemuck::cast_slice(&[material]));
    }

    pub fn modify_sphere(
        queue: &wgpu::Queue,
        buffer: &wgpu::Buffer,
        edits: &mut HashMap<[i32; 3], u32>,
        center: [f32; 3],
        radius: f32,
        material: u32
    ) {
        let r_int = radius.ceil() as i32;
        for x in -r_int..=r_int {
            for y in -r_int..=r_int {
                for z in -r_int..=r_int {
                    let dist_sq = (x*x + y*y + z*z) as f32;
                    if dist_sq <= radius * radius {
                        let vx = center[0].round() as i32 + x;
                        let vy = center[1].round() as i32 + y;
                        let vz = center[2].round() as i32 + z;
                        Self::update_voxel(queue, buffer, edits, [vx, vy, vz], material);
                    }
                }
            }
        }
    }
}