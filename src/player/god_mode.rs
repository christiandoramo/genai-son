use bevy::prelude::*;
use super::Player;
use crate::camera::MainCamera;

pub fn movimento_god_mode(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query_player: Query<&mut Transform, With<Player>>,
    query_camera: Query<&GlobalTransform, With<MainCamera>>, // Pega a câmera real
    query_player_comp: Query<&Player>,
) {
    let Ok(mut transform) = query_player.get_single_mut() else { return; };
    let Ok(camera_global) = query_camera.get_single() else { return; };
    let Ok(player) = query_player_comp.get_single() else { return; };

    // Direções absolutas de onde a câmera está olhando agora
    let forward = camera_global.forward().normalize_or_zero();
    let right = camera_global.right().normalize_or_zero();
    let up = camera_global.up().normalize_or_zero();

    let mut dir = Vec3::ZERO;

    if input.pressed(KeyCode::KeyW) { dir += forward; }
    if input.pressed(KeyCode::KeyS) { dir -= forward; }
    if input.pressed(KeyCode::KeyA) { dir -= right; }
    if input.pressed(KeyCode::KeyD) { dir += right; }
    
    // Q desce, E sobe
    if input.pressed(KeyCode::KeyQ) { dir -= up; }
    if input.pressed(KeyCode::KeyE) { dir += up; }

    transform.translation += dir.normalize_or_zero() * player.god_speed * time.delta_seconds();
}