use crate::game::player::Player;
use crate::engine::voxel::synchronization::WorldSync;
use crate::game::combat::simple_raycast;

pub fn fire(player: &mut Player, queue: &wgpu::Queue, buffer: &wgpu::Buffer) {
    if player.cooldown <= 0.0 {
        if let Some(hit) = simple_raycast(player, 100.0) {
            // Explosão: Fura CPU e GPU
            WorldSync::modify_sphere(queue, buffer, &mut player.world_edits, hit, 6.0, 0);
        }
        player.cooldown = 0.5;
    }
}