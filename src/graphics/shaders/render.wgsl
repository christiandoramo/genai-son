struct VertexOutput { @builtin(position) clip_position: vec4<f32>, @location(0) uv: vec2<f32>, };
struct Uniforms { resolution: vec2<f32>, time: f32, action: u32, camera_pos: vec3<f32>, flashlight_on: u32, camera_front: vec3<f32>, _padding3: f32, };
struct WorldBuffer { data: array<u32>, };
struct Projectile { pos: vec3<f32>, is_active: u32, vel: vec3<f32>, p_type: u32, mat_id: u32, pad1: u32, pad2: u32, pad3: u32 };

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read> world: WorldBuffer;
@group(0) @binding(2) var<storage, read> macro_world: WorldBuffer;
@group(0) @binding(3) var<storage, read> projectiles: array<Projectile>; 

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32((in_vertex_index << 1u) & 2u); let y = f32(in_vertex_index & 2u);
    out.clip_position = vec4<f32>(x * 2.0 - 1.0, 1.0 - y * 2.0, 0.0, 1.0); out.uv = vec2<f32>(x, y); return out;
}

fn get_voxel(p: vec3<u32>) -> u32 {
    if (p.x >= 512u || p.y >= 128u || p.z >= 512u) { return 0u; }
    return world.data[p.x + p.y * 512u + p.z * 65536u]; 
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let retro_res = vec2<f32>(320.0, 240.0);
    var uv = floor(in.clip_position.xy / uniforms.resolution.xy * retro_res) / retro_res;
    uv = uv * 2.0 - 1.0; uv.x *= uniforms.resolution.x / uniforms.resolution.y; uv.y = -uv.y;

    var ro = uniforms.camera_pos; 
    let forward = normalize(uniforms.camera_front); let right = normalize(cross(vec3<f32>(0.0, 1.0, 0.0), forward)); let up = cross(forward, right);
    let rd = normalize(forward + uv.x * right + uv.y * up);

    let sun_dir = normalize(vec3<f32>(sin(uniforms.time * 0.2), cos(uniforms.time * 0.2), 0.5));
    let sky_day = vec3<f32>(0.3, 0.5, 0.8); let sky_sunset = vec3<f32>(0.8, 0.4, 0.2); let sky_night = vec3<f32>(0.01, 0.01, 0.02);
    var sky_color = sky_night;
    if (sun_dir.y > 0.1) { sky_color = sky_day; } else if (sun_dir.y > -0.2) { sky_color = mix(sky_sunset, sky_day, clamp((sun_dir.y + 0.2) / 0.3, 0.0, 1.0)); } else { sky_color = mix(sky_night, sky_sunset, clamp((sun_dir.y + 0.4) / 0.2, 0.0, 1.0)); }

    var proj_hit = false; var proj_dist = 9999.0; var hit_proj_idx = 0u;
    for (var i = 0u; i < 64u; i++) {
        if (projectiles[i].is_active == 1u) {
            let oc = ro - projectiles[i].pos;
            let b = dot(oc, rd); let c = dot(oc, oc) - 0.3 * 0.3; let h = b * b - c;
            if (h > 0.0) {
                let t = -b - sqrt(h);
                if (t > 0.0 && t < proj_dist) { proj_dist = t; proj_hit = true; hit_proj_idx = i; }
            }
        }
    }

    let tmin = (vec3<f32>(0.0) - ro) / rd; let tmax = (vec3<f32>(512.0, 128.0, 512.0) - ro) / rd;
    let t1 = min(tmin, tmax); let t2 = max(tmin, tmax);
    let t_near = max(max(t1.x, t1.y), t1.z); let t_far = min(min(t2.x, t2.y), t2.z);
    
    if (t_far < max(t_near, 0.0)) { return vec4<f32>(floor(sky_color * 8.0) / 8.0, 1.0); }
    if (t_near > 0.0) { ro = ro + rd * (t_near + 0.005); }

    var map_pos = vec3<i32>(floor(ro));
    let delta_dist = abs(1.0 / rd); let step = vec3<i32>(sign(rd)); var side_dist = vec3<f32>(0.0);
    
    if (rd.x < 0.0) { side_dist.x = (ro.x - f32(map_pos.x)) * delta_dist.x; } else { side_dist.x = (f32(map_pos.x) + 1.0 - ro.x) * delta_dist.x; }
    if (rd.y < 0.0) { side_dist.y = (ro.y - f32(map_pos.y)) * delta_dist.y; } else { side_dist.y = (f32(map_pos.y) + 1.0 - ro.y) * delta_dist.y; }
    if (rd.z < 0.0) { side_dist.z = (ro.z - f32(map_pos.z)) * delta_dist.z; } else { side_dist.z = (f32(map_pos.z) + 1.0 - ro.z) * delta_dist.z; }

    var hit = false; var side = 0; 
    for (var i = 0; i < 500; i++) {
        if (map_pos.x < 0 || map_pos.x >= 512 || map_pos.y < 0 || map_pos.y >= 128 || map_pos.z < 0 || map_pos.z >= 512) { break; }
        let upos = vec3<u32>(map_pos);
        let macro_pos = upos >> vec3<u32>(3u, 3u, 3u); 
        
        if (macro_world.data[macro_pos.x + macro_pos.y * 64u + macro_pos.z * 1024u] == 0u) {
            var bound_x = f32(macro_pos.x << 3u); if (step.x > 0) { bound_x += 8.0; }
            var bound_y = f32(macro_pos.y << 3u); if (step.y > 0) { bound_y += 8.0; }
            var bound_z = f32(macro_pos.z << 3u); if (step.z > 0) { bound_z += 8.0; }
            var tx = (bound_x - ro.x) / rd.x; if (abs(rd.x) < 0.0001) { tx = 999999.0; }
            var ty = (bound_y - ro.y) / rd.y; if (abs(rd.y) < 0.0001) { ty = 999999.0; }
            var tz = (bound_z - ro.z) / rd.z; if (abs(rd.z) < 0.0001) { tz = 999999.0; }
            let t_next = min(min(tx, ty), tz);
            map_pos = vec3<i32>(floor(ro + rd * (t_next + 0.01)));
            if (rd.x < 0.0) { side_dist.x = (ro.x - f32(map_pos.x)) * delta_dist.x; } else { side_dist.x = (f32(map_pos.x) + 1.0 - ro.x) * delta_dist.x; }
            if (rd.y < 0.0) { side_dist.y = (ro.y - f32(map_pos.y)) * delta_dist.y; } else { side_dist.y = (f32(map_pos.y) + 1.0 - ro.y) * delta_dist.y; }
            if (rd.z < 0.0) { side_dist.z = (ro.z - f32(map_pos.z)) * delta_dist.z; } else { side_dist.z = (f32(map_pos.z) + 1.0 - ro.z) * delta_dist.z; }
            continue; 
        }
        if (get_voxel(upos) != 0u) { hit = true; break; }
        if (side_dist.x < side_dist.y && side_dist.x < side_dist.z) { side_dist.x += delta_dist.x; map_pos.x += step.x; side = 0; } 
        else if (side_dist.y < side_dist.z) { side_dist.y += delta_dist.y; map_pos.y += step.y; side = 1; } 
        else { side_dist.z += delta_dist.z; map_pos.z += step.z; side = 2; }
    }

    var color = sky_color;
    var voxel_dist = 9999.0;
    var diffuse = 0.0;
    var final_sun_light = vec3<f32>(1.0, 0.95, 0.9); 

    if (hit) {
        if (side == 0) { voxel_dist = side_dist.x - delta_dist.x; } else if (side == 1) { voxel_dist = side_dist.y - delta_dist.y; } else { voxel_dist = side_dist.z - delta_dist.z; }
        let hit_pos = ro + rd * voxel_dist;
        
        let raw_type = get_voxel(vec3<u32>(map_pos));
        let voxel_type = raw_type % 100u; // Ignora se é "detrito caindo" e pega só a cor nativa!
        var base_color = vec3<f32>(1.0);

     // ... (definição de cores base acima) ...
        if (voxel_type == 4u) { let is_white = (u32(map_pos.x) + u32(map_pos.y) + u32(map_pos.z)) % 2u == 0u; base_color = select(vec3<f32>(0.15), vec3<f32>(0.85), is_white); } 
        else if (voxel_type == 1u) { base_color = vec3<f32>(0.9, 0.8, 0.2); } 
        else if (voxel_type == 2u) { 
            // Água: Adiciona movimento senoidal sutil na cor
            let wave = sin(uniforms.time * 2.0 + hit_pos.x + hit_pos.z) * 0.05;
            base_color = vec3<f32>(0.2, 0.4, 0.9) + wave; 
        } 
        else if (voxel_type == 3u) { base_color = vec3<f32>(0.6, 0.6, 0.6); } 
        else if (voxel_type == 5u) { base_color = vec3<f32>(0.4, 0.25, 0.1); } 
        else if (voxel_type == 7u) { base_color = vec3<f32>(0.2, 0.6, 0.2); } 
        else if (voxel_type == 8u) { base_color = vec3<f32>(0.4, 0.4, 0.4); } 
        else if (voxel_type == 9u) { 
            // Magma: Movimento mais frenético e cor emissiva pura
            let pulsate = sin(uniforms.time * 4.0 + hit_pos.x * 2.0 + hit_pos.y) * 0.1;
            base_color = vec3<f32>(1.0, 0.3, 0.0) + pulsate; 
        } 

        let noise = f32((map_pos.x * 7 + map_pos.y * 3 + map_pos.z * 5) % 3) * 0.1;
        base_color = base_color + noise;
        
        diffuse = max(dot(vec3<f32>(select(0.0, -f32(step.x), side == 0), select(0.0, -f32(step.y), side == 1), select(0.0, -f32(step.z), side == 2)), sun_dir), 0.0); 
        
        // Magma ignora a luz do sol (Fullbright / Glow)
        if (voxel_type == 9u) { 
            color = base_color * 1.3; // Glow extra forte
        } else {
            color = base_color * (diffuse * final_sun_light + vec3<f32>(max(0.0, sun_dir.y * 0.2)));
        }
        
        // LANTERNA TÁTICA (Ajustada para ser suave)
        if (uniforms.flashlight_on == 1u) {
            let to_hit = normalize(hit_pos - ro);
            let spot = dot(to_hit, forward);
            if (spot > 0.85) { // Cone fechado e focado
                let falloff = smoothstep(0.85, 0.98, spot); 
                let dist_fade = smoothstep(60.0, 2.0, voxel_dist); // Decaimento mais realista
                // Cor amarelada quente e muito mais suave (0.4 de intensidade invés de 1.5)
                color += vec3<f32>(1.0, 0.95, 0.85) * (falloff * dist_fade * 0.4);
            }
        }
    }

    if (proj_hit && proj_dist < voxel_dist) {
        let p = projectiles[hit_proj_idx];
        if (p.p_type == 1u) {
            color = vec3<f32>(0.2); // Míssil (Cinza)
        } else {
            // A MAGIA VISUAL: O Estilhaço voador tem a COR EXATA do material que você atirou!
            let m = p.mat_id % 100u;
            if (m == 1u) { color = vec3<f32>(0.9, 0.8, 0.2); }
            else if (m == 5u) { color = vec3<f32>(0.4, 0.25, 0.1); }
            else if (m == 7u) { color = vec3<f32>(0.2, 0.6, 0.2); }
            else if (m == 8u) { color = vec3<f32>(0.4, 0.4, 0.4); }
            else { color = vec3<f32>(0.5); }
        }
        voxel_dist = proj_dist;
    }

    let fog_factor = clamp((voxel_dist * 0.05) / 20.0, 0.0, 1.0);
    color = mix(color, sky_color, fog_factor);
    color = floor(color * 8.0) / 8.0; 

    return vec4<f32>(color, 1.0);
}