#define_import_path math
#import constants::{WORLD_SIZE, MACRO_SIZE, MAT_AIR, MAT_SAND, MAT_WATER, MAT_GAS, MAT_SNOW, MAT_DIRT, MAT_GRASS, MAT_ROCK, MAT_MAGMA, MAT_IRON}
#import globals::{world, macro_world}
#import biomes::{get_biome_color}

fn hash(p: vec3<f32>) -> f32 {
    var p3 = fract(p * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z) * 2.0 - 1.0;
}

fn noise(x: vec3<f32>) -> f32 {
    let p = floor(x); let f = fract(x);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(mix(mix(hash(p + vec3<f32>(0.0,0.0,0.0)), hash(p + vec3<f32>(1.0,0.0,0.0)), u.x),
                   mix(hash(p + vec3<f32>(0.0,1.0,0.0)), hash(p + vec3<f32>(1.0,1.0,0.0)), u.x), u.y),
               mix(mix(hash(p + vec3<f32>(0.0,0.0,1.0)), hash(p + vec3<f32>(1.0,0.0,1.0)), u.x),
                   mix(hash(p + vec3<f32>(0.0,1.0,1.0)), hash(p + vec3<f32>(1.0,1.0,1.0)), u.x), u.y), u.z);
}

fn get_index(x: u32, y: u32, z: u32) -> u32 {
    return x + y * 256u + z * 65536u;
}

fn get_macro_index(x: u32, y: u32, z: u32) -> u32 {
    let mx = x >> 3u; let my = y >> 3u; let mz = z >> 3u;
    return mx + my * 32u + mz * 1024u;
}

fn get_planet_gravity_dir(pos: vec3<f32>) -> vec3<i32> {
    let rel = pos - vec3<f32>(128.0);
    let a = abs(rel);
    if (a.x >= a.y && a.x >= a.z) { return vec3<i32>(i32(-sign(rel.x)), 0, 0); }
    if (a.y >= a.x && a.y >= a.z) { return vec3<i32>(0, i32(-sign(rel.y)), 0); }
    return vec3<i32>(0, 0, i32(-sign(rel.z)));
}

fn get_voxel_color(voxel_id: u32) -> vec3<f32> {
    return get_biome_color(voxel_id, 0.0); // Delega para o biomes.wgsl
}

fn is_valid_i(p: vec3<i32>) -> bool {
    return p.x >= 0 && p.x < 256 && p.y >= 0 && p.y < 256 && p.z >= 0 && p.z < 256;
}


fn get_random_side(pos: vec3<u32>, seed: u32) -> vec3<u32> {
    let side_offsets = array<vec3<i32>, 4>(
        vec3<i32>(1, 0, 0), vec3<i32>(-1, 0, 0), 
        vec3<i32>(0, 0, 1), vec3<i32>(0, 0, -1)
    );
    return vec3<u32>(vec3<i32>(pos) + side_offsets[seed % 4u]);
}

fn get_orthogonal(g: vec3<i32>, index: u32) -> vec3<i32> {
    var u = vec3<i32>(0); var v = vec3<i32>(0);
    if (g.x != 0) { u = vec3<i32>(0,1,0); v = vec3<i32>(0,0,1); }
    else if (g.y != 0) { u = vec3<i32>(1,0,0); v = vec3<i32>(0,0,1); }
    else { u = vec3<i32>(1,0,0); v = vec3<i32>(0,1,0); }
    if (index == 0u) { return u; } if (index == 1u) { return -u; } if (index == 2u) { return v; } return -v;
}

fn move_voxel(from_idx: u32, to_idx: u32, tx: u32, ty: u32, tz: u32, mat: u32) {
    world.data[from_idx] = MAT_AIR;
    world.data[to_idx] = mat;
    macro_world.data[get_macro_index(tx, ty, tz)] = 1u;
}

fn is_valid(x: u32, y: u32, z: u32) -> bool {
    return x > 0u && x < 255u && y > 0u && y < 255u && z > 0u && z < 255u;
}

fn is_free(v: u32) -> bool {
    return v == MAT_AIR || v == MAT_GAS;
}

fn get_normal(side: u32) -> vec3<f32> {
    if (side == 0u) { return vec3<f32>(1.0, 0.0, 0.0); }
    if (side == 1u) { return vec3<f32>(0.0, 1.0, 0.0); }
    return vec3<f32>(0.0, 0.0, 1.0);
}