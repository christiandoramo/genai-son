use bevy::prelude::*;
use bevy::utils::HashMap;
use noise::{NoiseFn, OpenSimplex};

pub const PLANET_RADIUS: f32 = 80.0; // PLANETA GIGANTE! Curvatura muito mais natural.

#[derive(Clone, Copy, PartialEq)]
pub enum TipoBloco { Grama, Pedra, Neve, Agua, Nucleo }

#[derive(Resource, Default)]
pub struct VoxelWorld {
    pub mapa: HashMap<IVec3, TipoBloco>,
}

pub fn gerar_mundo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut mundo: ResMut<VoxelWorld>,
) {
    let mat_agua = materials.add(StandardMaterial { base_color: Color::srgb(0.1, 0.4, 0.8), perceptual_roughness: 0.1, ..default() });
    let mat_grama = materials.add(StandardMaterial { base_color: Color::srgb(0.2, 0.7, 0.2), ..default() });
    let mat_pedra = materials.add(StandardMaterial { base_color: Color::srgb(0.4, 0.4, 0.45), ..default() });
    let mat_neve = materials.add(StandardMaterial { base_color: Color::srgb(0.9, 0.9, 1.0), ..default() });
    let mat_nucleo = materials.add(StandardMaterial { base_color: Color::srgb(1.0, 0.3, 0.0), ..default() });
    
    let mesh_cubo = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight { illuminance: 15000.0, shadows_enabled: true, ..default() },
        transform: Transform::from_xyz(100.0, 150.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    let simplex = OpenSimplex::new(42);
    let limite = (PLANET_RADIUS + 40.0) as i32; // Limite acomoda montanhas gigantes

    for x in -limite..=limite {
        for y in -limite..=limite {
            for z in -limite..=limite {
                let pos = Vec3::new(x as f32, y as f32, z as f32);
                let dist = pos.length();
                if dist > limite as f32 { continue; } // Pula o ar vazio do espaço para economizar CPU

                let nx = x as f64 * 0.015; 
                let ny = y as f64 * 0.015; 
                let nz = z as f64 * 0.015;
                
                // RUÍDO FRACTAL ÉPICO
                // 1. Continentes e Oceanos profundos
                let continentes = simplex.get([nx, ny, nz]) * 20.0;
                
                // 2. Cadeias de Montanhas (abs() cria picos pontiagudos)
                let montanhas = simplex.get([nx * 4.0, ny * 4.0, nz * 4.0]).abs() * 18.0 ;
                
                let superficie_final = PLANET_RADIUS + (continentes + montanhas) as f32;
                let nivel_mar = PLANET_RADIUS + 2.0;

                if dist <= superficie_final {
                    let altitude_local = dist - PLANET_RADIUS;
                    
                    let tipo = if altitude_local > 22.0 { TipoBloco::Neve } // Picos nevados
                               else if dist > superficie_final - 2.0 { TipoBloco::Grama } 
                               else if dist > PLANET_RADIUS - 10.0 { TipoBloco::Pedra } 
                               else { TipoBloco::Nucleo };
                               
                    mundo.mapa.insert(IVec3::new(x, y, z), tipo);
                } else if dist <= nivel_mar {
                    mundo.mapa.insert(IVec3::new(x, y, z), TipoBloco::Agua);
                }
            }
        }
    }

    let direcoes = [IVec3::X, IVec3::NEG_X, IVec3::Y, IVec3::NEG_Y, IVec3::Z, IVec3::NEG_Z];
    for (&pos, &tipo) in mundo.mapa.iter() {
        let mut toca_o_ar = false;
        for dir in direcoes.iter() {
            if !mundo.mapa.contains_key(&(pos + *dir)) {
                toca_o_ar = true; break;
            }
        }
        
        if toca_o_ar {
            let material = match tipo {
                TipoBloco::Grama => mat_grama.clone(),
                TipoBloco::Pedra => mat_pedra.clone(),
                TipoBloco::Neve => mat_neve.clone(),
                TipoBloco::Nucleo => mat_nucleo.clone(),
                TipoBloco::Agua => mat_agua.clone(),
            };
            commands.spawn(PbrBundle {
                mesh: mesh_cubo.clone(), material,
                transform: Transform::from_xyz(pos.x as f32, pos.y as f32, pos.z as f32),
                ..default()
            });
        }
    }
}