#define_import_path physics_sand
#import globals::{world}
#import constants::{MAT_AIR, MAT_SAND}
#import math::{get_planet_gravity_dir, is_valid, get_index, get_orthogonal, move_voxel}

fn simulate_sand(idx: u32, pos: vec3<u32>, seed: u32) {
    let g_dir = get_planet_gravity_dir(vec3<f32>(pos));
    let down = vec3<u32>(vec3<i32>(pos) + g_dir);

    if !is_valid(down.x, down.y, down.z) { return; }

    if world.data[get_index(down.x, down.y, down.z)] == MAT_AIR {
        move_voxel(idx, get_index(down.x, down.y, down.z), down.x, down.y, down.z, MAT_SAND);
    } else {
        let side = get_orthogonal(g_dir, seed % 4u);
        let diag = vec3<u32>(vec3<i32>(down) + side);
        if is_valid(diag.x, diag.y, diag.z) && world.data[get_index(diag.x, diag.y, diag.z)] == MAT_AIR {
            move_voxel(idx, get_index(diag.x, diag.y, diag.z), diag.x, diag.y, diag.z, MAT_SAND);
        }
    }
}