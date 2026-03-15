// src/player/mod.rs
use bevy::prelude::*;
use crate::world::PLANET_RADIUS;

pub mod movement;
pub mod god_mode; // <--- Registrando o novo módulo!

pub const GRAVITY_INFLUENCE_RADIUS: f32 = PLANET_RADIUS * 5.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, movement::spawn_player)
           .add_systems(Update, (
               movement::tratar_inputs_estado,
               movement::rotacionar_camera,
               // Chama a função do arquivo novo quando estiver no modo Deus:
               god_mode::movimento_god_mode.run_if(movement::is_god_mode),
               movement::movimento_sobrevivencia.run_if(movement::is_survival_mode),
           ));
    }
}

#[derive(Component)]
pub struct Player {
    pub velocidade_y: f32,
    pub no_chao: bool,
    pub pitch: f32,
    #[allow(dead_code)]
    pub yaw: f32,
    pub god_mode: bool,
    pub god_speed: f32,
}