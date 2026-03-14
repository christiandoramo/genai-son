use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraPivot;

pub fn construir_rig_camera(parent: &mut ChildBuilder) {
    parent.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.7, 0.0)), 
        CameraPivot,
    )).with_children(|pivot| {
        pivot.spawn((
            Camera3dBundle::default(), 
            FogSettings {
                color: Color::srgb(0.4, 0.7, 0.9), 
                falloff: FogFalloff::Linear {
                    start: 20.0, 
                    end: 45.0, // Esconde os chunks nascendo
                },
                ..default()
            },
            MainCamera,
        ));
    });
}

pub fn sync_camera(
    player_query: Query<&crate::player::Player>,
    mut pivot_query: Query<&mut Transform, With<CameraPivot>>,
) {
    if let Ok(player) = player_query.get_single() {
        if let Ok(mut pivot_transform) = pivot_query.get_single_mut() {
            pivot_transform.rotation = Quat::from_rotation_x(player.pitch);
        }
    }
}