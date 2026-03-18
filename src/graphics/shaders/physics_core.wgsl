struct Uniforms { resolution: vec2<f32>, time: f32, action: u32, camera_pos: vec3<f32>, flashlight_on: u32, camera_front: vec3<f32>, _padding3: f32, };
@group(0) @binding(0) var<uniform> uniforms: Uniforms;
struct WorldBuffer { data: array<u32>, };
@group(0) @binding(1) var<storage, read_write> world: WorldBuffer;
@group(0) @binding(2) var<storage, read_write> macro_world: WorldBuffer;

struct Projectile { pos: vec3<f32>, is_active: u32, vel: vec3<f32>, p_type: u32, mat_id: u32, pad1: u32, pad2: u32, pad3: u32 };
@group(0) @binding(3) var<storage, read_write> projectiles: array<Projectile>;

fn get_index(x: u32, y: u32, z: u32) -> u32 { return x + y * 256u + z * 256u * 256u; }
fn get_macro_index(x: u32, y: u32, z: u32) -> u32 { let mx = x >> 3u; let my = y >> 3u; let mz = z >> 3u; return mx + my * 32u + mz * 32u * 32u; }

fn is_valid(x: u32, y: u32, z: u32) -> bool { return x > 0u && x < 255u && y > 0u && y < 255u && z > 0u && z < 255u; }

fn move_voxel(idx_from: u32, idx_to: u32, x: u32, y: u32, z: u32, material: u32) {
    world.data[idx_from] = 0u; world.data[idx_to] = material; macro_world.data[get_macro_index(x, y, z)] = 1u;
}

@compute @workgroup_size(4, 4, 4)
fn cp_main(@builtin(global_invocation_id) local_id: vec3<u32>) {
    let flat_id = local_id.x + local_id.y * 4u + local_id.z * 16u;
    
// 1. Orquestração Balística (Delega para projectiles.wgsl e weapons.wgsl)
    if (flat_id < 64u) {
        if (flat_id == 0u) { try_fire_weapons(); } // O NOME ATUALIZADO AQUI!
        update_projectiles(flat_id);
    }

    // 2. Mapeamento da Thread para o Mundo (Raio de 64 blocos ao redor da câmera)
    let cam_x = u32(max(0.0, uniforms.camera_pos.x - 64.0)); let cam_y = u32(max(0.0, uniforms.camera_pos.y - 64.0)); let cam_z = u32(max(0.0, uniforms.camera_pos.z - 64.0));
    let x = cam_x + local_id.x; let y = cam_y + local_id.y; let z = cam_z + local_id.z;
    if (!is_valid(x, y, z)) { return; }

    let index = get_index(x, y, z);
    let voxel = world.data[index];

    // 3. Orquestração do Jogador (Delega para weapons.wgsl)
    process_player_weapons(index, x, y, z, voxel);

// 4. Física do Núcleo: Autômatos Celulares (Delega para physics_fluids.wgsl)
    if (voxel == 0u || voxel == 4u || voxel == 5u || voxel == 7u || voxel == 8u) { return; }
    let seed = x * 1973u + y * 9277u + z * 26699u + u32(uniforms.time * 1000.0);
    
    if (voxel == 1u) { simulate_sand(index, x, y, z, seed); }
    else if (voxel == 2u) { simulate_liquid(index, x, y, z, seed, 2u, 1u); } 
    else if (voxel == 9u) { simulate_liquid(index, x, y, z, seed, 9u, 15u); } 
    else if (voxel == 3u) { 
        // O TETO SUBIU! Gás agora sobe até o limite do novo eixo Y (256)
if (y >= 254u) { world.data[index] = 0u; return; }
        let up = get_index(x,y+1u,z); 
        if (world.data[up] == 0u) { move_voxel(index,up,x,y+1u,z,3u); } 
    }
    else if (voxel > 100u) {
        let down = get_index(x, y - 1u, z);
        if (world.data[down] == 0u) { move_voxel(index, down, x, y - 1u, z, voxel); }
        else { world.data[index] = voxel % 100u; }
    }
}