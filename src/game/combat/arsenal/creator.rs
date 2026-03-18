use crate::game::player::Player;
use crate::engine::voxel::synchronization::WorldSync;
use crate::engine::voxel::material::*;

pub fn fire(player: &mut Player, queue: &wgpu::Queue, buffer: &wgpu::Buffer) {
    // Pinta num raio de 3 blocos à uma distância de 10 blocos da mira
    let front = player.camera.get_front();
    let target = [
        player.camera.pos[0] + front[0] * 10.0,
        player.camera.pos[1] + front[1] * 10.0,
        player.camera.pos[2] + front[2] * 10.0,
    ];

    // Só gasta cooldown e modifica se houver um material selecionado
    if player.selected_material != VOXEL_AIR {
        WorldSync::modify_sphere(
            queue, 
            buffer, 
            &mut player.world_edits, 
            target, 
            3.0, 
            player.selected_material
        );
        player.cooldown = 0.1;
    }
}