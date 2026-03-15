// src/world/mod.rs
use bevy::prelude::*;
use bevy::tasks::Task;
use bevy::utils::{HashMap, HashSet};

mod generator;
mod mesher;

pub const PLANET_RADIUS: f32 = 80.0; 
pub const CHUNK_SIZE: i32 = 32;
pub const RENDER_DISTANCE: i32 = 2;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TipoBloco { Grama, Pedra, Areia, Agua, Nucleo, Neve }

#[derive(Resource, Default)]
pub struct VoxelWorld {
    pub mapa: HashMap<IVec3, TipoBloco>,
}

#[derive(Resource, Default)]
pub struct ChunkManager {
    pub chunks_gerados: HashSet<IVec3>,
    pub meshes_ativos: HashMap<IVec3, Vec<Entity>>,
    pub chunks_para_remesh: HashSet<IVec3>, 
    // NOVO: Guarda as tarefas de multithreading que estão rodando na CPU no momento
    pub tarefas_geracao: HashMap<IVec3, Task<Vec<(IVec3, TipoBloco)>>>,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VoxelWorld>()
           .init_resource::<ChunkManager>()
           .add_systems(Update, generator::gerenciar_chunks); 
    }
}