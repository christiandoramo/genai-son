fn simulate_gas(idx: u32, pos: vec3<u32>, seed: u32) {
    let side = get_orthogonal(g_dir, seed % 4u);
    let s_pos = vec3<u32>(vec3<i32>(pos) + side);
    
    let dist_to_core = length(vec3<f32>(pos) - vec3<f32>(128.0));
    if dist_to_core > 100.0 { world.data[idx] = MAT_AIR; return; }

    let g_dir = get_planet_gravity_dir(vec3<f32>(pos));
    let up = vec3<u32>(vec3<i32>(pos) - g_dir); // Sobe contra o centro

    if !is_valid(up.x, up.y, up.z) { return; }

    if world.data[get_index(up.x, up.y, up.z)] == MAT_AIR {
        move_voxel(idx, get_index(up.x, up.y, up.z), up.x, up.y, up.z, MAT_GAS);
    } else {
        // Se bater no teto, espalha lateralmente para preencher cavernas
        let side = get_random_side(pos, seed);
        if is_valid(side.x, side.y, side.z) && world.data[get_index(side.x, side.y, side.z)] == MAT_AIR {
            move_voxel(idx, get_index(side.x, side.y, side.z), side.x, side.y, side.z, MAT_GAS);
        }
    }
}