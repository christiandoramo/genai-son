fn simulate_dirt(idx: u32, pos: vec3<u32>, seed: u32) {
    let g_dir = get_planet_gravity_dir(vec3<f32>(pos));
    let down = vec3<u32>(vec3<i32>(pos) + g_dir);

    if (!is_valid(down.x, down.y, down.z)) { return; }

    // Terra só cai se o bloco abaixo for Ar ou Gás (não desliza lateralmente como areia)
    let voxel_down = world.data[get_index(down.x, down.y, down.z)];
    if (voxel_down == MAT_AIR || voxel_down == MAT_GAS) {
        move_voxel(idx, get_index(down.x, down.y, down.z), down.x, down.y, down.z, MAT_DIRT);
    }
}