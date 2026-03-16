// src/world/generator.rs
use super::mesher::construir_mesh_do_chunk;
use super::{CHUNK_SIZE, ChunkManager, PLANET_RADIUS, RENDER_DISTANCE, TipoBloco, VoxelWorld};
use crate::player::Player;
use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use futures_lite::future;
use noise::{NoiseFn, OpenSimplex};

pub fn calcular_bloco(x: f32, y: f32, z: f32, simplex: &OpenSimplex) -> Option<TipoBloco> {
    let pos = Vec3::new(x, y, z);
    let dist_base = (pos.x.powi(4) + pos.y.powi(4) + pos.z.powi(4)).powf(0.25);

    if dist_base > PLANET_RADIUS + 40.0 {
        return None;
    }

    let dir = pos.normalize_or_zero();
    let (nx, ny, nz) = (dir.x as f64, dir.y as f64, dir.z as f64);

    // =========================================================================
    // BIOMA REFINADO: Sem covas profundas!
    // =========================================================================
    let continentes = simplex.get([nx * 1.2, ny * 1.2, nz * 1.2]) as f32;

    // Usamos ".max(0.0)" para garantir que colinas SÓ SOBEM, nunca cavam buracos!
    let colinas = (simplex.get([nx * 3.0, ny * 3.0, nz * 3.0]) as f32).max(0.0);

    // Detalhes mais suaves para evitar cubos soltos e pontiagudos
    let detalhes = (simplex.get([nx * 6.0, ny * 6.0, nz * 6.0]) as f32).max(0.0);

    let mut altura_superficie = PLANET_RADIUS + (continentes * 10.0);

    // Terra firme ganha as elevações extras
    if continentes > -0.1 {
        altura_superficie += colinas * 12.0;
        altura_superficie += detalhes * 4.0;
    }

    let superficie = (altura_superficie / 2.0).round() * 2.0;
    let nivel_mar = PLANET_RADIUS + 0.0;

    if dist_base <= superficie {
        let profundidade = superficie - dist_base;

        if continentes < -0.2 && dist_base <= nivel_mar + 1.5 {
            return Some(TipoBloco::Areia);
        } else if continentes > 0.4 && superficie > PLANET_RADIUS + 15.0 {
            if profundidade < 1.0 {
                return Some(TipoBloco::Neve);
            }
            return Some(TipoBloco::Pedra);
        } else {
            if profundidade < 1.0 {
                return Some(TipoBloco::Grama);
            }
            if profundidade < 3.0 {
                return Some(TipoBloco::Areia);
            }
            return Some(TipoBloco::Pedra);
        }
    } else if dist_base <= nivel_mar {
        return Some(TipoBloco::Agua);
    }

    None
}

pub fn gerenciar_chunks(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut mundo: ResMut<VoxelWorld>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let p_pos = player_transform.translation;
    let player_chunk = IVec3::new(
        (p_pos.x / CHUNK_SIZE as f32).floor() as i32,
        (p_pos.y / CHUNK_SIZE as f32).floor() as i32,
        (p_pos.z / CHUNK_SIZE as f32).floor() as i32,
    );

    let mut chunks_na_area = Vec::new();

    // VOLTAMOS AO PADRÃO ESCALÁVEL: Geração ao redor do Jogador!
    for cx in -RENDER_DISTANCE..=RENDER_DISTANCE {
        for cy in -RENDER_DISTANCE..=RENDER_DISTANCE {
            for cz in -RENDER_DISTANCE..=RENDER_DISTANCE {
                chunks_na_area.push(player_chunk + IVec3::new(cx, cy, cz));
            }
        }
    }

    // ===============================================================
    // O SEGREDO DO SPAWN INSTANTÂNEO
    // Em vez de gerar os blocos de ar ao redor do jogador no céu,
    // priorizamos o CHÃO que está exatamente abaixo dele!
    // ===============================================================
    let dir_pro_nucleo = p_pos.normalize_or_zero();
    let pos_chao = dir_pro_nucleo * PLANET_RADIUS;
    let chunk_chao = IVec3::new(
        (pos_chao.x / CHUNK_SIZE as f32).floor() as i32,
        (pos_chao.y / CHUNK_SIZE as f32).floor() as i32,
        (pos_chao.z / CHUNK_SIZE as f32).floor() as i32,
    );

    chunks_na_area.sort_by_key(|c| {
        // Agora a engine ordena pela distância do CHÃO, não da câmera!
        let dx = c.x - chunk_chao.x;
        let dy = c.y - chunk_chao.y;
        let dz = c.z - chunk_chao.z;
        dx * dx + dy * dy + dz * dz
    });

    // 1. DESCARREGAR LIXO VISUAL E CANCELAR TAREFAS ANTIGAS
    let mut chunks_a_remover = Vec::new();
    for chunk_pos in chunk_manager.meshes_ativos.keys() {
        if !chunks_na_area.contains(chunk_pos) {
            chunks_a_remover.push(*chunk_pos);
        }
    }
    for chunk_pos in chunks_a_remover {
        if let Some(entities) = chunk_manager.meshes_ativos.remove(&chunk_pos) {
            for entity in entities {
                commands.entity(entity).despawn();
            }
        }
        chunk_manager.chunks_para_remesh.remove(&chunk_pos);
    }

    let mut tarefas_a_remover = Vec::new();
    for chunk_pos in chunk_manager.tarefas_geracao.keys() {
        if !chunks_na_area.contains(chunk_pos) {
            tarefas_a_remover.push(*chunk_pos);
        }
    }
    for chunk_pos in tarefas_a_remover {
        chunk_manager.tarefas_geracao.remove(&chunk_pos);
    }

    // 2. DISPARAR GERAÇÃO EM BACKGROUND (COM FREIO DE EMERGÊNCIA)
    let thread_pool = AsyncComputeTaskPool::get();
    let mut tasks_disparadas_neste_frame = 0;

    for chunk_pos in &chunks_na_area {
        if !chunk_manager.chunks_gerados.contains(chunk_pos)
            && !chunk_manager.tarefas_geracao.contains_key(chunk_pos)
        {
            let c_pos = *chunk_pos;

            // OTIMIZAÇÃO EXTREMA: Se o Chunk está no espaço sideral profundo,
            // nem manda pra Thread! Marca como vazio instantaneamente.
            let centro_chunk = c_pos.as_vec3() * CHUNK_SIZE as f32;
            if centro_chunk.length() > PLANET_RADIUS + 80.0 {
                chunk_manager.chunks_gerados.insert(c_pos);
                continue; // Pula sem gastar processamento!
            }

            let task = thread_pool.spawn(async move {
                let simplex = OpenSimplex::new(42);
                let mut blocos_gerados = Vec::new();
                let start_x = c_pos.x * CHUNK_SIZE;
                let start_y = c_pos.y * CHUNK_SIZE;
                let start_z = c_pos.z * CHUNK_SIZE;

                for x in 0..CHUNK_SIZE {
                    for y in 0..CHUNK_SIZE {
                        for z in 0..CHUNK_SIZE {
                            let world_pos = IVec3::new(start_x + x, start_y + y, start_z + z);
                            if let Some(tipo) = calcular_bloco(
                                world_pos.x as f32,
                                world_pos.y as f32,
                                world_pos.z as f32,
                                &simplex,
                            ) {
                                blocos_gerados.push((world_pos, tipo));
                            }
                        }
                    }
                }
                blocos_gerados
            });

            chunk_manager.tarefas_geracao.insert(c_pos, task);

            tasks_disparadas_neste_frame += 1;
            if tasks_disparadas_neste_frame >= 6 {
                break;
            } // Limite gentil para a CPU
        }
    }
    // 3. COLETAR RESULTADOS
    let mut chunks_concluidos = Vec::new();
    for (chunk_pos, task) in chunk_manager.tarefas_geracao.iter_mut() {
        if let Some(blocos) = future::block_on(future::poll_once(task)) {
            chunks_concluidos.push((*chunk_pos, blocos));
            break;
        }
    }

    for (chunk_pos, blocos) in chunks_concluidos {
        chunk_manager.tarefas_geracao.remove(&chunk_pos);
        for (w_pos, tipo) in blocos {
            mundo.mapa.insert(w_pos, tipo);
        }
        chunk_manager.chunks_gerados.insert(chunk_pos);
        chunk_manager.chunks_para_remesh.insert(chunk_pos);

        for dir in [
            IVec3::X,
            IVec3::NEG_X,
            IVec3::Y,
            IVec3::NEG_Y,
            IVec3::Z,
            IVec3::NEG_Z,
        ] {
            let viz = chunk_pos + dir;
            if chunk_manager.chunks_gerados.contains(&viz) {
                chunk_manager.chunks_para_remesh.insert(viz);
            }
        }
    }

    // 4. CURA DO CACHE
    for chunk_pos in &chunks_na_area {
        if chunk_manager.chunks_gerados.contains(chunk_pos)
            && !chunk_manager.meshes_ativos.contains_key(chunk_pos)
        {
            chunk_manager.chunks_para_remesh.insert(*chunk_pos);
        }
    }

    // 5. DESENHAR
    let mut chunk_to_mesh = None;
    for c in &chunks_na_area {
        if chunk_manager.chunks_para_remesh.contains(c) {
            chunk_to_mesh = Some(*c);
            break;
        }
    }

    if let Some(chunk_pos) = chunk_to_mesh {
        chunk_manager.chunks_para_remesh.remove(&chunk_pos);
        if let Some(entities) = chunk_manager.meshes_ativos.remove(&chunk_pos) {
            for entity in entities {
                commands.entity(entity).despawn();
            }
        }
        let novas_entidades = construir_mesh_do_chunk(
            chunk_pos,
            &mundo,
            &mut commands,
            &mut meshes,
            &mut materials,
        );
        chunk_manager
            .meshes_ativos
            .insert(chunk_pos, novas_entidades);
    }
}
