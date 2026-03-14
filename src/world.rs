use bevy::prelude::*;
use bevy::utils::HashMap;
use noise::{NoiseFn, Perlin};

pub const GRID_SIZE: i32 = 40;

#[derive(Clone, Copy, PartialEq)]
pub enum TipoBloco { Grama, Pedra, Agua }

#[derive(Resource, Default)]
pub struct VoxelWorld {
    pub mapa: HashMap<IVec3, TipoBloco>, // Dicionário de colisões instantâneas
}

pub fn gerar_mundo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut mundo: ResMut<VoxelWorld>,
) {
    let mat_agua = materials.add(StandardMaterial { base_color: Color::srgb(0.1, 0.4, 0.8), ..default() });
    let mat_grama = materials.add(StandardMaterial { base_color: Color::srgb(0.2, 0.8, 0.2), ..default() });
    let mat_pedra = materials.add(StandardMaterial { base_color: Color::srgb(0.5, 0.5, 0.5), ..default() });
    
    let mesh_cubo = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight { illuminance: 12000.0, shadows_enabled: true, ..default() },
        transform: Transform::from_xyz(20.0, 40.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    let perlin = Perlin::new(42);

    for x in -GRID_SIZE..=GRID_SIZE {
        for z in -GRID_SIZE..=GRID_SIZE {
            let nx = x as f64 * 0.05;
            let nz = z as f64 * 0.05;
            let mut y = (perlin.get([nx, nz]) * 8.0).round() as i32;

            if y < -2 { y = -2; } // Planifica o fundo do mar

            let tipo = if y <= -2 { TipoBloco::Agua } else if y < 3 { TipoBloco::Grama } else { TipoBloco::Pedra };
            let material = match tipo {
                TipoBloco::Agua => mat_agua.clone(),
                TipoBloco::Grama => mat_grama.clone(),
                TipoBloco::Pedra => mat_pedra.clone(),
            };

            mundo.mapa.insert(IVec3::new(x, y, z), tipo);

            commands.spawn(PbrBundle {
                mesh: mesh_cubo.clone(),
                material,
                transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                ..default()
            });
        }
    }
}