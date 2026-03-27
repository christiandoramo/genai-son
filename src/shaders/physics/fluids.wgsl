#define_import_path physics_fluids
#import globals::{world}
#import constants::{MAT_AIR, MAT_GAS}
#import math::{get_planet_gravity_dir, is_valid, get_index, move_voxel, is_free}

// SIMULAÇÃO DE LÍQUIDOS (Água e Magma)
fn simulate_liquid(idx: u32, pos: vec3<u32>, seed: u32, mat_id: u32, viscosity: u32) {
    if (seed % viscosity != 0u) { return; }

    let f_pos = vec3<f32>(pos);
    let g_dir = get_planet_gravity_dir(f_pos);
    let down = vec3<u32>(vec3<i32>(pos) + g_dir);

    if (!is_valid(down.x, down.y, down.z)) { return; }

    // 1. Queda Livre
    let voxel_down = world.data[get_index(down.x, down.y, down.z)];
    if (is_free(voxel_down)) {
        move_voxel(idx, get_index(down.x, down.y, down.z), down.x, down.y, down.z, mat_id);
        return;
    }

    // 2. Tensão Superficial / Coesão
    // O líquido olha para os lados. Se houver o mesmo líquido por perto, 
    // ele prefere se mover para "grudar" ou preencher buracos próximos.
    let side_offsets = array<vec3<i32>, 4>(
        vec3<i32>(1, 0, 0), vec3<i32>(-1, 0, 0), 
        vec3<i32>(0, 0, 1), vec3<i32>(0, 0, -1)
    );
    
    // Tenta se espalhar lateralmente respeitando a gravidade local
    let start_idx = seed % 4u;
    for (var i = 0u; i < 4u; i++) {
        let offset = side_offsets[(start_idx + i) % 4u];
        let side = vec3<u32>(vec3<i32>(pos) + offset);
        
        if (is_valid(side.x, side.y, side.z)) {
            let s_idx = get_index(side.x, side.y, side.z);
            if (is_free(world.data[s_idx])) {
                // Checa se abaixo do lado também está livre (Cachoeira)
                let side_down = vec3<u32>(vec3<i32>(side) + g_dir);
                if (is_valid(side_down.x, side_down.y, side_down.z) && is_free(world.data[get_index(side_down.x, side_down.y, side_down.z)])) {
                    move_voxel(idx, s_idx, side.x, side.y, side.z, mat_id);
                    return;
                }
                // Se não é cachoeira, move-se apenas para manter o nível (Tensão Superficial)
                move_voxel(idx, s_idx, side.x, side.y, side.z, mat_id);
                return;
            }
        }
    }
}

// SIMULAÇÃO DE GÁS (Sobe contra a gravidade)
fn simulate_gas(idx: u32, pos: vec3<u32>, seed: u32) {
    let f_pos = vec3<f32>(pos);
    let g_dir = get_planet_gravity_dir(f_pos);
    let up = vec3<u32>(vec3<i32>(pos) - g_dir); // Gás vai para -Gravidade

    if (!is_valid(up.x, up.y, up.z)) { 
        world.data[idx] = MAT_AIR; // Dissipa no vácuo
        return; 
    }

    let voxel_up = world.data[get_index(up.x, up.y, up.z)];
    if (voxel_up == MAT_AIR) {
        move_voxel(idx, get_index(up.x, up.y, up.z), up.x, up.y, up.z, MAT_GAS);
    } else {
        // Gás se espalha pelo teto (como água invertida)
        let offset = vec3<i32>(select(-1, 1, (seed % 2u) == 0u), 0, select(-1, 1, (seed / 2u % 2u) == 0u));
        let side = vec3<u32>(vec3<i32>(pos) + offset);
        if (is_valid(side.x, side.y, side.z) && world.data[get_index(side.x, side.y, side.z)] == MAT_AIR) {
            move_voxel(idx, get_index(side.x, side.y, side.z), side.x, side.y, side.z, MAT_GAS);
        }
    }
}