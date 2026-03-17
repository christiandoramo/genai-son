// Lida EXCLUSIVAMENTE com a balística, voo e impacto de detritos/mísseis

fn explode(center: vec3<f32>, my_id: u32, radius: f32) {
    let cx = u32(clamp(center.x, 0.0, 511.0)); let cy = u32(clamp(center.y, 0.0, 127.0)); let cz = u32(clamp(center.z, 0.0, 511.0));
    var hit_mat_sample = 8u; 
    let r_int = u32(ceil(radius));
    
    for (var x = max(1u, cx - r_int); x < min(511u, cx + r_int); x++) {
        for (var y = max(1u, cy - r_int); y < min(127u, cy + r_int); y++) {
            for (var z = max(1u, cz - r_int); z < min(511u, cz + r_int); z++) {
                let dist = length(vec3<f32>(f32(x), f32(y), f32(z)) - center);
                let idx = get_index(x, y, z);
                let v = world.data[idx];
                
                if (dist < radius * 0.7 && v != 4u) { 
                    world.data[idx] = 0u; 
                    if (v != 0u && v < 100u) { hit_mat_sample = v; }
                } else if (dist < radius && v != 0u && v != 4u && v < 100u) {
                    world.data[idx] = v + 100u; 
                    macro_world.data[get_macro_index(x, y, z)] = 1u;
                }
            }
        }
    }

    var spawned = 0u;
    for (var i = 0u; i < 64u; i++) {
        if (projectiles[i].is_active == 0u && i != my_id) {
            projectiles[i].is_active = 1u; projectiles[i].p_type = 2u;
            projectiles[i].mat_id = hit_mat_sample; 
            projectiles[i].pos = center + vec3<f32>(0.0, 2.0, 0.0);
            let rx = f32((i * 137u) % 11u) / 5.0 - 1.0; let rz = f32((i * 271u) % 11u) / 5.0 - 1.0;
            projectiles[i].vel = vec3<f32>(rx * radius * 0.5, (radius * 0.3) + f32(i % 4u), rz * radius * 0.5); 
            spawned++; if (spawned > 20u) { break; }
        }
    }
}

fn update_projectiles(flat_id: u32) {
    if (projectiles[flat_id].is_active == 1u) {
        let p = projectiles[flat_id];
        let next_pos = p.pos + p.vel; let v_pos = vec3<u32>(next_pos);
        
        if (is_valid(v_pos.x, v_pos.y, v_pos.z)) {
            let hit_voxel = world.data[get_index(v_pos.x, v_pos.y, v_pos.z)];
            if (hit_voxel != 0u && hit_voxel != 3u) { 
                projectiles[flat_id].is_active = 0u;
                if (p.p_type == 1u) { 
                    explode(next_pos, flat_id, 6.0); 
                } 
                else if (p.p_type == 2u) {
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