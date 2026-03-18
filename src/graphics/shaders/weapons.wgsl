// ATIRADOR DE ARMAS BALÍSTICAS (Roda 1x por frame na thread master)
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
    // Obs: As armas 1 a 4 agora são processadas diretamente no espaço (pincel), 
    // não precisam mais disparar projéteis aqui.
}

// PROCESSADOR DE RAIOS E PINCÉIS (Roda em todas as threads espaciais do Compute Shader)
fn process_player_weapons(index: u32, x: u32, y: u32, z: u32, voxel: u32) {
    let action = uniforms.action;
    let p = vec3<f32>(f32(x), f32(y), f32(z));

    // Arma 5: Raio de Plasma (Laser Instantâneo - Cava cilindros)
if (action == 8u) { 
        let p_to_cam = p - uniforms.camera_pos; 

        
        let proj = dot(p_to_cam, uniforms.camera_front); 
        if (proj > 2.0 && proj < 40.0 && length(p - (uniforms.camera_pos + uniforms.camera_front * proj)) < 2.0) { 
           if (voxel != 10u) { world.data[index] = 0u; }// núcleo blindado!
        }
    }
    // Armas 1 a 4: Creator (Pincel de Criação Mágica - Pinta blocos/fluidos direto no ar)
    else if (action > 0u && action < 8u && action != 4u && action != 6u) {
        // Pinta num raio de 3 blocos à uma distância de 10 blocos da câmera
        if (length(p - (uniforms.camera_pos + uniforms.camera_front * 10.0)) < 3.0) { 
            // Só pinta onde é espaço vazio (Ar)
            if (voxel == 0u) {
                world.data[index] = action; 
                macro_world.data[get_macro_index(x,y,z)] = 1u; 
            }
        }
    }
}