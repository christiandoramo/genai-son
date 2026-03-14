use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraPivot; // O "Pescoço" do jogador

pub fn construir_rig_camera(parent: &mut ChildBuilder) {
    // O pivô fica na altura dos olhos (0.7m acima do centro do jogador que mede 1.8m)
    parent.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.7, 0.0)), 
        CameraPivot,
    )).with_children(|pivot| {
        pivot.spawn((
            Camera3dBundle::default(), // Em primeira pessoa, a câmera fica exatamete no pescoço
            MainCamera,
        ));
    });
}

// O movimento do mouse (para cima/baixo) gira apenas a câmera
pub fn sync_camera(
    player_query: Query<&crate::player::Player>,
    mut pivot_query: Query<&mut Transform, With<CameraPivot>>,
) {
    if let Ok(player) = player_query.get_single() {
        if let Ok(mut pivot_transform) = pivot_query.get_single_mut() {
            // Rotaciona a cabeça verticalmente
            pivot_transform.rotation = Quat::from_rotation_x(player.pitch);
        }
    }
}