pub mod arsenal;

use crate::game::player::{Player, WeaponType};

pub fn update_combat(
    player: &mut Player,
    queue: &wgpu::Queue,
    world_buffer: &wgpu::Buffer,
    dt: f32
) {
    if player.cooldown > 0.0 { player.cooldown -= dt; }
    if !player.is_shooting { return; }

    match player.active_weapon {
        WeaponType::Creator => arsenal::creator::fire(player, queue, world_buffer),
        WeaponType::Plasma => arsenal::plasma::fire(player, queue, world_buffer),
        WeaponType::Bazooka => arsenal::bazooka::fire(player, queue, world_buffer),
        WeaponType::None => { /* Mãos vazias, não faz nada */ }
    }
}

pub fn simple_raycast(player: &Player, max_dist: f32) -> Option<[f32; 3]> {
    let mut curr = player.camera.pos;
    let dir = player.camera.get_front();
    for _ in 0..(max_dist as i32) {
        curr[0] += dir[0]; curr[1] += dir[1]; curr[2] += dir[2];
        if crate::game::player::physics::is_colliding(player, curr) {
            return Some(curr);
        }
    }
    None
}