use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use super::Player;

pub fn rotacionar_camera(
    mut mouse_motion_events: EventReader<MouseMotion>, 
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let Ok((mut transform, mut player)) = query.get_single_mut() else { return };
    let mut mouse_dx = 0.0; 
    let mut mouse_dy = 0.0;
    
    for ev in mouse_motion_events.read() { 
        mouse_dx -= ev.delta.x * 0.003; 
        mouse_dy -= ev.delta.y * 0.003; 
    }
    
    player.pitch = (player.pitch + mouse_dy).clamp(-1.5, 1.5);
    
    if player.god_mode {
        player.yaw += mouse_dx;
        transform.rotation = Quat::from_rotation_y(player.yaw) * Quat::from_rotation_x(player.pitch);
    } else { 
        transform.rotate_local_y(mouse_dx); 
    }
}