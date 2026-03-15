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
    parent
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.7, 0.0)),
            CameraPivot,
        ))
        .with_children(|pivot| {
            pivot.spawn((
                Camera3dBundle::default(),
                FogSettings {
                    color: Color::srgb(0.4, 0.7, 0.9),
                    falloff: FogFalloff::Linear {
                        start: 30.0,
                        end: 65.0, // Névoa densa protegendo os chunks de carregar na sua cara
                    },
                    ..default()
                },
                MainCamera,
            ));
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
