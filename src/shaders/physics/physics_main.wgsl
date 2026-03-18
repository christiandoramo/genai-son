@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read_write> world: WorldBuffer;
@group(0) @binding(2) var<storage, read_write> macro_world: WorldBuffer;

@compute @workgroup_size(4, 4, 4)
fn cp_main(@builtin(global_invocation_id) id: vec3<u32>) {
    if (!is_valid(id.x, id.y, id.z)) { return; }
    
    let idx = get_index(id.x, id.y, id.z);
    let voxel = world.data[idx];
    let seed = id.x * 1337u + id.y * 7331u + id.z * 1234u + u32(uniforms.time * 100.0);

    if (voxel == MAT_AIR) { return; }

    if (voxel == MAT_SAND) {
        simulate_sand(idx, id, seed);
    } else if (voxel == MAT_WATER) {
        simulate_liquid(idx, id, seed, MAT_WATER, 1u);
    } else if (voxel == MAT_MAGMA) {
        simulate_liquid(idx, id, seed, MAT_MAGMA, 15u);
    } else if (voxel == MAT_GAS) {
        simulate_gas(idx, id, seed);
    } else if (voxel == MAT_DIRT) {
        simulate_dirt(idx, id, seed);
    }
}

fn move_voxel(from_idx: u32, to_idx: u32, tx: u32, ty: u32, tz: u32, mat: u32) {
    world.data[from_idx] = MAT_AIR;
    world.data[to_idx] = mat;
    macro_world.data[get_macro_index(tx, ty, tz)] = 1u;
}

fn is_valid(x: u32, y: u32, z: u32) -> bool {
    return x > 0u && x < 255u && y > 0u && y < 255u && z > 0u && z < 255u;
}