struct Uniforms { resolution: vec2<f32>, time: f32, action: u32, camera_pos: vec3<f32>, flashlight_on: u32, camera_front: vec3<f32>, _padding3: f32, };
@group(0) @binding(0) var<uniform> uniforms: Uniforms;
struct WorldBuffer { data: array<u32>, };
@group(0) @binding(1) var<storage, read_write> world: WorldBuffer;
@group(0) @binding(2) var<storage, read_write> macro_world: WorldBuffer;

struct Projectile { pos: vec3<f32>, is_active: u32, vel: vec3<f32>, p_type: u32, mat_id: u32, pad1: u32, pad2: u32, pad3: u32 };
@group(0) @binding(3) var<storage, read_write> projectiles: array<Projectile>;

fn get_index(x: u32, y: u32, z: u32) -> u32 { return x + y * 512u + z * 512u * 128u; }
fn get_macro_index(x: u32, y: u32, z: u32) -> u32 { let mx = x >> 3u; let my = y >> 3u; let mz = z >> 3u; return mx + my * 64u + mz * 64u * 16u; }
fn is_valid(x: u32, y: u32, z: u32) -> bool { return x > 0u && x < 511u && y > 0u && y < 127u && z > 0u && z < 511u; }

fn move_voxel(idx_from: u32, idx_to: u32, x: u32, y: u32, z: u32, material: u32) {
    world.data[idx_from] = 0u; world.data[idx_to] = material; macro_world.data[get_macro_index(x, y, z)] = 1u;
}

fn explode(center: vec3<f32>, my_id: u32) {
    let r = 5.0; 
    let cx = u32(clamp(center.x, 0.0, 511.0)); let cy = u32(clamp(center.y, 0.0, 127.0)); let cz = u32(clamp(center.z, 0.0, 511.0));
    
    // Varredura para encontrar QUAL MATERIAL estilhaçar e desintegrar o miolo
    var hit_mat_sample = 8u; // Default Pedra
    
    for (var x = max(1u, cx - 5u); x < min(511u, cx + 5u); x++) {
        for (var y = max(1u, cy - 5u); y < min(127u, cy + 5u); y++) {
            for (var z = max(1u, cz - 5u); z < min(511u, cz + 5u); z++) {
                let dist = length(vec3<f32>(f32(x), f32(y), f32(z)) - center);
                let idx = get_index(x, y, z);
                let v = world.data[idx];
                
                if (dist < 4.0 && v != 4u) { 
                    world.data[idx] = 0u; 
                    if (v != 0u && v < 100u) { hit_mat_sample = v; }
                } else if (dist < r && v != 0u && v != 4u && v < 100u) {
                    // Magia: Transforma o bloco da borda em "Detrito Caindo" somando 100 no ID!
                    world.data[idx] = v + 100u; 
                    macro_world.data[get_macro_index(x, y, z)] = 1u;
                }
            }
        }
    }

    // Estilhaços
    var spawned = 0u;
    for (var i = 0u; i < 64u; i++) {
        if (projectiles[i].is_active == 0u && i != my_id) {
            projectiles[i].is_active = 1u; projectiles[i].p_type = 2u;
            projectiles[i].mat_id = hit_mat_sample; 
            projectiles[i].pos = center + vec3<f32>(0.0, 2.0, 0.0);
            let rx = f32((i * 137u) % 11u) / 5.0 - 1.0; let rz = f32((i * 271u) % 11u) / 5.0 - 1.0;
            projectiles[i].vel = vec3<f32>(rx * 2.5, 2.0 + f32(i % 4u), rz * 2.5); 
            spawned++; if (spawned > 20u) { break; }
        }
    }
}

fn simulate_sand(idx: u32, x: u32, y: u32, z: u32, rand: u32) {
    let down = get_index(x, y - 1u, z);
    if (world.data[down] == 0u) { move_voxel(idx, down, x, y - 1u, z, 1u); return; }
    let dx = select(x - 1u, x + 1u, (rand % 2u) == 0u); let dz = select(z - 1u, z + 1u, (rand / 2u) == 0u);
    let diag1 = get_index(dx, y - 1u, z); if (world.data[diag1] == 0u) { move_voxel(idx, diag1, dx, y - 1u, z, 1u); return; }
    let diag2 = get_index(x, y - 1u, dz); if (world.data[diag2] == 0u) { move_voxel(idx, diag2, x, y - 1u, dz, 1u); return; }
}

fn simulate_liquid(idx: u32, x: u32, y: u32, z: u32, rand: u32, mat_id: u32, speed_limit: u32) {
    let down = get_index(x, y - 1u, z);
    if (world.data[down] == 0u) { move_voxel(idx, down, x, y - 1u, z, mat_id); return; }
    
    let dirs = array<vec2<u32>, 4>(vec2<u32>(1u, 0u), vec2<u32>(0u, 1u), vec2<u32>(4294967295u, 0u), vec2<u32>(0u, 4294967295u));
    // Procura cachoeira
    for(var i = 0u; i < 4u; i++) {
        let d = dirs[(rand + i) % 4u]; let side = get_index(x + d.x, y, z + d.y);
        if (world.data[side] == 0u) {
            let side_down = get_index(x + d.x, y - 1u, z + d.y);
            if (world.data[side_down] == 0u) { move_voxel(idx, side, x + d.x, y, z + d.y, mat_id); return; }
        }
    }
    
    // Viscosidade: Só se espalha em planos a cada X frames
    if (rand % speed_limit != 0u) { return; } 
    let d = dirs[rand % 4u]; let side = get_index(x + d.x, y, z + d.y);
    if (world.data[side] == 0u) {
        let s_up = get_index(x + d.x, y + 1u, z + d.y);
        if (world.data[s_up] != mat_id) { move_voxel(idx, side, x + d.x, y, z + d.y, mat_id); }
    }
}

@compute @workgroup_size(4, 4, 4)
fn cp_main(@builtin(global_invocation_id) local_id: vec3<u32>) {
    let flat_id = local_id.x + local_id.y * 4u + local_id.z * 16u;
    if (flat_id < 64u) {
        if (flat_id == 0u && uniforms.action == 9u) {
            for (var i = 0u; i < 64u; i++) {
                if (projectiles[i].is_active == 0u) {
                    projectiles[i].is_active = 1u; projectiles[i].p_type = 1u;
                    projectiles[i].pos = uniforms.camera_pos + uniforms.camera_front * 2.0;
                    projectiles[i].vel = uniforms.camera_front * 3.0; 
                    break;
                }
            }
        }
        
        if (projectiles[flat_id].is_active == 1u) {
            let p = projectiles[flat_id];
            let next_pos = p.pos + p.vel; let v_pos = vec3<u32>(next_pos);
            
            if (is_valid(v_pos.x, v_pos.y, v_pos.z)) {
                let hit_voxel = world.data[get_index(v_pos.x, v_pos.y, v_pos.z)];
                
                if (hit_voxel != 0u && hit_voxel != 3u) { 
                    projectiles[flat_id].is_active = 0u;
                    if (p.p_type == 1u) { explode(next_pos, flat_id); } 
                    else if (p.p_type == 2u) {
                        // Estilhaço Caiu no chão. Vira a matéria ORIGINAL!
                        let place = vec3<u32>(p.pos);
                        world.data[get_index(place.x, place.y, place.z)] = p.mat_id;
                        macro_world.data[get_macro_index(place.x, place.y, place.z)] = 1u;
                    }
                } else {
                    projectiles[flat_id].pos = next_pos;
                    if (p.p_type == 2u) { projectiles[flat_id].vel.y -= 0.15; } 
                }
            } else { projectiles[flat_id].is_active = 0u; }
        }
    }

    let cam_x = u32(max(0.0, uniforms.camera_pos.x - 64.0)); let cam_y = u32(max(0.0, uniforms.camera_pos.y - 64.0)); let cam_z = u32(max(0.0, uniforms.camera_pos.z - 64.0));
    let x = cam_x + local_id.x; let y = cam_y + local_id.y; let z = cam_z + local_id.z;
    if (!is_valid(x, y, z)) { return; }

    let index = get_index(x, y, z);
    let voxel = world.data[index];
    let p = vec3<f32>(f32(x), f32(y), f32(z));

    if (uniforms.action == 8u) { 
        let p_to_cam = p - uniforms.camera_pos; let proj = dot(p_to_cam, uniforms.camera_front); 
        if (proj > 2.0 && proj < 40.0 && length(p - (uniforms.camera_pos + uniforms.camera_front * proj)) < 2.0) { 
            if (voxel != 4u) { world.data[index] = 0u; } 
        }
    } else if (uniforms.action > 0u && uniforms.action < 8u && uniforms.action != 4u && uniforms.action != 6u) {
        if (length(p - (uniforms.camera_pos + uniforms.camera_front * 10.0)) < 3.0) { world.data[index] = uniforms.action; macro_world.data[get_macro_index(x,y,z)] = 1u; }
    }

    if (voxel == 0u || voxel == 4u || voxel == 5u || voxel == 7u || voxel == 8u) { return; }
    let seed = x * 1973u + y * 9277u + z * 26699u + u32(uniforms.time * 1000.0);
    
    if (voxel == 1u) { simulate_sand(index, x, y, z, seed); }
    else if (voxel == 2u) { simulate_liquid(index, x, y, z, seed, 2u, 6u); } // Água (Rápida)
    else if (voxel == 9u) { simulate_liquid(index, x, y, z, seed, 9u, 25u); } // Magma (Lento e Viscoso)
    else if (voxel == 3u) { 
        if (y >= 126u) { world.data[index] = 0u; return; }
        let up = get_index(x,y+1u,z); 
        if (world.data[up] == 0u) { move_voxel(index,up,x,y+1u,z,3u); } 
        else {
            let dirs = array<vec2<u32>, 4>(vec2<u32>(1u, 0u), vec2<u32>(0u, 1u), vec2<u32>(4294967295u, 0u), vec2<u32>(0u, 4294967295u));
            let d = dirs[seed % 4u]; let side = get_index(x + d.x, y, z + d.y);
            if (world.data[side] == 0u) { move_voxel(index, side, x + d.x, y, z + d.y, 3u); }
        }
    }
    // A mágica: Blocos com ID > 100 são DETRITOS CAINDO das explosões!
    else if (voxel > 100u) {
        let down = get_index(x, y - 1u, z);
        if (world.data[down] == 0u) { move_voxel(index, down, x, y - 1u, z, voxel); }
        else { world.data[index] = voxel % 100u; } // Bateu no chão? Volta a ser o bloco original duro!
    }
}