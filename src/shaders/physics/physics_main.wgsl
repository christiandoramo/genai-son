#import globals::{uniforms, world, macro_world}
#import constants::{MAT_AIR, MAT_SAND, MAT_WATER, MAT_MAGMA, MAT_GAS, MAT_DIRT}
#import math::{is_valid, get_index}
#import physics_sand::{simulate_sand}
#import physics_fluids::{simulate_liquid}
#import physics_gas::{simulate_gas}
#import physics_dirt::{simulate_dirt}

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