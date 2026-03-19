pub mod camera;
pub mod controller;
pub mod math_util;
pub mod physics;

use crate::engine::voxel::storage::Projectile;
use crate::game::player::camera::Camera;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameMode {
    God,
    Normal,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WeaponType {
    None,
    Creator,
    Plasma,
    Bazooka,
}

pub struct Player {
    pub camera: Camera,
    pub mode: GameMode,
    pub active_weapon: WeaponType,
    pub selected_material: u32,
    pub flashlight: bool,
    pub is_day: bool,
    pub is_shooting: bool,
    pub cooldown: f32,
    pub velocity_y: f32,
    pub on_ground: bool,
    pub physics_up: [f32; 3],
    pub visual_up: [f32; 3],
    // Cache de edições locais para colisão (Sincronizado com o World)
    pub world_edits: HashMap<[i32; 3], u32>,
    pub active_projectiles: Vec<Projectile>,
}

impl Player {
    pub fn new(pos: [f32; 3]) -> Self {
        Self {
            camera: Camera::new(pos),
            mode: GameMode::Normal,
            active_weapon: WeaponType::Creator,
            selected_material: 1,
            flashlight: false,
            is_day: true,
            is_shooting: false,
            cooldown: 0.0,
            velocity_y: 0.0,
            on_ground: false,
            physics_up: [0.0, 1.0, 0.0],
            visual_up: [0.0, 1.0, 0.0],
            world_edits: HashMap::new(),
            active_projectiles: Vec::new(),
        }
    }
}
