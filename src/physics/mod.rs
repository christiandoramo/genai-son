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
                        // Limites matemáticos do Voxel (AABB)
                        let v_min = b_pos.as_vec3() - Vec3::splat(0.5);
                        let v_max = b_pos.as_vec3() + Vec3::splat(0.5);

                        // Esfera da Base (Pés do jogador)
                        let p_base = *pos + up * radius;
                        let closest_base = p_base.clamp(v_min, v_max);
                        let dist_base = p_base.distance(closest_base);

                        // O Segredo de Minkowski: Se a distância for menor que o raio, empurra pra fora!
                        if dist_base < radius {
                            let mut raw_push_dir = (p_base - closest_base).normalize_or_zero();

                            if raw_push_dir == Vec3::ZERO {
                                raw_push_dir = up;
                                *pos += raw_push_dir * (radius + 0.1);
                                tocou_no_chao = true;
                            } else {
                                let dot_up = raw_push_dir.dot(up);
                                let mut final_push = raw_push_dir;


                                // Aumentamos a intolerância ao extremo: 0.98!
                                // O chão tem que ser quase perfeitamente plano, senão é considerado parede.
                                let is_wall = dot_up.abs() < 0.98;

                                if is_wall {
                                    // Zera completamente o vetor vertical. Você desliza na parede, mas NUNCA sobe!
                                    final_push -= dot_up * up;
                                    final_push = final_push.normalize_or_zero();
                                }

                                *pos += final_push * (radius - dist_base);

                                // Tem que estar muito apoiado em cima do bloco para contar como chão
                                if dot_up > 0.85 {
                                    tocou_no_chao = true;
                                }
                            }
                        }

                        // Esfera do Topo (Cabeça, para não varar os tetos)
                        let p_topo = *pos + up * altura;
                        let closest_topo = p_topo.clamp(v_min, v_max);
                        let dist_topo = p_topo.distance(closest_topo);

                        if dist_topo < radius {
                            let push_dir = (p_topo - closest_topo).normalize_or_zero();
                            if push_dir != Vec3::ZERO {
                                *pos += push_dir * (radius - dist_topo);
                            }
                        }
                    }
                }
            }
        }
    }
    tocou_no_chao
}
