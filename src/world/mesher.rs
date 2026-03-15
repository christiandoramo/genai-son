// src/world/mesher.rs
use super::{CHUNK_SIZE, TipoBloco, VoxelWorld};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

fn cor_do_bloco(tipo: TipoBloco) -> [f32; 4] {
    match tipo {
        TipoBloco::Grama => [0.2, 0.7, 0.2, 1.0],
        TipoBloco::Pedra => [0.4, 0.4, 0.45, 1.0],
        TipoBloco::Areia => [0.9, 0.8, 0.5, 1.0],
        TipoBloco::Nucleo => [0.2, 0.2, 0.2, 1.0],
        TipoBloco::Agua => [0.1, 0.4, 0.8, 0.6],
        TipoBloco::Neve => [0.95, 0.95, 1.0, 1.0],
    }
}

fn is_transparent(tipo: TipoBloco) -> bool {
    tipo == TipoBloco::Agua
}

#[derive(Default)]
struct ChunkMeshBuilder {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    colors: Vec<[f32; 4]>,
    indices: Vec<u32>,
}

impl ChunkMeshBuilder {
    fn add_quad(
        &mut self,
        v0: [f32; 3],
        v1: [f32; 3],
        v2: [f32; 3],
        v3: [f32; 3],
        n: [f32; 3],
        c: [f32; 4],
        reverse: bool,
    ) {
        let base = self.positions.len() as u32;
        self.positions.extend([v0, v1, v2, v3]);
        self.normals.extend([n, n, n, n]);
        self.colors.extend([c, c, c, c]);
        if reverse {
            self.indices
                .extend([base, base + 2, base + 1, base, base + 3, base + 2]);
        } else {
            self.indices
                .extend([base, base + 1, base + 2, base, base + 2, base + 3]);
        }
    }
    fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }
    fn build_mesh(self) -> Mesh {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, self.colors);
        mesh.insert_indices(Indices::U32(self.indices));
        mesh
    }
}

pub fn construir_mesh_do_chunk(
    chunk_pos: IVec3,
    mundo: &VoxelWorld,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Vec<Entity> {
// Voltamos ao padrão da indústria: as costas invisíveis dos polígonos não são renderizadas!
    let mat_opaque = materials.add(StandardMaterial { 
        base_color: Color::WHITE, 
        alpha_mode: AlphaMode::Opaque, 
        perceptual_roughness: 0.9, 
        ..default() 
    });
    
    let mat_transparent = materials.add(StandardMaterial { 
        base_color: Color::WHITE, 
        alpha_mode: AlphaMode::Blend, 
        perceptual_roughness: 0.1, 
        ..default() 
    });

    let cx = chunk_pos.x * CHUNK_SIZE;
    let cy = chunk_pos.y * CHUNK_SIZE;
    let cz = chunk_pos.z * CHUNK_SIZE;

    // =========================================================================
    // O SEGREDO 2: CACHE LINEAR 1D (Derruba o tempo de Meshing de 40ms para 2ms)
    // =========================================================================
    let p_size = CHUNK_SIZE + 2; // +2 para incluir a "casca" de vizinhos
    let mut cache_local = vec![None; (p_size * p_size * p_size) as usize];
    let start_x = cx - 1;
    let start_y = cy - 1;
    let start_z = cz - 1;

    // Fazemos apenas ~39k chamadas ao HashMap UMA VEZ para preencher o Cache
    for x in 0..p_size {
        for y in 0..p_size {
            for z in 0..p_size {
                let w_pos = IVec3::new(start_x + x, start_y + y, start_z + z);
                let idx = (z * p_size * p_size) + (y * p_size) + x;
                cache_local[idx as usize] = mundo.mapa.get(&w_pos).copied();
            }
        }
    }

    // Função interna super-rápida (Array indexing) para substituir o HashMap
    let get_cached = |w_pos: IVec3| -> Option<TipoBloco> {
        let lx = w_pos.x - start_x;
        let ly = w_pos.y - start_y;
        let lz = w_pos.z - start_z;
        let idx = (lz * p_size * p_size) + (ly * p_size) + lx;
        cache_local[idx as usize]
    };
    // =========================================================================

    let dirs = [
        (0, 1, IVec3::X, [1.0, 0.0, 0.0]),
        (0, -1, IVec3::NEG_X, [-1.0, 0.0, 0.0]),
        (1, 1, IVec3::Y, [0.0, 1.0, 0.0]),
        (1, -1, IVec3::NEG_Y, [0.0, -1.0, 0.0]),
        (2, 1, IVec3::Z, [0.0, 0.0, 1.0]),
        (2, -1, IVec3::NEG_Z, [0.0, 0.0, -1.0]),
    ];

    let mut b_opaque = ChunkMeshBuilder::default();
    let mut b_transp = ChunkMeshBuilder::default();

    for &(d, sign, dir_vec, normal) in &dirs {
        let u = (d + 1) % 3;
        let v = (d + 2) % 3;
        for slice in 0..CHUNK_SIZE {
            let mut mask = vec![None; (CHUNK_SIZE * CHUNK_SIZE) as usize];
            for j in 0..CHUNK_SIZE {
                for i in 0..CHUNK_SIZE {
                    let mut pos = IVec3::ZERO;
                    pos[d] = slice;
                    pos[u] = i;
                    pos[v] = j;
                    let world_pos = IVec3::new(cx, cy, cz) + pos;

                    // Lendo do nosso Cache ultrarrápido ao invés do HashMap!
                    let b_current = get_cached(world_pos);
                    let b_neighbor = get_cached(world_pos + dir_vec);

                    if let Some(t_curr) = b_current {
                        let should_draw = if is_transparent(t_curr) {
                            b_neighbor.is_none()
                        } else {
                            b_neighbor.map_or(true, is_transparent)
                        };
                        if should_draw {
                            mask[(j * CHUNK_SIZE + i) as usize] = Some(t_curr);
                        }
                    }
                }
            }

            let mut j = 0;
            while j < CHUNK_SIZE {
                let mut i = 0;
                while i < CHUNK_SIZE {
                    if let Some(tipo) = mask[(j * CHUNK_SIZE + i) as usize] {
                        let mut width = 1;
                        while i + width < CHUNK_SIZE
                            && mask[(j * CHUNK_SIZE + i + width) as usize] == Some(tipo)
                        {
                            width += 1;
                        }
                        let mut height = 1;
                        'outer: while j + height < CHUNK_SIZE {
                            for w in 0..width {
                                if mask[((j + height) * CHUNK_SIZE + i + w) as usize] != Some(tipo)
                                {
                                    break 'outer;
                                }
                            }
                            height += 1;
                        }
                        for h in 0..height {
                            for w in 0..width {
                                mask[((j + h) * CHUNK_SIZE + i + w) as usize] = None;
                            }
                        }

                        let color = cor_do_bloco(tipo);
                        let offset_d = if sign == 1 { 0.5 } else { -0.5 };
                        let mut p0 = [0.0; 3];
                        let mut p1 = [0.0; 3];
                        let mut p2 = [0.0; 3];
                        let mut p3 = [0.0; 3];
                        let bases = [cx as f32, cy as f32, cz as f32];
                        p0[d] = bases[d] + slice as f32 + offset_d;
                        p1[d] = p0[d];
                        p2[d] = p0[d];
                        p3[d] = p0[d];
                        p0[u] = bases[u] + i as f32 - 0.5;
                        p0[v] = bases[v] + j as f32 - 0.5;
                        p1[u] = bases[u] + (i + width) as f32 - 0.5;
                        p1[v] = bases[v] + j as f32 - 0.5;
                        p2[u] = bases[u] + (i + width) as f32 - 0.5;
                        p2[v] = bases[v] + (j + height) as f32 - 0.5;
                        p3[u] = bases[u] + i as f32 - 0.5;
                        p3[v] = bases[v] + (j + height) as f32 - 0.5;

                        let reverse_winding = sign == -1;
                        if is_transparent(tipo) {
                            b_transp.add_quad(p0, p1, p2, p3, normal, color, reverse_winding);
                        } else {
                            b_opaque.add_quad(p0, p1, p2, p3, normal, color, reverse_winding);
                        }
                        i += width;
                    } else {
                        i += 1;
                    }
                }
                j += 1;
            }
        }
    }

    let mut spawnadas = Vec::new();
    if !b_opaque.is_empty() {
        spawnadas.push(
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(b_opaque.build_mesh()),
                    material: mat_opaque,
                    ..default()
                })
                .id(),
        );
    }
    if !b_transp.is_empty() {
        spawnadas.push(
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(b_transp.build_mesh()),
                    material: mat_transparent,
                    ..default()
                })
                .id(),
        );
    }
    spawnadas
}
