use crate::game::player::Player;
use crate::engine::voxel::synchronization::WorldSync;

pub fn fire(player: &mut Player, queue: &wgpu::Queue, buffer: &wgpu::Buffer) {
    let front = player.camera.get_front();
    for i in 2..40 {
        let p = [
            player.camera.pos[0] + front[0]*i as f32, 
            player.camera.pos[1] + front[1]*i as f32, 
            player.camera.pos[2] + front[2]*i as f32
        ];
        WorldSync::modify_sphere(queue, buffer, &mut player.world_edits, p, 1.5, 0);
    }
    player.cooldown = 0.05;
}