// src/player/movement.rs
use super::Player;
use crate::camera::construir_rig_camera;
use crate::world::{ChunkManager, VoxelWorld};
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

pub fn is_god_mode(query: Query<&Player>) -> bool {
    query.get_single().map(|p| p.god_mode).unwrap_or(false)
}
pub fn is_survival_mode(query: Query<&Player>) -> bool {
    query.get_single().map(|p| !p.god_mode).unwrap_or(false)
}

pub fn spawn_player(mut commands: Commands) {
    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(
                0.0,
                super::PLANET_RADIUS + 80.0,
                0.0,
            )),
            Player {
                velocidade_y: 0.0,
                no_chao: false,
                pitch: 0.0,
                yaw: 0.0,
                god_mode: false,
                god_speed: 60.0,
            },
        ))
        .with_children(|parent| {
            construir_rig_camera(parent);
        });
}

pub fn tratar_inputs_estado(
    input: Res<ButtonInput<KeyCode>>,
    mut scroll_events: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let Ok((transform, mut player)) = query.get_single_mut() else {
        return;
    };

    if input.just_pressed(KeyCode::KeyF) {
        player.god_mode = !player.god_mode;
        player.velocidade_y = 0.0;

        if player.god_mode {
            // CORREÇÃO BEVY 0.14: look_to agora exige o tipo estrito Dir3 para evitar bugs matemáticos
            // if let Ok(dir_forward) = Dir3::new(transform.forward().into()) {
            //     transform.look_to(dir_forward, Dir3::Y);
            // }

            let (yaw, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            player.yaw = yaw;
            player.pitch = pitch;
        }
    }

    for ev in scroll_events.read() {
        if player.god_mode {
            let scroll = match ev.unit {
                MouseScrollUnit::Line => ev.y * 5.0,
                MouseScrollUnit::Pixel => ev.y * 0.1,
            };
            player.god_speed = (player.god_speed + scroll).clamp(5.0, 300.0);
        }
    }
}

pub fn rotacionar_camera(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let Ok((mut transform, mut player)) = query.get_single_mut() else {
        return;
    };
    let mut mouse_dx = 0.0;
    let mut mouse_dy = 0.0;

    for ev in mouse_motion_events.read() {
        mouse_dx -= ev.delta.x * 0.003;
        mouse_dy -= ev.delta.y * 0.003;
    }

    player.pitch = (player.pitch + mouse_dy).clamp(-1.5, 1.5);

    // if player.god_mode {
    //     // CORREÇÃO BEVY 0.14: Usa Dir3::Y no lugar de Vec3::Y
    //     transform.rotate_axis(Dir3::Y, mouse_dx);
    // } else {
    //     transform.rotate_local_y(mouse_dx);
    // }

    if player.god_mode {
        // Usa o vetor 'up' LOCAL do próprio jogador, e não o do universo.
        let up_local = *transform.up();
        if let Ok(dir_up) = Dir3::new(up_local) {
            transform.rotate_axis(dir_up, mouse_dx);
        }
    } else {
        transform.rotate_local_y(mouse_dx);
    }
}

pub fn movimento_sobrevivencia(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mundo: Res<VoxelWorld>,
    chunk_manager: Res<ChunkManager>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let Ok((mut transform, mut player)) = query.get_single_mut() else { return; };
    let dt = time.delta_seconds().min(0.05); 
    let pos_atual = transform.translation;

    let under_gravity = pos_atual.length() < super::GRAVITY_INFLUENCE_RADIUS;
    
    // ----------------------------------------------------------------
    // 1. GRAVIDADE FÍSICA (O Cubo Duro - Mantém você colado no chão)
    // ----------------------------------------------------------------
    let bias = 1.0; 
    let mut physics_up = if under_gravity {
        let up_atual_fis = if transform.up().x.abs() > 0.5 { Vec3::X * transform.up().x.signum() }
                           else if transform.up().y.abs() > 0.5 { Vec3::Y * transform.up().y.signum() }
                           else { Vec3::Z * transform.up().z.signum() };

        let abs_x = pos_atual.x.abs() + if up_atual_fis.x.abs() > 0.5 { bias } else { 0.0 };
        let abs_y = pos_atual.y.abs() + if up_atual_fis.y.abs() > 0.5 { bias } else { 0.0 };
        let abs_z = pos_atual.z.abs() + if up_atual_fis.z.abs() > 0.5 { bias } else { 0.0 };

        if abs_x > abs_y && abs_x > abs_z { Vec3::new(pos_atual.x.signum(), 0.0, 0.0) } 
        else if abs_y > abs_x && abs_y > abs_z { Vec3::new(0.0, pos_atual.y.signum(), 0.0) } 
        else { Vec3::new(0.0, 0.0, pos_atual.z.signum()) }
    } else {
        *transform.up()
    };
    if physics_up == Vec3::ZERO { physics_up = Vec3::Y; }

    // ----------------------------------------------------------------
    // 2. GRAVIDADE VISUAL (A Esfera Suave - Antecipa a quina para a Câmera)
    // ----------------------------------------------------------------
    let mut visual_up = if under_gravity {
        // Usa a direção pura do centro do planeta para inclinar a câmera aos poucos!
        let p_norm = pos_atual.normalize_or_zero();
        let p = 2.5_f32; // Esse valor faz a câmera começar a dobrar um pouco antes da borda
        Vec3::new(
            p_norm.x.abs().powf(p) * p_norm.x.signum(),
            p_norm.y.abs().powf(p) * p_norm.y.signum(),
            p_norm.z.abs().powf(p) * p_norm.z.signum(),
        ).normalize_or_zero()
    } else {
        *transform.up()
    };
    if visual_up == Vec3::ZERO { visual_up = Vec3::Y; }

    // Rotacionamos o JOGADOR com base na Gravidade VISUAL
    if under_gravity && transform.up().dot(visual_up) > -0.999 {
        let fwd: Vec3 = transform.forward().into();
        let mut proj_fwd = (fwd - fwd.dot(visual_up) * visual_up).normalize_or_zero();
        if proj_fwd == Vec3::ZERO { proj_fwd = transform.up().into(); } 

        if let (Ok(dir_fwd), Ok(dir_up)) = (Dir3::new(proj_fwd), Dir3::new(visual_up)) {
            let target_rotation = Transform::default().looking_to(dir_fwd, dir_up).rotation;
            transform.rotation = transform.rotation.slerp(target_rotation, time.delta_seconds() * 8.0);
        }
    }

    // ----------------------------------------------------------------
    // 3. MOVIMENTO E COLISÃO (Usa estritamente a Física Cúbica)
    // ----------------------------------------------------------------
    let pos_futura = pos_atual + (-physics_up * 2.0); 
    let chunk_futuro = IVec3::new(
        (pos_futura.x / crate::world::CHUNK_SIZE as f32).floor() as i32,
        (pos_futura.y / crate::world::CHUNK_SIZE as f32).floor() as i32,
        (pos_futura.z / crate::world::CHUNK_SIZE as f32).floor() as i32,
    );
    let is_chunk_loaded = chunk_manager.chunks_gerados.contains(&chunk_futuro);

    let forward = transform.forward().normalize_or_zero();
    let right = transform.right().normalize_or_zero();
    let velocidade_andar = if under_gravity { 8.0 } else { 2.0 };
    let mut dir = Vec3::ZERO;

    if is_chunk_loaded {
        if input.pressed(KeyCode::KeyW) { dir += forward; }
        if input.pressed(KeyCode::KeyS) { dir -= forward; }
        if input.pressed(KeyCode::KeyA) { dir -= right; }
        if input.pressed(KeyCode::KeyD) { dir += right; }
    } else {
        player.velocidade_y = 0.0;
    }

    let mut move_delta = dir.normalize_or_zero() * velocidade_andar * dt;
    // Anula a velocidade vertical FÍSICA para ele deslizar perfeitamente no plano
    move_delta -= move_delta.dot(physics_up) * physics_up;
    transform.translation += move_delta;

    if is_chunk_loaded {
        if under_gravity {
            if player.no_chao && player.velocidade_y <= 0.0 {
                player.velocidade_y = -0.5; 
            } else {
                player.velocidade_y -= 25.0 * dt;
            }
        } else {
            player.velocidade_y = player.velocidade_y.lerp(0.0, dt * 2.0);
        }

        player.velocidade_y = player.velocidade_y.clamp(-20.0, 20.0);
        
        let mut nova_pos = transform.translation;
        nova_pos += physics_up * player.velocidade_y * dt;
        
        // Minkowski varre usando o CIMA físico do cubo
        let tocou_no_chao = crate::physics::resolver_colisao_minkowski(&mundo.mapa, &mut nova_pos, physics_up);

        if tocou_no_chao {
            if player.velocidade_y < 0.0 {
                player.no_chao = true;
                player.velocidade_y = 0.0;
            }
        } else {
            player.no_chao = false;
        }
        
        transform.translation = nova_pos;

        // Pular também empurra no eixo Cúbico perfeito
        if input.pressed(KeyCode::Space) && under_gravity && player.no_chao {
            player.velocidade_y = 10.0;
            player.no_chao = false;
        }
    }
}