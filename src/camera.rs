pub struct Camera {
    pub pos: [f32; 3],
    pub yaw: f32,
    pub pitch: f32,
}

impl Camera {
    pub fn new(start_pos: [f32; 3]) -> Self {
        Self { pos: start_pos, yaw: std::f32::consts::FRAC_PI_2, pitch: 0.0 }
    }

    pub fn mouse_move(&mut self, dx: f64, dy: f64) {
        let sensitivity = 0.003;
        self.yaw -= (dx as f32) * sensitivity;
        self.pitch -= (dy as f32) * sensitivity;
        self.pitch = self.pitch.clamp(-1.56, 1.56);
    }

    pub fn get_front(&self) -> [f32; 3] {
        [self.yaw.cos() * self.pitch.cos(), self.pitch.sin(), self.yaw.sin() * self.pitch.cos()]
    }

    pub fn get_right(&self) -> [f32; 3] {
        let front = self.get_front();
        let right = [front[2], 0.0, -front[0]];
        let right_len = (right[0] * right[0] + right[2] * right[2]).sqrt().max(0.001);
        [right[0] / right_len, 0.0, right[2] / right_len]
    }

    // NOVO: Descobre qual é o "Cima" em relação a para onde você está olhando
    pub fn get_up(&self) -> [f32; 3] {
        let front = self.get_front();
        let right = self.get_right();
        [
            right[1] * front[2] - right[2] * front[1],
            right[2] * front[0] - right[0] * front[2],
            right[0] * front[1] - right[1] * front[0]
        ]
    }
}