// src/camera/mod.rs
use crate::player::Player;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sync_camera);
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraPivot;

pub fn construir_rig_camera(parent: &mut ChildBuilder) {
    // Calculamos o limite visual exato onde os chunks param de carregar
    let raio_visao = crate::world::RENDER_DISTANCE as f32 * crate::world::CHUNK_SIZE as f32;

    parent
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.7, 0.0)),
            CameraPivot,
        ))
        .with_children(|pivot| {
            pivot.spawn((
                Camera3dBundle {
                    projection: Projection::Perspective(PerspectiveProjection {
                        far: raio_visao * 1.5, 
                        ..default()
                    }),
                    ..default()
                },
                FogSettings {
                    color: Color::srgb(0.4, 0.7, 0.9), // Combina com o céu
                    falloff: FogFalloff::Linear {
                        start: raio_visao * 0.6, // Começa a borrar um pouco antes do limite
                        end: raio_visao * 0.95,  // Fica 100% opaco no limite, escondendo o carregamento
                    },
                    ..default()
                },
                MainCamera,
            ));
        });
        
    parent.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: false,
            ..default()
        },
        ..default()
    });
}

fn sync_camera(
    player_query: Query<&Player>,
    mut pivot_query: Query<&mut Transform, With<CameraPivot>>,
) {
    if let Ok(player) = player_query.get_single() {
        if let Ok(mut pivot_transform) = pivot_query.get_single_mut() {
            pivot_transform.rotation = Quat::from_rotation_x(player.pitch);
        }
    }
}