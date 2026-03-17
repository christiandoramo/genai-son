// ATIRADOR DE ARMAS (Roda 1x por frame na thread master)
fn try_fire_weapons() {
    let action = uniforms.action;

    // Arma 6: Bazuca (Míssil Explosivo Rápido)
    if (action == 9u) {
        for (var i = 0u; i < 64u; i++) {
            if (projectiles[i].is_active == 0u) {
                projectiles[i].is_active = 1u; 
                projectiles[i].p_type = 1u; // 1 = Míssil
                projectiles[i].pos = uniforms.camera_pos + uniforms.camera_front * 2.0;
                projectiles[i].vel = uniforms.camera_front * 3.0; 
                break;
            }
        }
    }
    // Armas 1 a 4: Pistolas de Elementos (Água, Areia, Terra, Gás)
    else if (action > 0u && action < 8u && action != 4u && action != 6u) {
        // Dispara uma gota física do material selecionado
        for (var i = 0u; i < 64u; i++) {
            if (projectiles[i].is_active == 0u) {
                projectiles[i].is_active = 1u; 
                projectiles[i].p_type = 2u; // 2 = Estilhaço (Cai com a gravidade e vira bloco)
                projectiles[i].mat_id = action;
                projectiles[i].pos = uniforms.camera_pos + uniforms.camera_front * 2.0;
                // Velocidade menor para criar o efeito de esguicho/mangueira
                projectiles[i].vel = uniforms.camera_front * 1.5; 
                break;
            }
        }
    }
}

// PROCESSADOR DE RAIOS CONTÍNUOS (Roda em todas as threads espaciais)
fn process_player_weapons(index: u32, x: u32, y: u32, z: u32, voxel: u32) {
    // Arma 5: Raio de Plasma (Laser Instantâneo - Exceção que não usa projétil)
    if (uniforms.action == 8u) { 
        let p = vec3<f32>(f32(x), f32(y), f32(z));
        let p_to_cam = p - uniforms.camera_pos; 
        let proj = dot(p_to_cam, uniforms.camera_front); 
        if (proj > 2.0 && proj < 40.0 && length(p - (uniforms.camera_pos + uniforms.camera_front * proj)) < 2.0) { 
            if (voxel != 4u) { world.data[index] = 0u; } 
        }
    } 
}