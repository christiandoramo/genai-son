use bevy::prelude::*;
use bevy::utils::HashMap;
use noise::{NoiseFn, OpenSimplex};

pub const PLANET_RADIUS: f32 = 45.0; 

#[derive(Clone, Copy, PartialEq)]
pub enum TipoBloco { Grama, Pedra, Areia, Agua, Nucleo, Neve }

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
    let mat_areia = materials.add(StandardMaterial { base_color: Color::srgb(0.9, 0.8, 0.5), ..default() });
    let mat_nucleo = materials.add(StandardMaterial { base_color: Color::srgb(0.2, 0.2, 0.2), ..default() });
    let mat_neve = materials.add(StandardMaterial { base_color: Color::srgb(0.95, 0.95, 1.0), ..default() });
    
    let mesh_cubo = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    let simplex = OpenSimplex::new(42);
    let limite = (PLANET_RADIUS + 30.0) as i32;

    for x in -limite..=limite {
        for y in -limite..=limite {
            for z in -limite..=limite {
                let pos = Vec3::new(x as f32, y as f32, z as f32);
                
                // Formato Cúbico Arredondado
                let dist_base = (pos.x.powi(4) + pos.y.powi(4) + pos.z.powi(4)).powf(0.25);
                if dist_base > limite as f32 { continue; }

                // O Vetor Normal para o cálculo de ruído perfeito nas 6 faces
                let dir = pos.normalize_or_zero();
                let nx = dir.x as f64; 
                let ny = dir.y as f64; 
                let nz = dir.z as f64;
                
                // --- ALGORITMO DE RELEVO ---
                // 1. Continentes vs Oceanos (Grandes Massas)
                let noise_cont = simplex.get([nx * 1.5, ny * 1.5, nz * 1.5]);
                let base_altura = noise_cont * 15.0; 
                
                let mut modificador_relevo = base_altura;

                // 2. Se for continente (acima da água), joga colinas e montanhas
                if base_altura > 1.0 {
                    let noise_colina = simplex.get([nx * 4.0, ny * 4.0, nz * 4.0]);
                    modificador_relevo += noise_colina * 6.0;

                    // 3. Cordilheiras rochosas apenas nas partes altas
                    if base_altura > 6.0 {
                        // O abs() e a inversão criam picos agressivos estilo Alpes
                        let noise_pico = (simplex.get([nx * 8.0, ny * 8.0, nz * 8.0]).abs() * -1.0 + 0.5) * 20.0;
                        modificador_relevo += noise_pico;
                    }
                }

                // TERRACING: Força a altura a "encaixar" em degraus inteiros largos (Patamares e Planaltos)
                // Isso quebra a "bolha suave" e dá a estética de blocos empilhados
                modificador_relevo = (modificador_relevo / 2.0).round() * 2.0;
                
                let superficie = PLANET_RADIUS + modificador_relevo as f32;
                let nivel_mar = PLANET_RADIUS + 2.0;

                // Preenchimento dos Blocos
                if dist_base <= superficie {
                    let altitude = dist_base - PLANET_RADIUS;
                    
                    let tipo = if altitude > 18.0 { TipoBloco::Neve } // Picos altíssimos
                               else if altitude > 6.0 { TipoBloco::Pedra } // Montanhas rochosas
                               else if dist_base <= nivel_mar + 1.5 && modificador_relevo < 2.0 { TipoBloco::Areia } // Praias
                               else if dist_base > superficie - 3.0 { TipoBloco::Grama } // Camada de terra
                               else { TipoBloco::Nucleo }; // Profundeza
                               
                    mundo.mapa.insert(IVec3::new(x, y, z), tipo);
                } else if dist_base <= nivel_mar {
                    mundo.mapa.insert(IVec3::new(x, y, z), TipoBloco::Agua);
                }
            }
        }
    }

    // Otimização: Só desenha a casca
    let direcoes = [IVec3::X, IVec3::NEG_X, IVec3::Y, IVec3::NEG_Y, IVec3::Z, IVec3::NEG_Z];
    for (&pos, &tipo) in mundo.mapa.iter() {
        let mut toca_o_ar = false;
        for dir in direcoes.iter() {
            if !mundo.mapa.contains_key(&(pos + *dir)) { toca_o_ar = true; break; }
        }
        
        if toca_o_ar {
            let material = match tipo {
                TipoBloco::Grama => mat_grama.clone(), TipoBloco::Pedra => mat_pedra.clone(),
                TipoBloco::Areia => mat_areia.clone(), TipoBloco::Nucleo => mat_nucleo.clone(),
                TipoBloco::Agua => mat_agua.clone(), TipoBloco::Neve => mat_neve.clone(),
            };
            commands.spawn(PbrBundle {
                mesh: mesh_cubo.clone(), material,
                transform: Transform::from_xyz(pos.x as f32, pos.y as f32, pos.z as f32),
                ..default()
            });
        }
    }
}