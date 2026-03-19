fn calculate_lighting(hit: RayHit, ro: vec3<f32>, rd: vec3<f32>, time: f32, sky_color: vec3<f32>) -> vec3<f32> {
        
    var base_color = get_biome_color(hit.voxel_id, hit.dist);
    let normal = get_normal(hit.side);
    
    // 1. ANIMAÇÃO DE FLUIDOS (Ondas e Brilho)
    if (hit.voxel_id == MAT_WATER) {
        let wave = sin(time * 3.0 + hit.pos.x * 1.5) * cos(time * 2.0 + hit.pos.z * 1.5);
        base_color += wave * 0.1;
    }
    if (hit.voxel_id == MAT_MAGMA) {
        let pulse = sin(time * 5.0 + hit.pos.x * 2.0) * 0.2;
        return base_color + pulse; // Magma é emissivo, ignora sombra
    }

    // 2. SOL DINÂMICO
    let sun_dir = normalize(vec3<f32>(sin(time), cos(time), 0.5));
    let diffuse = max(dot(normal, sun_dir), 0.1);
    var final_light = base_color * diffuse;

    // 3. LANTERNA TÁTICA
    if (uniforms.flashlight_on == 1u) {
        let to_hit = normalize(hit.pos - ro);
        let spot = dot(to_hit, uniforms.camera_front);
        if (spot > 0.85) {
            let falloff = smoothstep(0.85, 0.98, spot);
            let dist_fade = smoothstep(60.0, 2.0, hit.dist);
            final_light += vec3<f32>(1.0, 0.95, 0.8) * (falloff * dist_fade * 0.4);
        }
    }

    // 4. FOG DE DISTÂNCIA (Nostalgia N64)
    // let fog_factor = clamp(hit.dist / 250.0, 0.0, 1.0);
    // let sky_color = vec3<f32>(0.01, 0.01, 0.02); // Espaço profundo
    
    return final_light;// mix(final_light, sky_color, fog_factor);
}