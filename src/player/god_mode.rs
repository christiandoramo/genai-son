use bevy::prelude::*;
use super::Player;
use crate::camera::MainCamera;

pub fn movimento_god_mode(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    // Simplifiquei a query para pegar os dois componentes de uma vez
    mut query_player: Query<(&mut Transform, &Player)>, 
    query_camera: Query<&GlobalTransform, With<MainCamera>>,
) {
    let Ok((mut transform, player)) = query_player.get_single_mut() else { return; };
    let Ok(camera_global) = query_camera.get_single() else { return; };

    // ----------------------------------------------------------------
    // 1. ORIENTAÇÃO ESFÉRICA (Para não inverter os eixos do mouse!)
    // ----------------------------------------------------------------
    let pos_atual = transform.translation;
    let planet_up = pos_atual.normalize_or_zero();
    
    if planet_up != Vec3::ZERO {
        let fwd: Vec3 = transform.forward().into();
        let mut proj_fwd = (fwd - fwd.dot(planet_up) * planet_up).normalize_or_zero();
        if proj_fwd == Vec3::ZERO { proj_fwd = transform.up().into(); }

        if let (Ok(dir_fwd), Ok(dir_up)) = (Dir3::new(proj_fwd), Dir3::new(planet_up)) {
            let target_rotation = Transform::default().looking_to(dir_fwd, dir_up).rotation;
            // O God Mode agora sabe para onde fica o "chão" e gira o seu corpo suavemente
            transform.rotation = transform.rotation.slerp(target_rotation, time.delta_seconds() * 10.0);
        }
    }

    // ----------------------------------------------------------------
    // 2. MOVIMENTO LIVRE DE DRONE (6 Graus de Liberdade)
    // ----------------------------------------------------------------
    let forward = camera_global.forward().normalize_or_zero();
    let right = camera_global.right().normalize_or_zero();
    let up = camera_global.up().normalize_or_zero();

    let mut dir = Vec3::ZERO;

    if input.pressed(KeyCode::KeyW) { dir += forward; }
    if input.pressed(KeyCode::KeyS) { dir -= forward; }
    if input.pressed(KeyCode::KeyA) { dir -= right; }
    if input.pressed(KeyCode::KeyD) { dir += right; }
    
    // Q e E respeitam a visão da câmera
    if input.pressed(KeyCode::KeyQ) { dir -= up; }
    if input.pressed(KeyCode::KeyE) { dir += up; }

    transform.translation += dir.normalize_or_zero() * player.god_speed * time.delta_seconds();
}