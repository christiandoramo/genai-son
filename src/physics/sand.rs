// src/physics/sand.rs
use crate::world::VoxelWorld;
use bevy::prelude::*;

#[derive(Component)]
pub struct ParticulaAreia {
    pub velocidade: Vec3,
    pub tempo_vida: Timer,
    pub dormindo: bool, // <-- A variável salvadora de CPUs voltou!
}

pub fn atualizar_particulas_areia(
    mut commands: Commands,
    time: Res<Time>,
    mundo: Res<VoxelWorld>,
    mut query: Query<(Entity, &mut Transform, &mut ParticulaAreia)>,
) {
    let dt = time.delta_seconds().min(0.05);
    let raio = 0.1;
    let diametro = 0.2;
    let diametro_sq = diametro * diametro; // Evita usar raiz quadrada (economia de CPU)

    // O(1) Cache: Coletamos todos os grãos para eles se empurrarem
    let mut posicoes: Vec<(Entity, Vec3)> =
        query.iter().map(|(e, t, _)| (e, t.translation)).collect();

    for (entity, mut transform, mut particula) in query.iter_mut() {
        particula.tempo_vida.tick(time.delta());

        if particula.tempo_vida.just_finished() {
            commands.entity(entity).despawn();
            continue;
        }

        // Se ele empilhou direitinho, ele dorme. Zero lag, zero tremedeira!
        if particula.dormindo {
            continue;
        }

        let pos_atual = transform.translation;

        let abs_x = pos_atual.x.abs();
        let abs_y = pos_atual.y.abs();
        let abs_z = pos_atual.z.abs();
        let up = if abs_x > abs_y && abs_x > abs_z {
            Vec3::new(pos_atual.x.signum(), 0.0, 0.0)
        } else if abs_y > abs_x && abs_y > abs_z {
            Vec3::new(0.0, pos_atual.y.signum(), 0.0)
        } else {
            Vec3::new(0.0, 0.0, pos_atual.z.signum())
        };

        // Gravidade
        particula.velocidade -= up * 18.0 * dt;
        let mut nova_pos = pos_atual + particula.velocidade * dt;

        // 1. COLISÃO COM O MUNDO (Impede de afundar)
        let coord = IVec3::new(
            nova_pos.x.round() as i32,
            nova_pos.y.round() as i32,
            nova_pos.z.round() as i32,
        );
        if mundo.mapa.contains_key(&coord) {
            let closest = nova_pos.clamp(
                coord.as_vec3() - Vec3::splat(0.5),
                coord.as_vec3() + Vec3::splat(0.5),
            );
            let dist = nova_pos.distance(closest);

            if dist < raio {
                let mut push = (nova_pos - closest).normalize_or_zero();
                if push == Vec3::ZERO {
                    push = up;
                }

                nova_pos += push * (raio - dist + 0.01);

                // Fricção extrema: bateu no mundo Voxel, escorrega muito pouco
                // Separamos o cálculo na variável `dot_vel` para o Rust não dar erro de Borrow
                let dot_vel = particula.velocidade.dot(push);
                particula.velocidade -= dot_vel * push;
                particula.velocidade *= 0.3;
            }
        }

        // 2. COLISÃO ENTRE GRÃOS (O Efeito Empilhamento)
        for (outra_entity, outra_pos) in &posicoes {
            if entity == *outra_entity {
                continue;
            }

            let dist_vec = nova_pos - *outra_pos;
            let dist_sq = dist_vec.length_squared();

            // Se estão encostando...
            if dist_sq > 0.0001 && dist_sq < diametro_sq {
                let dist = dist_sq.sqrt();
                let overlap = diametro - dist;
                let push_dir = dist_vec / dist;

                nova_pos += push_dir * overlap * 0.6; // Empurrão rígido
                particula.velocidade *= 0.5; // Absorve o impacto
            }
        }

        // 3. A MÁGICA DA OTIMIZAÇÃO VIRTUAL
        // Se a velocidade ficou baixinha, o grão "congela" e alinha ao Grid!
        if particula.velocidade.length_squared() < 0.2 {
            particula.dormindo = true;

            // Snap: Força a posição arredondar para múltiplos de 0.2m (Tamanho do voxelzinho)
            nova_pos = (nova_pos / diametro).round() * diametro;
        }

        if let Some(pos) = posicoes.iter_mut().find(|(e, _)| *e == entity) {
            pos.1 = nova_pos;
        }
        transform.translation = nova_pos;
    }
}
