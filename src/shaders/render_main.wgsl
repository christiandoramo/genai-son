@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read_write> world: WorldBuffer;
@group(0) @binding(2) var<storage, read_write> macro_world: WorldBuffer;
@group(0) @binding(3) var<storage, read_write> projectiles: array<Projectile>;

@fragment
fn fs_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    // Resolução Retro (320x240)
    let retro_res = vec2<f32>(320.0, 240.0);
    var uv = (frag_coord.xy / uniforms.resolution) * retro_res;
    uv = floor(uv) / retro_res;
    uv = uv * 2.0 - 1.0;
    uv.x *= uniforms.resolution.x / uniforms.resolution.y;

    // Setup da Câmera (Alinhada ao UP visual do planeta)
    let ro = uniforms.camera_pos;
    let forward = normalize(uniforms.camera_front);
    let right = normalize(cross(uniforms.camera_up, forward));
    let up = cross(forward, right);
    let rd = normalize(forward + uv.x * right + uv.y * up);

    // Executa o Raycast
       let hit = cast_ray(ro, rd, 500.0);
    
    // Céu e Ciclo
    let sky_day = vec3<f32>(0.3, 0.5, 0.8);
    let sky_night = vec3<f32>(0.01, 0.01, 0.02);
    let sky_col = mix(sky_day, sky_night, clamp(uniforms.time / 3.14159, 0.0, 1.0));
    
    var color: vec3<f32>;
    var final_dist = 9999.0;
    
    if (hit.hit) {
        color = calculate_lighting(hit, ro, rd, uniforms.time, sky_col);
        final_dist = hit.dist;
    } else {
        color = sky_col;
    }

    // Míssil da Bazuca visível e ardente
    var proj_hit = false; var proj_dist = 9999.0;
    for (var i = 0u; i < 64u; i++) {
        if (projectiles[i].is_active == 1u) {
            let oc = ro - projectiles[i].pos;
            let b = dot(oc, rd); 
            let c = dot(oc, oc) - 0.3 * 0.3; 
            let h_val = b * b - c;
            if (h_val > 0.0) {
                let t = -b - sqrt(h_val);
                if (t > 0.0 && t < proj_dist) { proj_dist = t; proj_hit = true; }
            }
        }
    }
    if (proj_hit && proj_dist < final_dist) {
        color = vec3<f32>(1.0, 0.4, 0.1); // Míssil Laranja Brilhante
        final_dist = proj_dist;
    }

    // Névoa
    let fog = clamp(final_dist / 400.0, 0.0, 1.0);
    color = mix(color, sky_col, fog);

    var final_color = apply_psx_effects(color, frag_coord.xy);
    
    // Crosshair (Mira elegante no centro da tela)
    let center_dist = max(abs(frag_coord.x - uniforms.resolution.x * 0.5), abs(frag_coord.y - uniforms.resolution.y * 0.5));
    if (center_dist < 4.0 && center_dist > 1.0) { final_color = mix(final_color, vec3<f32>(1.0, 1.0, 1.0), 0.8); }
    
    return vec4<f32>(final_color, 1.0);
}
// Para suportar o RayHit do dda.wgsl e o biome_color
fn get_normal(side: u32) -> vec3<f32> {
    if (side == 0u) { return vec3<f32>(1.0, 0.0, 0.0); }
    if (side == 1u) { return vec3<f32>(0.0, 1.0, 0.0); }
    return vec3<f32>(0.0, 0.0, 1.0);
}