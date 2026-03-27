#define_import_path physics_gas
#import globals::{world}
#import constants::{MAT_AIR, MAT_GAS}
#import math::{get_planet_gravity_dir, is_valid, get_index, get_orthogonal, move_voxel}

fn simulate_gas(idx: u32, pos: vec3<u32>, seed: u32) {
    let dist_to_core = length(vec3<f32>(pos) - vec3<f32>(128.0));
    if dist_to_core > 100.0 { world.data[idx] = MAT_AIR; return; }

    let g_dir = get_planet_gravity_dir(vec3<f32>(pos));
    let up = vec3<u32>(vec3<i32>(pos) - g_dir); // Sobe contra o centro

    if !is_valid(up.x, up.y, up.z) { return; }

    if world.data[get_index(up.x, up.y, up.z)] == MAT_AIR {
        move_voxel(idx, get_index(up.x, up.y, up.z), up.x, up.y, up.z, MAT_GAS);
    } else {
        // Se bater no teto, espalha lateralmente para preencher cavernas
        let side = get_orthogonal(g_dir, seed % 4u);
        let s_pos = vec3<u32>(vec3<i32>(pos) + side);
        
        if is_valid(s_pos.x, s_pos.y, s_pos.z) && world.data[get_index(s_pos.x, s_pos.y, s_pos.z)] == MAT_AIR {
            move_voxel(idx, get_index(s_pos.x, s_pos.y, s_pos.z), s_pos.x, s_pos.y, s_pos.z, MAT_GAS);
        }
    }
}