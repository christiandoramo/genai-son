// src/world/generator.rs
use bevy::prelude::*;
use noise::{NoiseFn, OpenSimplex};
use crate::player::Player;
use super::{VoxelWorld, ChunkManager, TipoBloco, PLANET_RADIUS, CHUNK_SIZE, RENDER_DISTANCE};
use super::mesher::construir_mesh_do_chunk;

pub fn calcular_bloco(x: f32, y: f32, z: f32, simplex: &OpenSimplex) -> Option<TipoBloco> {
    let pos = Vec3::new(x, y, z);
    let dist_base = (pos.x.powi(4) + pos.y.powi(4) + pos.z.powi(4)).powf(0.25);
    
    if dist_base > PLANET_RADIUS + 40.0 { return None; }

    let dir = pos.normalize_or_zero();
    let (nx, ny, nz) = (dir.x as f64, dir.y as f64, dir.z as f64);
    
    let base_altura = simplex.get([nx * 1.5, ny * 1.5, nz * 1.5]) * 22.0; 
    let mut modificador_relevo = base_altura;

    if base_altura > 0.0 { 
        modificador_relevo += simplex.get([nx * 4.0, ny * 4.0, nz * 4.0]) * 8.0;
        if base_altura > 5.0 { modificador_relevo += (simplex.get([nx * 8.0, ny * 8.0, nz * 8.0]).abs() * -1.0 + 0.5) * 25.0; }
    }

    modificador_relevo = (modificador_relevo / 2.0).round() * 2.0;
    
    let superficie = PLANET_RADIUS + modificador_relevo as f32;
    let nivel_mar = PLANET_RADIUS + 0.5;

    if dist_base <= superficie {
        let altitude = dist_base - PLANET_RADIUS;
        if altitude > 22.0 { return Some(TipoBloco::Neve); }
        if altitude > 8.0 { return Some(TipoBloco::Pedra); }
        if dist_base <= nivel_mar + 1.5 && modificador_relevo < 2.0 { return Some(TipoBloco::Areia); }
        if dist_base > superficie - 4.0 { return Some(TipoBloco::Grama); }
        return Some(TipoBloco::Nucleo);
    } else if dist_base <= nivel_mar { return Some(TipoBloco::Agua); }
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
    let Ok(player_transform) = player_query.get_single() else { return };
    let p_pos = player_transform.translation;
    
    let player_chunk = IVec3::new(
        (p_pos.x / CHUNK_SIZE as f32).floor() as i32,
        (p_pos.y / CHUNK_SIZE as f32).floor() as i32,
        (p_pos.z / CHUNK_SIZE as f32).floor() as i32,
    );

    let simplex = OpenSimplex::new(42);
    let mut chunks_na_area = Vec::new();

    // Forma de Cubo (Mais estável que esfera para o Greedy Mesher)
    for cx in -RENDER_DISTANCE..=RENDER_DISTANCE {
        for cy in -RENDER_DISTANCE..=RENDER_DISTANCE {
            for cz in -RENDER_DISTANCE..=RENDER_DISTANCE {
                chunks_na_area.push(player_chunk + IVec3::new(cx, cy, cz));
            }
        }
    }

    chunks_na_area.sort_by_key(|c| {
        let dx = c.x - player_chunk.x; let dy = c.y - player_chunk.y; let dz = c.z - player_chunk.z;
        dx*dx + dy*dy + dz*dz
    });

    // 1. DESCARREGAR LIXO VISUAL
    let mut chunks_a_remover = Vec::new();
    for chunk_pos in chunk_manager.meshes_ativos.keys() {
        if !chunks_na_area.contains(chunk_pos) { chunks_a_remover.push(*chunk_pos); }
    }
    for chunk_pos in chunks_a_remover {
        if let Some(entities) = chunk_manager.meshes_ativos.remove(&chunk_pos) {
            for entity in entities { commands.entity(entity).despawn(); }
        }
        chunk_manager.chunks_para_remesh.remove(&chunk_pos);
    }

    // 2. GERAR NA RAM (Até 4 por frame para não deixar você cair no vazio)
    let mut ram_gens = 0;
    for chunk_pos in &chunks_na_area {
        if !chunk_manager.chunks_gerados.contains(chunk_pos) {
            let start_x = chunk_pos.x * CHUNK_SIZE;
            let start_y = chunk_pos.y * CHUNK_SIZE;
            let start_z = chunk_pos.z * CHUNK_SIZE;

            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        let world_pos = IVec3::new(start_x + x, start_y + y, start_z + z);
                        if let Some(tipo) = calcular_bloco(world_pos.x as f32, world_pos.y as f32, world_pos.z as f32, &simplex) {
                            mundo.mapa.insert(world_pos, tipo);
                        }
                    }
                }
            }
            chunk_manager.chunks_gerados.insert(*chunk_pos);
            chunk_manager.chunks_para_remesh.insert(*chunk_pos);
            
            // Avisa os vizinhos que ganhamos blocos novos (Remove paredes internas de vidro)
            for dir in[IVec3::X, IVec3::NEG_X, IVec3::Y, IVec3::NEG_Y, IVec3::Z, IVec3::NEG_Z] {
                let viz = *chunk_pos + dir;
                if chunk_manager.chunks_gerados.contains(&viz) { chunk_manager.chunks_para_remesh.insert(viz); }
            }
            
            ram_gens += 1;
            if ram_gens >= 4 { break; } 
        }
    }

    // 3. A CURA DO CACHE FANTASMA: 
    // Se o chunk tá na RAM e na nossa Área, mas tá invisível, OBRIGA a desenhar!
    for chunk_pos in &chunks_na_area {
        if chunk_manager.chunks_gerados.contains(chunk_pos) && !chunk_manager.meshes_ativos.contains_key(chunk_pos) {
            chunk_manager.chunks_para_remesh.insert(*chunk_pos);
        }
    }

    // 4. DESENHAR POLÍGONOS (1 por frame para segurar o FPS)
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
            for entity in entities { commands.entity(entity).despawn(); }
        }
        let novas_entidades = construir_mesh_do_chunk(chunk_pos, &mundo, &mut commands, &mut meshes, &mut materials);
        chunk_manager.meshes_ativos.insert(chunk_pos, novas_entidades);
    }
}