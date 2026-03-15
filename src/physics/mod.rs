// src/physics/mod.rs
use crate::world::TipoBloco;
use bevy::prelude::*;
use bevy::utils::HashMap;

// Algoritmo Minkowski Sweep (Esfera deslizando em AABBs)
pub fn resolver_colisao_minkowski(
    mapa: &HashMap<IVec3, TipoBloco>,
    pos: &mut Vec3,
    up: Vec3,
) -> bool {
    let radius = 0.35; // Raio da cápsula
    let altura = 1.0; // Distância entre a esfera base (pé) e a esfera topo (cabeça)
    let mut tocou_no_chao = false;

    let r = 2; // Área de busca
    let cx = pos.x.round() as i32;
    let cy = pos.y.round() as i32;
    let cz = pos.z.round() as i32;

    // Resolve colisões iterativamente para deslizar suavemente pelas quinas
    for _ in 0..3 {
        for x in -r..=r {
            for y in -r..=r {
                for z in -r..=r {
                    let b_pos = IVec3::new(cx + x, cy + y, cz + z);
                    if mapa.contains_key(&b_pos) {
                        let v_min = b_pos.as_vec3() - Vec3::splat(0.5);
                        let v_max = b_pos.as_vec3() + Vec3::splat(0.5);

                        // ====================================================
                        // 1. ESFERA DA BASE (PÉS)
                        // ====================================================
                        let p_base = *pos + up * radius;
                        let closest_base = p_base.clamp(v_min, v_max);
                        let dist_base = p_base.distance(closest_base);

                        if dist_base < radius {
                            let raw_push_dir = (p_base - closest_base).normalize_or_zero();

                            if raw_push_dir == Vec3::ZERO {
                                // O centro do jogador está DENTRO do bloco. 
                                // Verifica a direção real entre o centro do bloco e o jogador
                                let center_dir = (p_base - b_pos.as_vec3()).normalize_or_zero();
                                let dot_up = center_dir.dot(up);
                                
                                if dot_up > 0.5 { 
                                    // Chão verdadeiro: Ejeta pra cima
                                    *pos += up * 0.05;
                                    tocou_no_chao = true;
                                } else { 
                                    // Parede: Ejeta estritamente pro lado
                                    let push_side = center_dir - dot_up * up;
                                    *pos += push_side.normalize_or_zero() * 0.05;
                                }
                            } else {
                                let dot_up = raw_push_dir.dot(up);
                                let mut final_push = raw_push_dir;

                                // Tolerância extrema para não subir em paredes
                                let is_wall = dot_up.abs() < 0.98;

                                if is_wall {
                                    final_push -= dot_up * up;
                                    final_push = final_push.normalize_or_zero();
                                }

                                *pos += final_push * (radius - dist_base);

                                if dot_up > 0.85 {
                                    tocou_no_chao = true;
                                }
                            }
                        }

                        // ====================================================
                        // 2. ESFERA DO TOPO (CABEÇA)
                        // ====================================================
                        let p_topo = *pos + up * altura;
                        let closest_topo = p_topo.clamp(v_min, v_max);
                        let dist_topo = p_topo.distance(closest_topo);

                        if dist_topo < radius {
                            let raw_push_dir = (p_topo - closest_topo).normalize_or_zero();
                            
                            if raw_push_dir == Vec3::ZERO {
                                let center_dir = (p_topo - b_pos.as_vec3()).normalize_or_zero();
                                let dot_up = center_dir.dot(up);
                                
                                if dot_up < -0.5 { 
                                    // Bateu a cabeça no teto puro: Empurra pra baixo
                                    *pos -= up * 0.05; 
                                } else { 
                                    // Parede na altura do ombro/cabeça: Empurra pro lado
                                    let push_side = center_dir - dot_up * up;
                                    *pos += push_side.normalize_or_zero() * 0.05;
                                }
                            } else {
                                let dot_up = raw_push_dir.dot(up);
                                let mut final_push = raw_push_dir;

                                // A Cabeça AGORA TAMBÉM é impedida de agarrar na parede!
                                let is_wall = dot_up.abs() < 0.98;

                                if is_wall {
                                    final_push -= dot_up * up;
                                    final_push = final_push.normalize_or_zero();
                                }

                                *pos += final_push * (radius - dist_topo);
                            }
                        }
                    }
                }
            }
        }
    }
    tocou_no_chao
}