@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read_write> world: WorldBuffer;
@group(0) @binding(2) var<storage, read_write> macro_world: WorldBuffer;

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
    
    var color: vec3<f32>;
    
    if (hit.hit) {
        // Cálculo simples de luz (Difusa + Sol)
        let sun_dir = normalize(vec3<f32>(sin(uniforms.time), cos(uniforms.time), 0.5));
        let diffuse = max(dot(get_normal(hit.side), sun_dir), 0.1);
        
        let base_color = get_voxel_color(hit.voxel_id);
        color = base_color * diffuse;
        
        // Névoa de Profundidade
        let fog = clamp(hit.dist / 400.0, 0.0, 1.0);
        color = mix(color, vec3<f32>(0.01, 0.01, 0.02), fog);
    } else {
        color = vec3<f32>(0.01, 0.01, 0.02); // Cor do Espaço
    }

    // Aplica o Filtro PSX no final
    let final_color = apply_psx_effects(color, frag_coord.xy);
    
    return vec4<f32>(final_color, 1.0);
}
// Para suportar o RayHit do dda.wgsl e o biome_color
fn get_normal(side: u32) -> vec3<f32> {
    if (side == 0u) { return vec3<f32>(1.0, 0.0, 0.0); }
    if (side == 1u) { return vec3<f32>(0.0, 1.0, 0.0); }
    return vec3<f32>(0.0, 0.0, 1.0);
}