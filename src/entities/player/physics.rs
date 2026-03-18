use super::Player;
use crate::entities::camera::math;
use winit::keyboard::KeyCode;

pub fn update_god_mode(player: &mut Player, dt: f32) {
    let speed = 80.0 * dt;
    
    let front = player.camera.get_front(); // Frente da mira exata
    let right = player.camera.get_right(); // Direita exata
    
    // A MÁGICA DO DRONE: O "Cima" agora é relativo à sua cabeça, 
    // ignorando completamente a gravidade ou o planeta!
    let up = math::normalize_or_zero(math::cross(right, front)); 
    
    let mut dir = [0.0, 0.0, 0.0];

    if player.keys[KeyCode::KeyW as usize] { dir[0] += front[0]; dir[1] += front[1]; dir[2] += front[2]; }
    if player.keys[KeyCode::KeyS as usize] { dir[0] -= front[0]; dir[1] -= front[1]; dir[2] -= front[2]; }
    
    if player.keys[KeyCode::KeyA as usize] { dir[0] -= right[0]; dir[1] -= right[1]; dir[2] -= right[2]; }
    if player.keys[KeyCode::KeyD as usize] { dir[0] += right[0]; dir[1] += right[1]; dir[2] += right[2]; }
    
    if player.keys[KeyCode::KeyE as usize] { dir[0] += up[0]; dir[1] += up[1]; dir[2] += up[2]; }
    if player.keys[KeyCode::KeyQ as usize] { dir[0] -= up[0]; dir[1] -= up[1]; dir[2] -= up[2]; }

    let dir_norm = math::normalize_or_zero(dir);
    player.camera.pos[0] += dir_norm[0] * speed;
    player.camera.pos[1] += dir_norm[1] * speed;
    player.camera.pos[2] += dir_norm[2] * speed;

    handle_shooting(player);
}

pub fn update_survival(player: &mut Player, dt: f32) {
    let center = [128.0, 128.0, 128.0];
    let rel_pos = [
        player.camera.pos[0] - center[0],
        player.camera.pos[1] - center[1],
        player.camera.pos[2] - center[2],
    ];

    let under_gravity = math::length(rel_pos) < 40.0 * 5.0;

    // 1. ZONAS ABSOLUTAS DE GRAVIDADE
    if under_gravity {
        let bias = 2.0;
        let abs_x = rel_pos[0].abs()
            + if player.physics_up[0].abs() > 0.5 {
                bias
            } else {
                0.0
            };
        let abs_y = rel_pos[1].abs()
            + if player.physics_up[1].abs() > 0.5 {
                bias
            } else {
                0.0
            };
        let abs_z = rel_pos[2].abs()
            + if player.physics_up[2].abs() > 0.5 {
                bias
            } else {
                0.0
            };

        player.physics_up = if abs_x >= abs_y && abs_x >= abs_z {
            [rel_pos[0].signum(), 0.0, 0.0]
        } else if abs_y >= abs_x && abs_y >= abs_z {
            [0.0, rel_pos[1].signum(), 0.0]
        } else {
            [0.0, 0.0, rel_pos[2].signum()]
        };
    }

    // 2. GRAVIDADE VISUAL SUAVE
    let target_visual_up = if under_gravity {
        let p_norm = math::normalize_or_zero(rel_pos);
        let p = 2.5_f32;
        math::normalize_or_zero([
            p_norm[0].abs().powf(p) * p_norm[0].signum(),
            p_norm[1].abs().powf(p) * p_norm[1].signum(),
            p_norm[2].abs().powf(p) * p_norm[2].signum(),
        ])
    } else {
        [0.0, 1.0, 0.0]
    };

    player.visual_up =
        math::normalize_or_zero(math::slerp(player.visual_up, target_visual_up, dt * 8.0));
    player.camera.reorient(player.visual_up);

    // 3. MOVIMENTAÇÃO WASD (Agora alinhado ao corpo, nunca apontando pro chão!)
    let speed = 15.0 * dt;
    let local_fwd = player.camera.local_forward; // Usa o corpo, não a cabeça
    let right = player.camera.get_right();
    let mut dir = [0.0, 0.0, 0.0];

    if player.keys[KeyCode::KeyW as usize] {
        dir[0] += local_fwd[0];
        dir[1] += local_fwd[1];
        dir[2] += local_fwd[2];
    }
    if player.keys[KeyCode::KeyS as usize] {
        dir[0] -= local_fwd[0];
        dir[1] -= local_fwd[1];
        dir[2] -= local_fwd[2];
    }
    if player.keys[KeyCode::KeyA as usize] {
        dir[0] -= right[0];
        dir[1] -= right[1];
        dir[2] -= right[2];
    }
    if player.keys[KeyCode::KeyD as usize] {
        dir[0] += right[0];
        dir[1] += right[1];
        dir[2] += right[2];
    }

    let dir_norm = math::normalize_or_zero(dir);
    let mut move_delta = [
        dir_norm[0] * speed,
        dir_norm[1] * speed,
        dir_norm[2] * speed,
    ];

    let dot_up = math::dot(move_delta, player.physics_up);
    move_delta[0] -= dot_up * player.physics_up[0];
    move_delta[1] -= dot_up * player.physics_up[1];
    move_delta[2] -= dot_up * player.physics_up[2];

  // 4. MOVE AND SLIDE (AABB com Step-Height para não raspar no chão)
    let mut next_pos = player.camera.pos;
    
    // Elevador virtual: Levanta o teste em 10cm na direção UP da gravidade
    let step_up = [
        player.physics_up[0] * 0.1, 
        player.physics_up[1] * 0.1, 
        player.physics_up[2] * 0.1
    ];

    // Tenta mover no Eixo X
    next_pos[0] += move_delta[0];
    let test_x = [next_pos[0] + step_up[0], next_pos[1] + step_up[1], next_pos[2] + step_up[2]];
    if is_colliding(player, test_x) { next_pos[0] -= move_delta[0]; } // Desliza no X

    // Tenta mover no Eixo Y
    next_pos[1] += move_delta[1];
    let test_y = [next_pos[0] + step_up[0], next_pos[1] + step_up[1], next_pos[2] + step_up[2]];
    if is_colliding(player, test_y) { next_pos[1] -= move_delta[1]; } // Desliza no Y

    // Tenta mover no Eixo Z
    next_pos[2] += move_delta[2];
    let test_z = [next_pos[0] + step_up[0], next_pos[1] + step_up[1], next_pos[2] + step_up[2]];
    if is_colliding(player, test_z) { next_pos[2] -= move_delta[2]; } // Desliza no Z

    player.camera.pos = next_pos;
    // 5. GRAVIDADE E PULO
    if under_gravity {
        player.velocidade_y -= 25.0 * dt;
    } else {
        player.velocidade_y *= 0.9;
    }

    let grav_delta = [
        player.physics_up[0] * player.velocidade_y * dt,
        player.physics_up[1] * player.velocidade_y * dt,
        player.physics_up[2] * player.velocidade_y * dt,
    ];

    next_pos = player.camera.pos;
    next_pos[0] += grav_delta[0];
    next_pos[1] += grav_delta[1];
    next_pos[2] += grav_delta[2];

    if is_colliding(player, next_pos) {
        player.velocidade_y = 0.0;
        player.no_chao = true;
    } else {
        player.camera.pos = next_pos;
        player.no_chao = false;
    }

    if player.keys[KeyCode::Space as usize] && player.no_chao {
        player.velocidade_y = 10.0;
        player.no_chao = false;
    }

    handle_shooting(player);
}

fn is_colliding(player: &Player, test_pos: [f32; 3]) -> bool {
    let radius = 0.25; // Reduzi levemente para deslizar melhor nas quinas

    // A MÁGICA DA BORDA: Usamos a `visual_up` (que faz a curva suavemente como uma esfera)
    // para orientar o seu corpo, evitando que a caixa colida com as quinas em 90 graus!
    let up = player.visual_up;

    let head = [
        test_pos[0] + up[0] * 0.2,
        test_pos[1] + up[1] * 0.2,
        test_pos[2] + up[2] * 0.2,
    ];
    let center = test_pos;
    let feet = [
        test_pos[0] - up[0] * 1.4,
        test_pos[1] - up[1] * 1.4,
        test_pos[2] - up[2] * 1.4,
    ];

    for p in [head, center, feet] {
        let min_x = (p[0] - radius).floor() as i32;
        let max_x = (p[0] + radius).ceil() as i32;
        let min_y = (p[1] - radius).floor() as i32;
        let max_y = (p[1] + radius).ceil() as i32;
        let min_z = (p[2] - radius).floor() as i32;
        let max_z = (p[2] + radius).ceil() as i32;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    // 1. CHECA AS MODIFICAÇÕES FEITAS PELO JOGADOR (Buracos e Blocos Novos)
                    if let Some(&voxel) = player.world_edits.get(&[x, y, z]) {
                        if voxel != 0 && voxel != 2 {
                            return true;
                        } // Se não for Ar (0) ou Água (2), colide!
                        continue; // Se for Ar, pula a checagem do ruído (O buraco existe!)
                    }

                    // 2. SE NÃO FOI MODIFICADO, CHECA O PLANETA MATEMÁTICO
                    if gpu_noise::is_voxel_solid(x, y, z) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

// O ESPELHO DO SHADER: A CPU simula o tiro para atualizar a própria física em O(1)
pub fn handle_shooting(player: &mut Player) {
    if player.is_shooting && player.cooldown <= 0.0 {
        let ro = player.camera.pos;
        let rd = player.camera.get_front();

        // PINCEL CREADOR (Coloca blocos)
        if player.active_weapon == crate::entities::player::Weapon::Creator {
            let center = [
                ro[0] + rd[0] * 10.0,
                ro[1] + rd[1] * 10.0,
                ro[2] + rd[2] * 10.0,
            ];
            for x in -3..=3 {
                for y in -3..=3 {
                    for z in -3..=3 {
                        let dist = ((x * x + y * y + z * z) as f32).sqrt();
                        if dist < 3.0 {
                            let vx = (center[0] + x as f32).round() as i32;
                            let vy = (center[1] + y as f32).round() as i32;
                            let vz = (center[2] + z as f32).round() as i32;
                            player
                                .world_edits
                                .insert([vx, vy, vz], player.selected_material);
                        }
                    }
                }
            }
            player.cooldown = 0.15;
        }
        // RAIO DE PLASMA (Escava buracos cilíndricos)
        else if player.active_weapon == crate::entities::player::Weapon::Plasma {
            // Faz um Raycast simples de 40 blocos para frente
            for step in 2..40 {
                let p = [
                    ro[0] + rd[0] * step as f32,
                    ro[1] + rd[1] * step as f32,
                    ro[2] + rd[2] * step as f32,
                ];
                for x in -1..=1 {
                    for y in -1..=1 {
                        for z in -1..=1 {
                            let vx = (p[0] + x as f32).round() as i32;
                            let vy = (p[1] + y as f32).round() as i32;
                            let vz = (p[2] + z as f32).round() as i32;
                            player.world_edits.insert([vx, vy, vz], 0); // "0" significa AR (Buraco)
                        }
                    }
                }
            }
            player.cooldown = 0.05;
        }
    }
}

// O ESPELHO EXATO DA GPU: Agora com suporte à Cavernas!
mod gpu_noise {
    fn fract(x: f32) -> f32 {
        x - x.floor()
    }
    fn hash(x: f32, y: f32, z: f32) -> f32 {
        let mut p3x = fract(x * 0.1031);
        let mut p3y = fract(y * 0.1031);
        let mut p3z = fract(z * 0.1031);
        let dot = p3x * (p3y + 33.33) + p3y * (p3z + 33.33) + p3z * (p3x + 33.33);
        p3x += dot;
        p3y += dot;
        p3z += dot;
        fract((p3x + p3y) * p3z) * 2.0 - 1.0
    }
    fn mix(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }
    pub fn noise_3d(x: f32, y: f32, z: f32) -> f32 {
        let px = x.floor();
        let py = y.floor();
        let pz = z.floor();
        let fx = fract(x);
        let fy = fract(y);
        let fz = fract(z);
        let ux = fx * fx * (3.0 - 2.0 * fx);
        let uy = fy * fy * (3.0 - 2.0 * fy);
        let uz = fz * fz * (3.0 - 2.0 * fz);
        let n000 = hash(px, py, pz);
        let n100 = hash(px + 1., py, pz);
        let n010 = hash(px, py + 1., pz);
        let n110 = hash(px + 1., py + 1., pz);
        let n001 = hash(px, py, pz + 1.);
        let n101 = hash(px + 1., py, pz + 1.);
        let n011 = hash(px, py + 1., pz + 1.);
        let n111 = hash(px + 1., py + 1., pz + 1.);
        mix(
            mix(mix(n000, n100, ux), mix(n010, n110, ux), uy),
            mix(mix(n001, n101, ux), mix(n011, n111, ux), uy),
            uz,
        )
    }

    // Testador Voxel exato!
    pub fn is_voxel_solid(x: i32, y: i32, z: i32) -> bool {
        if x < 0 || x > 255 || y < 0 || y > 255 || z < 0 || z > 255 {
            return false;
        }
        let px = x as f32 - 128.0;
        let py = y as f32 - 128.0;
        let pz = z as f32 - 128.0;
        let dist_base = (px * px * px * px + py * py * py * py + pz * pz * pz * pz)
            .sqrt()
            .sqrt();
        let planet_radius = 40.0;

        if dist_base > planet_radius + 40.0 {
            return false;
        }

        // Terreno Principal
        let dir = super::math::normalize_or_zero([px, py, pz]);
        let continentes = noise_3d(dir[0] * 1.2, dir[1] * 1.2, dir[2] * 1.2);
        let colinas = noise_3d(dir[0] * 3.0, dir[1] * 3.0, dir[2] * 3.0).max(0.0);
        let detalhes = noise_3d(dir[0] * 6.0, dir[1] * 6.0, dir[2] * 6.0).max(0.0);

        let mut altura = planet_radius + (continentes * 10.0);
        if continentes > -0.1 {
            altura += colinas * 12.0 + detalhes * 4.0;
        }
        let superficie = (altura / 2.0).round() * 2.0;

        if dist_base > superficie {
            return false;
        } // É ar limpo

        // CAVERNAS VOLUMÉTRICAS (Worms espalhados no subterrâneo)
        let cave_noise = noise_3d(px * 0.08, py * 0.08, pz * 0.08).abs();
        if cave_noise < 0.05 && dist_base > planet_radius - 15.0 {
            return false; // É ar dentro da caverna
        }

        true // É rocha maciça!
    }
}
