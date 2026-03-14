use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel, MouseScrollUnit};
use crate::world::VoxelWorld;
use crate::physics::esta_dentro_do_chao;
use crate::camera::construir_rig_camera;

#[derive(Component)]
pub struct Player {
    pub velocidade_y: f32,
    pub no_chao: bool,
    pub pitch: f32,
    pub yaw: f32,
    pub god_mode: bool,
    pub god_speed: f32,
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(0.0, 70.0, 0.0)),
        Player { velocidade_y: 0.0, no_chao: false, pitch: 0.0, yaw: 0.0, god_mode: false, god_speed: 60.0 },
    ))
    .with_children(|parent| {
        construir_rig_camera(parent);
    });
}

pub fn movimento_e_fisica(
    input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut scroll_events: EventReader<MouseWheel>,
    time: Res<Time>,
    mundo: Res<VoxelWorld>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let (mut transform, mut player) = query.single_mut();
    let dt = time.delta_seconds();
    
    if input.just_pressed(KeyCode::KeyF) {
        player.god_mode = !player.god_mode;
        player.velocidade_y = 0.0;
    }

    for ev in scroll_events.read() {
        if player.god_mode {
            let scroll = match ev.unit { MouseScrollUnit::Line => ev.y * 5.0, MouseScrollUnit::Pixel => ev.y * 0.1 };
            player.god_speed = (player.god_speed + scroll).clamp(5.0, 300.0);
        }
    }

    let mut mouse_dx = 0.0;
    let mut mouse_dy = 0.0;
    for ev in mouse_motion_events.read() {
        mouse_dx -= ev.delta.x * 0.003;
        mouse_dy -= ev.delta.y * 0.003;
    }
    player.pitch = (player.pitch + mouse_dy).clamp(-1.5, 1.5);

    if player.god_mode {
        player.yaw += mouse_dx;
        let rot = Quat::from_rotation_y(player.yaw) * Quat::from_rotation_x(player.pitch);
        transform.rotation = rot;

        let mut dir = Vec3::ZERO;
        if input.pressed(KeyCode::KeyW) { dir += rot * Vec3::NEG_Z; }
        if input.pressed(KeyCode::KeyS) { dir -= rot * Vec3::NEG_Z; }
        if input.pressed(KeyCode::KeyA) { dir -= rot * Vec3::X; }
        if input.pressed(KeyCode::KeyD) { dir += rot * Vec3::X; }
        if input.pressed(KeyCode::KeyQ) { dir -= Vec3::Y; } 
        if input.pressed(KeyCode::KeyE) { dir += Vec3::Y; }

        transform.translation += dir.normalize_or_zero() * player.god_speed * dt;
        return; 
    }

    let p = transform.translation;
    let mut up = Vec3::new(p.x.powi(3), p.y.powi(3), p.z.powi(3)).normalize_or_zero();
    if up == Vec3::ZERO { up = Vec3::Y; }

    let current_up = transform.up();
    let align_rot = Quat::from_rotation_arc(*current_up, up);
    transform.rotation = align_rot * transform.rotation;
    transform.rotate_local_y(mouse_dx);

    let forward = transform.forward().normalize_or_zero();
    let right = transform.right().normalize_or_zero();
    let velocidade_andar = 8.0;

    let mut dir = Vec3::ZERO;
    if input.pressed(KeyCode::KeyW) { dir += forward; }
    if input.pressed(KeyCode::KeyS) { dir -= forward; }
    if input.pressed(KeyCode::KeyA) { dir -= right; }
    if input.pressed(KeyCode::KeyD) { dir += right; }
    
    let mut move_delta = dir.normalize_or_zero() * velocidade_andar * dt;
    move_delta = move_delta - (move_delta.dot(up) * up); 

    if move_delta.length() > 0.0 {
        let pos_teste_livre = transform.translation + move_delta + (up * 0.15);
        
        if !esta_dentro_do_chao(&mundo.mapa, pos_teste_livre, up) {
            transform.translation += move_delta; 
        } else {
            // REMOVIDO O AVISO DO COMPILADOR: Apenas quebramos o loop se conseguir subir
            for i in 2..=12 {
                let altura_degrau = i as f32 * 0.1;
                let pos_degrau = transform.translation + move_delta + (up * altura_degrau);
                
                if !esta_dentro_do_chao(&mundo.mapa, pos_degrau, up) {
                    transform.translation = pos_degrau;
                    break;
                }
            }
        }
    }

    player.velocidade_y -= 25.0 * dt;
    let old_pos_y = transform.translation;
    transform.translation += up * player.velocidade_y * dt;

    if esta_dentro_do_chao(&mundo.mapa, transform.translation, up) {
        transform.translation = old_pos_y; 
        if player.velocidade_y < 0.0 { player.no_chao = true; }
        player.velocidade_y = 0.0;
        
        let mut safe = 5;
        while esta_dentro_do_chao(&mundo.mapa, transform.translation, up) && safe > 0 {
            transform.translation += up * 0.01;
            safe -= 1;
        }
    } else {
        player.no_chao = false;
    }

    if input.pressed(KeyCode::Space) && player.no_chao {
        player.velocidade_y = 12.0;
        player.no_chao = false;
    }
}