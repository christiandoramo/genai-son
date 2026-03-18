
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

struct Uniforms {
    resolution: vec2<f32>,
    time: f32,
    action: u32,
    camera_pos: vec3<f32>,
    flashlight_on: u32,
    camera_front: vec3<f32>,
    pad1: f32,
    camera_up: vec3<f32>,
    pad2: f32,
};
struct WorldBuffer {
    data: array<u32>,
};
struct Projectile {
    pos: vec3<f32>,
    is_active: u32,
    vel: vec3<f32>,
    p_type: u32,
    mat_id: u32,
    pad1: u32,
    pad2: u32,
    pad3: u32};

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
    if p.x >= 256u || p.y >= 256u || p.z >= 256u { return 0u; }
    return world.data[p.x + p.y * 256u + p.z * 65536u];
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // 1. RESOLUÇÃO PSX (320x240)
    let retro_res = vec2<f32>(320.0, 240.0);
    var uv = floor(in.clip_position.xy / uniforms.resolution.xy * retro_res) / retro_res;
    uv = uv * 2.0 - 1.0;
    uv.x *= uniforms.resolution.x / uniforms.resolution.y;
    uv.y = -uv.y;

    // 2. A CURA DA CÂMERA: A GPU agora respeita o Cima do Planeta!
    var ro = uniforms.camera_pos;
    let forward = normalize(uniforms.camera_front);
    let up_vec = normalize(uniforms.camera_up); // Usa a gravidade real!
    let right = normalize(cross(up_vec, forward));
    let up = cross(forward, right);
    let rd = normalize(forward + uv.x * right + uv.y * up);

    let sun_dir = normalize(vec3<f32>(sin(uniforms.time), cos(uniforms.time), 0.5));

    let sky_day = vec3<f32>(0.3, 0.5, 0.8); let sky_sunset = vec3<f32>(0.8, 0.4, 0.2); let sky_night = vec3<f32>(0.01, 0.01, 0.02);
    var sky_color = sky_night;
    if sun_dir.y > 0.1 { sky_color = sky_day; } else if sun_dir.y > -0.2 { sky_color = mix(sky_sunset, sky_day, clamp((sun_dir.y + 0.2) / 0.3, 0.0, 1.0)); } else { sky_color = mix(sky_night, sky_sunset, clamp((sun_dir.y + 0.4) / 0.2, 0.0, 1.0)); }

    var proj_hit = false; var proj_dist = 9999.0; var hit_proj_idx = 0u;
    for (var i = 0u; i < 64u; i++) {
        if projectiles[i].is_active == 1u {
            let oc = ro - projectiles[i].pos;
            let b = dot(oc, rd); let c = dot(oc, oc) - 0.3 * 0.3; let h = b * b - c;
            if h > 0.0 {
                let t = -b - sqrt(h);
                if t > 0.0 && t < proj_dist { proj_dist = t; proj_hit = true; hit_proj_idx = i; }
            }
        }
    }

    let tmin = (vec3<f32>(0.0) - ro) / rd; let tmax = (vec3<f32>(256.0, 256.0, 256.0) - ro) / rd;

    let t1 = min(tmin, tmax); let t2 = max(tmin, tmax);
    let t_near = max(max(t1.x, t1.y), t1.z); let t_far = min(min(t2.x, t2.y), t2.z);

    if t_far < max(t_near, 0.0) { return vec4<f32>(floor(sky_color * 8.0) / 8.0, 1.0); }
    if t_near > 0.0 { ro = ro + rd * (t_near + 0.005); }

    var map_pos = vec3<i32>(floor(ro));
    let delta_dist = abs(1.0 / rd);
    let step = vec3<i32>(sign(rd));
    var side_dist = vec3<f32>(0.0);

    if rd.x < 0.0 { side_dist.x = (ro.x - f32(map_pos.x)) * delta_dist.x; } else { side_dist.x = (f32(map_pos.x) + 1.0 - ro.x) * delta_dist.x; }
    if rd.y < 0.0 { side_dist.y = (ro.y - f32(map_pos.y)) * delta_dist.y; } else { side_dist.y = (f32(map_pos.y) + 1.0 - ro.y) * delta_dist.y; }
    if rd.z < 0.0 { side_dist.z = (ro.z - f32(map_pos.z)) * delta_dist.z; } else { side_dist.z = (f32(map_pos.z) + 1.0 - ro.z) * delta_dist.z; }

    // DECLARAÇÃO GLOBAL DO RAIO (Obrigatório ficar ANTES do loop 'for')
    var hit = false;
    var side = 0;
    var hit_water = false;
    var water_hit_pos = vec3<f32>(0.0);

    for (var i = 0; i < 500; i++) {

        if map_pos.x < 0 || map_pos.x >= 256 || map_pos.y < 0 || map_pos.y >= 256 || map_pos.z < 0 || map_pos.z >= 256 { break; }

        let upos = vec3<u32>(map_pos);
        let macro_pos = upos >> vec3<u32>(3u, 3u, 3u);

        // A MUDANÇA ESTÁ AQUI (y * 32 e z * 1024):
        if macro_world.data[macro_pos.x + macro_pos.y * 32u + macro_pos.z * 1024u] == 0u {
            var bound_x = f32(macro_pos.x << 3u); if step.x > 0 { bound_x += 8.0; }
            var bound_y = f32(macro_pos.y << 3u); if step.y > 0 { bound_y += 8.0; }
            var bound_z = f32(macro_pos.z << 3u); if step.z > 0 { bound_z += 8.0; }
            var tx = (bound_x - ro.x) / rd.x; if abs(rd.x) < 0.0001 { tx = 999999.0; }
            var ty = (bound_y - ro.y) / rd.y; if abs(rd.y) < 0.0001 { ty = 999999.0; }
            var tz = (bound_z - ro.z) / rd.z; if abs(rd.z) < 0.0001 { tz = 999999.0; }
            let t_next = min(min(tx, ty), tz);
            map_pos = vec3<i32>(floor(ro + rd * (t_next + 0.01)));
            if rd.x < 0.0 { side_dist.x = (ro.x - f32(map_pos.x)) * delta_dist.x; } else { side_dist.x = (f32(map_pos.x) + 1.0 - ro.x) * delta_dist.x; }
            if rd.y < 0.0 { side_dist.y = (ro.y - f32(map_pos.y)) * delta_dist.y; } else { side_dist.y = (f32(map_pos.y) + 1.0 - ro.y) * delta_dist.y; }
            if rd.z < 0.0 { side_dist.z = (ro.z - f32(map_pos.z)) * delta_dist.z; } else { side_dist.z = (f32(map_pos.z) + 1.0 - ro.z) * delta_dist.z; }
            continue;
        }
        let v_id = get_voxel(upos);
        if v_id != 0u {
            if v_id == 2u {
                if !hit_water {
                    hit_water = true;
                    water_hit_pos = vec3<f32>(map_pos); // Salva as coordenadas globais da água!
                }
            } else {
                hit = true; break; // Bateu na rocha sólida!
            }
        }

        if side_dist.x < side_dist.y && side_dist.x < side_dist.z { side_dist.x += delta_dist.x; map_pos.x += step.x; side = 0; } 
        else if side_dist.y < side_dist.z { side_dist.y += delta_dist.y; map_pos.y += step.y; side = 1; } 
        else { side_dist.z += delta_dist.z; map_pos.z += step.z; side = 2; }
    }

    var color = sky_color;
    var voxel_dist = 9999.0;
    var diffuse = 0.0;
    var final_sun_light = vec3<f32>(1.0, 0.95, 0.9);

    if hit {
        if side == 0 { voxel_dist = side_dist.x - delta_dist.x; } else if side == 1 { voxel_dist = side_dist.y - delta_dist.y; } else { voxel_dist = side_dist.z - delta_dist.z; }
        let hit_pos = ro + rd * voxel_dist;

        let raw_type = get_voxel(vec3<u32>(map_pos));
        let voxel_type = raw_type % 100u; // Ignora se é "detrito caindo" e pega só a cor nativa!
        var base_color = vec3<f32>(1.0);

        // ... (definição de cores base acima) ...

        if voxel_type == 4u { base_color = vec3<f32>(0.9, 0.95, 1.0); }
        else if voxel_type == 1u { base_color = vec3<f32>(0.9, 0.8, 0.2); } 
        else if voxel_type == 3u { base_color = vec3<f32>(0.6, 0.6, 0.6); } 
        else if voxel_type == 5u { base_color = vec3<f32>(0.4, 0.25, 0.1); } 
        else if voxel_type == 7u { base_color = vec3<f32>(0.2, 0.6, 0.2); } 
        else if voxel_type == 8u { base_color = vec3<f32>(0.4, 0.4, 0.4); } 
        else if voxel_type == 9u {
            // Magma: Movimento mais frenético e cor emissiva pura
            let flow = sin(uniforms.time * 5.0 + hit_pos.x * 2.5) * cos(uniforms.time * 4.0 + hit_pos.z * 2.5) * 0.15;
            base_color = vec3<f32>(1.0, 0.3, 0.0) + flow;
        }
        else if voxel_type == 10u {
            // Ferro Metálico Brilhante
            base_color = vec3<f32>(0.8, 0.85, 0.9);
            final_sun_light = vec3<f32>(2.0); // Especularidade alta (brilho)
        }

        diffuse = max(dot(vec3<f32>(select(0.0, -f32(step.x), side == 0), select(0.0, -f32(step.y), side == 1), select(0.0, -f32(step.z), side == 2)), sun_dir), 0.0);

        // Magma ignora a luz do sol (Fullbright / Glow)
        if voxel_type == 9u {
            color = base_color * 1.3; // Glow extra forte
        } else {
            color = base_color * (diffuse * final_sun_light + vec3<f32>(max(0.0, sun_dir.y * 0.2)));
        }

        // LANTERNA TÁTICA (Ajustada para ser suave)
        if uniforms.flashlight_on == 1u {
            let to_hit = normalize(hit_pos - ro);
            let spot = dot(to_hit, forward);
            if spot > 0.85 { // Cone fechado e focado
                let falloff = smoothstep(0.85, 0.98, spot);
                let dist_fade = smoothstep(60.0, 2.0, voxel_dist); // Decaimento mais realista
                // Cor amarelada quente e muito mais suave (0.4 de intensidade invés de 1.5)
                color += vec3<f32>(1.0, 0.95, 0.85) * (falloff * dist_fade * 0.4);
            }
        }
    }

    // A MÁGICA DA ÁGUA (Transparência Blendada e Fluxo) 
    if hit_water {
        // Usa o water_hit_pos (Posição global 3D) para as ondas
        let wave_x = sin(uniforms.time * 3.0 + water_hit_pos.x * 1.5);
        let wave_z = cos(uniforms.time * 2.0 + water_hit_pos.z * 1.5);
        let flow = wave_x * wave_z * 0.15;

        let water_color = vec3<f32>(0.1, 0.4, 0.8) + flow;
        color = mix(color, water_color, 0.6); // Mistura a rocha do fundo com a água
    }

    if proj_hit && proj_dist < voxel_dist {
        let p = projectiles[hit_proj_idx];
        if p.p_type == 1u {
            color = vec3<f32>(0.2); // Míssil (Cinza)
        } else {
            // A MAGIA VISUAL: O Estilhaço voador tem a COR EXATA do material que você atirou!
            let m = p.mat_id % 100u;
            if m == 1u { color = vec3<f32>(0.9, 0.8, 0.2); }
            else if m == 5u { color = vec3<f32>(0.4, 0.25, 0.1); }
            else if m == 7u { color = vec3<f32>(0.2, 0.6, 0.2); }
            else if m == 8u { color = vec3<f32>(0.4, 0.4, 0.4); }
            else { color = vec3<f32>(0.5); }
        }
        voxel_dist = proj_dist;
    }

    let fog_factor = clamp((voxel_dist * 0.05) / 20.0, 0.0, 1.0);
    color = mix(color, sky_color, fog_factor);

    // 3. EFEITO PS1: BAYER DITHERING 4x4
    let dither_mat = array<f32, 16>(
        0.0 / 16.0, 8.0 / 16.0, 2.0 / 16.0, 10.0 / 16.0,
        12.0 / 16.0, 4.0 / 16.0, 14.0 / 16.0, 6.0 / 16.0,
        3.0 / 16.0, 11.0 / 16.0, 1.0 / 16.0, 9.0 / 16.0,
        15.0 / 16.0, 7.0 / 16.0, 13.0 / 16.0, 5.0 / 16.0
    );
    let dither_x = u32(in.clip_position.x) % 4u;
    let dither_y = u32(in.clip_position.y) % 4u;
    let dither_val = dither_mat[dither_y * 4u + dither_x] - 0.5;

    // 4. EFEITO PS1: CORES 5-BIT (Posterization) + Dithering
    color = color + vec3<f32>(dither_val * 0.1);
    color = floor(color * 32.0) / 32.0; // Corta para 32 tons de cor por canal

    return vec4<f32>(color, 1.0);
}