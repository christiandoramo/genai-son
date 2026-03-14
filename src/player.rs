use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel, MouseScrollUnit};
use crate::world::{VoxelWorld, TipoBloco};
use crate::camera::construir_rig_camera;

#[derive(Component)]
pub struct Player {
    pub velocidade_y: f32,
    pub no_chao: bool,
    pub yaw: f32,   
    pub pitch: f32,
    pub god_mode: bool,
    pub god_speed: f32,
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(0.1, 120.0, 0.1)), // Nasce mais alto!
        Player { velocidade_y: 0.0, no_chao: false, yaw: 0.0, pitch: 0.0, god_mode: false, god_speed: 60.0 },
    ))
    .with_children(|parent| {
        construir_rig_camera(parent);
    });
}

fn blocos_no_raio(mapa: &bevy::utils::HashMap<IVec3, TipoBloco>, pos: Vec3, raio: f32) -> bool {
    let r = raio.ceil() as i32;
    let cx = pos.x.round() as i32;
    let cy = pos.y.round() as i32;
    let cz = pos.z.round() as i32;

    for x in -r..=r {
        for y in -r..=r {
            for z in -r..=r {
                let b_pos = IVec3::new(cx + x, cy + y, cz + z);
                if mapa.contains_key(&b_pos) {
                    let b_center = Vec3::new(b_pos.x as f32, b_pos.y as f32, b_pos.z as f32);
                    if pos.distance(b_center) < raio + 0.45 { // Raio perfeito para escorregar sem prender
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn esta_dentro_do_chao(mapa: &bevy::utils::HashMap<IVec3, TipoBloco>, pos: Vec3, up_vector: Vec3) -> bool {
    let pe = pos - (up_vector * 0.85);
    let cintura = pos;
    let cabeca = pos + (up_vector * 0.6);
    let raio_corpo = 0.25;

    blocos_no_raio(mapa, pe, raio_corpo) || 
    blocos_no_raio(mapa, cintura, raio_corpo) || 
    blocos_no_raio(mapa, cabeca, raio_corpo)
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
            player.god_speed = (player.god_speed + scroll).clamp(5.0, 300.0); // Velocidade máxima aumentada
        }
    }

    for ev in mouse_motion_events.read() {
        player.yaw -= ev.delta.x * 0.003;
        player.pitch -= ev.delta.y * 0.003;
    }
    player.pitch = player.pitch.clamp(-1.5, 1.5);

    // ==========================================
    // MODO DEUS (Voo Absoluto Sem Atrito)
    // ==========================================
    if player.god_mode {
        let rotacao_livre = Quat::from_rotation_y(player.yaw) * Quat::from_rotation_x(player.pitch);
        transform.rotation = rotacao_livre;

        let forward = rotacao_livre * Vec3::NEG_Z;
        let right = rotacao_livre * Vec3::X;
        let up = rotacao_livre * Vec3::Y;

        let mut delta_mov = Vec3::ZERO;
        if input.pressed(KeyCode::KeyW) { delta_mov += forward; }
        if input.pressed(KeyCode::KeyS) { delta_mov -= forward; }
        if input.pressed(KeyCode::KeyA) { delta_mov -= right; }
        if input.pressed(KeyCode::KeyD) { delta_mov += right; }
        if input.pressed(KeyCode::KeyQ) { delta_mov -= up; } 
        if input.pressed(KeyCode::KeyE) { delta_mov += up; } 
        
        transform.translation += delta_mov.normalize_or_zero() * player.god_speed * dt;
        return; 
    }

    // ==========================================
    // MODO SOBREVIVÊNCIA (Planeta Gigante)
    // ==========================================
    let up_vector = transform.translation.normalize_or_zero(); 
    
    let mut align_rot = Quat::IDENTITY;
    if up_vector.distance(Vec3::Y) > 0.001 && up_vector.distance(Vec3::NEG_Y) > 0.001 {
        align_rot = Quat::from_rotation_arc(Vec3::Y, up_vector);
    } else if up_vector.distance(Vec3::NEG_Y) <= 0.001 {
        align_rot = Quat::from_rotation_x(std::f32::consts::PI);
    }

    transform.rotation = align_rot * Quat::from_rotation_y(player.yaw) * Quat::from_rotation_x(player.pitch);

    let gravidade = -25.0;
    let velocidade_pulo = 12.0;
    let velocidade_andar = 8.0;

    let move_rot = align_rot * Quat::from_rotation_y(player.yaw);
    let move_frente = move_rot * Vec3::NEG_Z;
    let move_lado = move_rot * Vec3::X;

    let mut delta_mov = Vec3::ZERO;
    if input.pressed(KeyCode::KeyW) { delta_mov += move_frente; }
    if input.pressed(KeyCode::KeyS) { delta_mov -= move_frente; }
    if input.pressed(KeyCode::KeyA) { delta_mov -= move_lado; }
    if input.pressed(KeyCode::KeyD) { delta_mov += move_lado; }
    
    delta_mov = delta_mov.normalize_or_zero() * velocidade_andar * dt;

    // --- AUTO-STEP AMANTEIGADO (A CURA DA TREMEDEIRA) ---
    if delta_mov.length() > 0.0 {
        let old_pos = transform.translation;
        transform.translation += delta_mov; // Tenta andar pra frente
        
        // Se bateu, tenta "deslizar" para cima em passos minúsculos (suaves)
        let mut altura_degrau = 0.0;
        while esta_dentro_do_chao(&mundo.mapa, transform.translation, up_vector) && altura_degrau < 1.2 {
            transform.translation += up_vector * 0.1;
            altura_degrau += 0.1;
        }
        
        // Se teve que subir mais de 1.2, é um muro alto demais! Bloqueia o passo.
        if altura_degrau >= 1.2 {
            transform.translation = old_pos;
        }
    }

    // --- GRAVIDADE E PULO PERFEITO ---
    player.velocidade_y += gravidade * dt;
    let old_pos_y = transform.translation;
    transform.translation += up_vector * player.velocidade_y * dt;

    if esta_dentro_do_chao(&mundo.mapa, transform.translation, up_vector) {
        transform.translation = old_pos_y; // Desfaz a queda além do bloco
        
        if player.velocidade_y < 0.0 {
            player.no_chao = true;
        }
        player.velocidade_y = 0.0;
        
        // Encaixa suavemente no chão exato para não "tremer" na descida de morros
        let mut max_tentativas = 10;
        while esta_dentro_do_chao(&mundo.mapa, transform.translation, up_vector) && max_tentativas > 0 {
            transform.translation += up_vector * 0.05;
            max_tentativas -= 1;
        }
    } else {
        player.no_chao = false;
    }

    if input.pressed(KeyCode::Space) && player.no_chao {
        player.velocidade_y = velocidade_pulo;
        player.no_chao = false;
    }
}