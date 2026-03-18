use crate::game::player::math_util::*;

pub struct Camera {
    pub pos: [f32; 3],
    pub local_forward: [f32; 3],
    pub up: [f32; 3],
    pub pitch: f32,
}

impl Camera {
    pub fn new(pos: [f32; 3]) -> Self {
        Self { pos, local_forward: [0.0, 0.0, -1.0], up: [0.0, 1.0, 0.0], pitch: 0.0 }
    }

    pub fn mouse_move(&mut self, dx: f64, dy: f64) {
        let sensitivity = 0.003;
        self.local_forward = normalize_or_zero(rotate_vector(self.local_forward, self.up, (dx as f32) * sensitivity));
        self.pitch += (dy as f32) * sensitivity;
        self.pitch = self.pitch.clamp(-1.55, 1.55);
        let right = normalize_or_zero(cross(self.local_forward, self.up));
        self.local_forward = normalize_or_zero(cross(self.up, right));
    }

    pub fn reorient(&mut self, new_up: [f32; 3]) {
        let new_up = normalize_or_zero(new_up);
        let d = dot(self.up, new_up).clamp(-1.0, 1.0);
        if d < 0.99999 {
            let axis = normalize_or_zero(cross(self.up, new_up));
            let angle = d.acos();
            self.local_forward = normalize_or_zero(rotate_vector(self.local_forward, axis, angle));
        }
        self.up = new_up;
        let right = normalize_or_zero(cross(self.local_forward, self.up));
        self.local_forward = normalize_or_zero(cross(self.up, right));
    }

    pub fn get_front(&self) -> [f32; 3] {
        let right = normalize_or_zero(cross(self.up, self.local_forward));
        normalize_or_zero(rotate_vector(self.local_forward, right, self.pitch))
    }

    pub fn get_right(&self) -> [f32; 3] {
        normalize_or_zero(cross(self.up, self.local_forward))
    }
}