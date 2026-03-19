pub mod arsenal;

use crate::game::player::{Player, WeaponType};

pub fn update_combat(
    player: &mut Player,
    queue: &wgpu::Queue,
    world_buffer: &wgpu::Buffer,
    dt: f32,
) {
    if player.cooldown > 0.0 {
        player.cooldown -= dt;
    }

    // Usando índices para evitar o bloqueio (Borrow Checker) do 'player'
    let len = player.active_projectiles.len();
    for i in 0..len {
        let mut p = player.active_projectiles[i]; // Faz uma cópia da struct
        
        if p.is_active == 1 {
            p.pos[0] += p.vel[0];
            p.pos[1] += p.vel[1];
            p.pos[2] += p.vel[2];
            
            // Atualiza de volta no vetor (Libera o player)
            player.active_projectiles[i] = p; 

            // Como não estamos travando o loop com &mut, o player tá livre pra checar colisão!
            if crate::game::player::physics::is_colliding(player, p.pos) {
                player.active_projectiles[i].is_active = 0;
                crate::engine::voxel::synchronization::WorldSync::modify_sphere(
                    queue,
                    world_buffer,
                    &mut player.world_edits,
                    p.pos,
                    6.0,
                    0,
                );
            }
        }
    }
    
    // Limpa os mísseis que já explodiram da memória
    player.active_projectiles.retain(|p| p.is_active == 1);

    if !player.is_shooting {
        return;
    }

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
        curr[0] += dir[0];
        curr[1] += dir[1];
        curr[2] += dir[2];
        if crate::game::player::physics::is_colliding(player, curr) {
            return Some(curr);
        }
    }
    None
}