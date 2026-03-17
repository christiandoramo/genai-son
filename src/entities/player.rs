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
    pub freeze_time: bool, 
    pub is_shooting: bool,
    pub cooldown: f32, 
    keys: [bool; 255],
}

impl Player {
    pub fn new(start_pos: [f32; 3]) -> Self {
        Self {
            camera: Camera::new(start_pos),
            mode: GameMode::God,
            active_weapon: Weapon::Creator,
            selected_material: 1, 
            flashlight: false,
            freeze_time: false,
            is_shooting: false,
            cooldown: 0.0,
            keys: [false; 255],
        }
    }

    pub fn handle_keyboard(&mut self, keycode: KeyCode, pressed: bool) {
        let code = keycode as usize;
        if code < 255 { self.keys[code] = pressed; }

        if pressed {
            match keycode {
                KeyCode::KeyG => self.mode = match self.mode { GameMode::God => GameMode::Normal, GameMode::Normal => GameMode::God },
                KeyCode::KeyF => self.flashlight = !self.flashlight,
                KeyCode::KeyN => self.freeze_time = !self.freeze_time, // Para o Sol!
                
                KeyCode::Digit1 => { self.active_weapon = Weapon::Creator; self.selected_material = 1; }
                KeyCode::Digit2 => { self.active_weapon = Weapon::Creator; self.selected_material = 2; }
                KeyCode::Digit3 => { self.active_weapon = Weapon::Creator; self.selected_material = 3; }
                KeyCode::Digit4 => { self.active_weapon = Weapon::Creator; self.selected_material = 5; } // Terra (Flutua)
                
                KeyCode::Digit5 => self.active_weapon = Weapon::Plasma,
                KeyCode::Digit6 => self.active_weapon = Weapon::Bazooka,
                _ => {}
            }
        }
    }

    pub fn handle_mouse_click(&mut self, pressed: bool) { self.is_shooting = pressed; }
    pub fn handle_mouse_move(&mut self, dx: f64, dy: f64) { self.camera.mouse_move(dx, dy); }

    pub fn update(&mut self, dt: f32) {
        if self.cooldown > 0.0 { self.cooldown -= dt; }

        let speed = 40.0 * dt;
        let front = self.camera.get_front();
        let right = self.camera.get_right();
        let up = self.camera.get_up(); // Pega a inclinação da nuca

        if self.keys[KeyCode::KeyW as usize] { self.camera.pos[0] += front[0] * speed; if self.mode == GameMode::God { self.camera.pos[1] += front[1] * speed; } self.camera.pos[2] += front[2] * speed; }
        if self.keys[KeyCode::KeyS as usize] { self.camera.pos[0] -= front[0] * speed; if self.mode == GameMode::God { self.camera.pos[1] -= front[1] * speed; } self.camera.pos[2] -= front[2] * speed; }
        if self.keys[KeyCode::KeyA as usize] { self.camera.pos[0] -= right[0] * speed; self.camera.pos[2] -= right[2] * speed; }
        if self.keys[KeyCode::KeyD as usize] { self.camera.pos[0] += right[0] * speed; self.camera.pos[2] += right[2] * speed; }
        
        if self.mode == GameMode::God {
            // Sobe/Desce relativo à visão da câmera!
            if self.keys[KeyCode::KeyE as usize] { self.camera.pos[0] += up[0] * speed; self.camera.pos[1] += up[1] * speed; self.camera.pos[2] += up[2] * speed; }
            if self.keys[KeyCode::KeyQ as usize] { self.camera.pos[0] -= up[0] * speed; self.camera.pos[1] -= up[1] * speed; self.camera.pos[2] -= up[2] * speed; }
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