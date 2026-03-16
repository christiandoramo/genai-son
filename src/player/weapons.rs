// src/player/weapons.rs
use crate::camera::MainCamera;
use crate::physics::sand::ParticulaAreia;
use crate::world::VoxelWorld;
use bevy::prelude::*;

#[derive(Component)]
pub struct Projetil {
    pub velocidade: Vec3,
    pub tempo_vida: Timer,
}

pub fn atirar_pistola(
    mut commands: Commands,
    input: Res<ButtonInput<MouseButton>>,
    query_camera: Query<&GlobalTransform, With<MainCamera>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if input.just_pressed(MouseButton::Left) {
        if let Ok(camera_transform) = query_camera.get_single() {
            let origin = camera_transform.translation();
            let direction = camera_transform.forward().normalize_or_zero();

            commands.spawn((
                PbrBundle {
                    // O TIRO AGORA É UM VOXEL CÚBICO PESADO
                    mesh: meshes.add(Cuboid::new(0.3, 0.3, 0.3)),
                    material: materials.add(StandardMaterial {
                        base_color: Color::srgb(0.1, 0.1, 0.1),
                        ..default()
                    }),
                    transform: Transform::from_translation(origin + direction * 1.5),
                    ..default()
                },
                Projetil {
                    velocidade: direction * 60.0,
                    tempo_vida: Timer::from_seconds(3.0, TimerMode::Once),
                },
            ));
        }
    }
}

pub fn atualizar_projeteis(
    mut commands: Commands,
    time: Res<Time>,
    mundo: Res<VoxelWorld>,
    mut query: Query<(Entity, &mut Transform, &mut Projetil)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dt = time.delta_seconds();
    let mat_areia = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.4, 0.2), // Terra úmida
        ..default()
    });

    for (entity, mut transform, mut projetil) in query.iter_mut() {
        projetil.tempo_vida.tick(time.delta());

        if projetil.tempo_vida.just_finished() {
            commands.entity(entity).despawn();
            continue;
        }

        let nova_pos = transform.translation + projetil.velocidade * dt;
        let coord = IVec3::new(
            nova_pos.x.round() as i32,
            nova_pos.y.round() as i32,
            nova_pos.z.round() as i32,
        );

        if mundo.mapa.contains_key(&coord) {
            // EXPLOSÃO DE 64 VOXELS POR TIRO!
            // Criamos uma grade 4x4x4 instantânea que vai se desfazer e escorrer.
            for x in -1..=2 {
                for y in -1..=2 {
                    for z in -1..=2 {
                        let espalhamento = Vec3::new(x as f32, y as f32, z as f32) * 0.2;

                        commands.spawn((
                            PbrBundle {
                                mesh: meshes.add(Cuboid::new(0.2, 0.2, 0.2)),
                                material: mat_areia.clone(),
                                transform: Transform::from_translation(
                                    transform.translation + espalhamento,
                                ),
                                ..default()
                            },
                            ParticulaAreia {
                                velocidade: (projetil.velocidade * 0.05) + espalhamento * 8.0,
                                tempo_vida: Timer::from_seconds(20.0, TimerMode::Once),
                                dormindo: false, // <--- A peça que estava faltando!
                            },
                        ));
                    }
                }
            }
            commands.entity(entity).despawn();
        } else {
            transform.translation = nova_pos;
        }
    }
}
