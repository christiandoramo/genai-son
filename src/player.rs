use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use crate::world::{VoxelWorld, TipoBloco};
use crate::camera::construir_rig_camera;

#[derive(Component)]
pub struct Player {
    pub velocidade_y: f32,
    pub no_chao: bool,
    pub yaw: f32,   
    pub pitch: f32, 
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(0.0, 30.0, 0.0)),
        Player { velocidade_y: 0.0, no_chao: false, yaw: 0.0, pitch: 0.0 },
    ))
    .with_children(|parent| {
        construir_rig_camera(parent);
    });
}

// A CAIXA DE COLISÃO PERFEITA
fn colide_com_cenario(mapa: &bevy::utils::HashMap<IVec3, TipoBloco>, pos: Vec3) -> bool {
    let raio = 0.35; // Espessura do jogador
    let altura_meio = 0.89; // Metade da altura (para não agarrar o chão)

    let p_min_x = pos.x - raio;
    let p_max_x = pos.x + raio;
    let p_min_y = pos.y - altura_meio;
    let p_max_y = pos.y + altura_meio;
    let p_min_z = pos.z - raio;
    let p_max_z = pos.z + raio;

    let min_x = (p_min_x - 0.5).floor() as i32;
    let max_x = (p_max_x + 0.5).ceil() as i32;
    let min_y = (p_min_y - 0.5).floor() as i32;
    let max_y = (p_max_y + 0.5).ceil() as i32;
    let min_z = (p_min_z - 0.5).floor() as i32;
    let max_z = (p_max_z + 0.5).ceil() as i32;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                if mapa.contains_key(&IVec3::new(x, y, z)) {
                    let b_min_x = x as f32 - 0.5;
                    let b_max_x = x as f32 + 0.5;
                    let b_min_y = y as f32 - 0.5;
                    let b_max_y = y as f32 + 0.5;
                    let b_min_z = z as f32 - 0.5;
                    let b_max_z = z as f32 + 0.5;

                    if p_min_x < b_max_x && p_max_x > b_min_x &&
                       p_min_y < b_max_y && p_max_y > b_min_y &&
                       p_min_z < b_max_z && p_max_z > b_min_z 
                    {
                        return true;
                    }
                }
            }
        }
    }
    false
}

pub fn movimento_e_fisica(
    input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    time: Res<Time>,
    mundo: Res<VoxelWorld>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let (mut transform, mut player) = query.single_mut();
    let dt = time.delta_seconds();
    let gravidade = -25.0;
    let velocidade_pulo = 9.0;
    let velocidade_andar = 6.0;

    // --- 1. MOUSE LOOK ---
    for ev in mouse_motion_events.read() {
        player.yaw -= ev.delta.x * 0.003;
        player.pitch -= ev.delta.y * 0.003;
    }
    player.pitch = player.pitch.clamp(-1.5, 1.5);
    transform.rotation = Quat::from_rotation_y(player.yaw);

    // --- 2. VETORES DE DIREÇÃO ---
    let forward = transform.forward().normalize_or_zero();
    let right = transform.right().normalize_or_zero();
    
    let direcao_frente = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    let direcao_lado = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

    let mut delta_mov = Vec3::ZERO;
    if input.pressed(KeyCode::KeyW) { delta_mov += direcao_frente; }
    if input.pressed(KeyCode::KeyS) { delta_mov -= direcao_frente; }
    if input.pressed(KeyCode::KeyA) { delta_mov -= direcao_lado; }
    if input.pressed(KeyCode::KeyD) { delta_mov += direcao_lado; }
    
    delta_mov = delta_mov.normalize_or_zero() * velocidade_andar * dt;

    // --- 3. MOVIMENTO X (Testado Isoladamente) ---
    transform.translation.x += delta_mov.x;
    if colide_com_cenario(&mundo.mapa, transform.translation) {
        transform.translation.x -= delta_mov.x; // Bateu? Desfaz o passo.
    }

    // --- 4. MOVIMENTO Z (Testado Isoladamente) ---
    transform.translation.z += delta_mov.z;
    if colide_com_cenario(&mundo.mapa, transform.translation) {
        transform.translation.z -= delta_mov.z; // Bateu? Desfaz o passo.
    }

    // --- 5. GRAVIDADE E PULO (Eixo Y Isolado) ---
    player.velocidade_y += gravidade * dt;
    let dy = player.velocidade_y * dt;
    transform.translation.y += dy;

    if colide_com_cenario(&mundo.mapa, transform.translation) {
        if player.velocidade_y < 0.0 {
            // ESTAVA CAINDO E BATEU NO CHÃO
            // Calcula o bloco exato que o pé afundou
            let bota_y = transform.translation.y - 0.89;
            let bloco_y = bota_y.round();
            
            // Encaixa milimetricamente acima do bloco
            transform.translation.y = (bloco_y as f32) + 0.5 + 0.891; 
            player.no_chao = true;
        } else if player.velocidade_y > 0.0 {
            // ESTAVA SUBINDO E BATEU A CABEÇA NO TETO
            let cabeca_y = transform.translation.y + 0.89;
            let bloco_y = cabeca_y.round();
            
            // Encaixa abaixo do teto
            transform.translation.y = (bloco_y as f32) - 0.5 - 0.891;
        }
        player.velocidade_y = 0.0; // Anula a inércia da queda/pulo
    } else {
        // Nada embaixo, caindo livremente
        player.no_chao = false;
    }

    // --- 6. COMANDO DE PULO ---
    if input.pressed(KeyCode::Space) && player.no_chao {
        player.velocidade_y = velocidade_pulo;
        player.no_chao = false;
    }

    // Salva-vidas do Limbo
    if transform.translation.y < -30.0 {
        transform.translation = Vec3::new(0.0, 30.0, 0.0);
        player.velocidade_y = 0.0;
    }
}