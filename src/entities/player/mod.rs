pub mod physics;

use std::collections::HashMap;
use crate::entities::camera::Camera;
use winit::keyboard::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameMode { God, Normal }
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Weapon { Creator, Plasma, Bazooka }

pub struct Player {
    pub camera: Camera,
    pub mode: GameMode,
    pub active_weapon: Weapon,
    pub selected_material: u32,
    pub flashlight: bool,
    pub is_day: bool,
    pub is_shooting: bool,
    pub cooldown: f32,
    pub velocidade_y: f32,
    pub no_chao: bool,
    pub physics_up: [f32; 3],
    pub visual_up: [f32; 3],
    pub keys: [bool; 255],
    pub world_edits: HashMap<[i32; 3], u32>,
}

impl Player {
    pub fn new(start_pos: [f32; 3]) -> Self {
        Self {
            camera: Camera::new(start_pos),
            mode: GameMode::Normal,
            active_weapon: Weapon::Creator,
            selected_material: 1,
            flashlight: false,
            is_day: true,
            is_shooting: false,
            cooldown: 0.0,
            velocidade_y: 0.0,
            no_chao: false,
            physics_up: [0.0, 1.0, 0.0],
            visual_up: [0.0, 1.0, 0.0],
            keys: [false; 255],
            world_edits: HashMap::new(),
        }
    }

    pub fn handle_keyboard(&mut self, keycode: KeyCode, pressed: bool) {
        let code = keycode as usize;
        if code < 255 { self.keys[code] = pressed; }
        if pressed {
            match keycode {
                KeyCode::KeyG => {
                    self.mode = if self.mode == GameMode::God { GameMode::Normal } else { GameMode::God };
                    if self.mode == GameMode::God {
                        self.camera.up = [0.0, 1.0, 0.0];
                        self.visual_up = [0.0, 1.0, 0.0];
                    }
                }
                KeyCode::KeyF => self.flashlight = !self.flashlight,
                KeyCode::KeyN => self.is_day = !self.is_day,
                KeyCode::Digit1 => { self.active_weapon = Weapon::Creator; self.selected_material = 1; }
                KeyCode::Digit2 => { self.active_weapon = Weapon::Creator; self.selected_material = 2; }
                KeyCode::Digit3 => { self.active_weapon = Weapon::Creator; self.selected_material = 3; }
                KeyCode::Digit4 => { self.active_weapon = Weapon::Creator; self.selected_material = 5; }
                KeyCode::Digit5 => self.active_weapon = Weapon::Plasma,
                KeyCode::Digit6 => self.active_weapon = Weapon::Bazooka,
                _ => {}
            }
        }
    }

    pub fn handle_mouse_click(&mut self, pressed: bool) { self.is_shooting = pressed; }
    pub fn handle_mouse_move(&mut self, dx: f64, dy: f64) { self.camera.mouse_move(dx, dy, self.mode == GameMode::God); }

    pub fn update(&mut self, dt: f32) {
        if self.cooldown > 0.0 { self.cooldown -= dt; }
        if self.mode == GameMode::God {
            physics::update_god_mode(self, dt);
        } else {
            physics::update_survival(self, dt);
        }
    }

    pub fn get_shader_action(&mut self) -> u32 {
        if self.is_shooting {
            match self.active_weapon {
                Weapon::Creator => self.selected_material,
                Weapon::Plasma => 8,
                Weapon::Bazooka => { if self.cooldown <= 0.0 { self.cooldown = 0.3; 9 } else { 0 } }
            }
        } else { 0 }
    }
}