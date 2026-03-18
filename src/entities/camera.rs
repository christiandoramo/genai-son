pub struct Camera {
    pub pos: [f32; 3],
    pub local_forward: [f32; 3], // Sempre 100% paralelo ao chão
    pub up: [f32; 3],
    pub pitch: f32, // Inclinação da cabeça (Cima/Baixo)
}

impl Camera {
    pub fn new(start_pos: [f32; 3]) -> Self {
        Self { 
            pos: start_pos, 
            local_forward: [0.0, 0.0, -1.0], 
            up: [0.0, 1.0, 0.0],
            pitch: 0.0,
        }
    }

    pub fn mouse_move(&mut self, dx: f64, dy: f64, is_god_mode: bool) {
        let sensitivity = 0.003;
        let up_axis = if is_god_mode { [0.0, 1.0, 0.0] } else { self.up };

        // Gira o corpo (Esquerda/Direita)
        self.local_forward = math::normalize_or_zero(math::rotate_vector(
            self.local_forward, up_axis, (dx as f32) * sensitivity
        ));

        // Gira apenas a cabeça (Cima/Baixo) com trava de limite (89 graus)
        self.pitch -= (dy as f32) * sensitivity;
        self.pitch = self.pitch.clamp(-1.55, 1.55);

        // Garante que o corpo nunca desalinhe do horizonte
        let right = math::normalize_or_zero(math::cross(self.local_forward, up_axis));
        self.local_forward = math::normalize_or_zero(math::cross(up_axis, right));
    }

    pub fn reorient(&mut self, new_up: [f32; 3]) {
        let new_up = math::normalize_or_zero(new_up);
        let dot = math::dot(self.up, new_up).clamp(-1.0, 1.0);
        
        if dot < 0.99999 {
            let axis = math::normalize_or_zero(math::cross(self.up, new_up));
            let angle = dot.acos();
            self.local_forward = math::normalize_or_zero(math::rotate_vector(self.local_forward, axis, angle));
        }
        self.up = new_up;

        // Limpeza de erros matemáticos
        let right = math::normalize_or_zero(math::cross(self.local_forward, self.up));
        self.local_forward = math::normalize_or_zero(math::cross(self.up, right));
    }

    pub fn get_front(&self) -> [f32; 3] {
        // A mágica: O shader recebe a visão final combinando o corpo e a cabeça!
        let right = math::normalize_or_zero(math::cross(self.local_forward, self.up));
        math::normalize_or_zero(math::rotate_vector(self.local_forward, right, self.pitch))
    }

    pub fn get_right(&self) -> [f32; 3] {
        math::normalize_or_zero(math::cross(self.local_forward, self.up))
    }
}

pub mod math {
    pub fn dot(a: [f32; 3], b: [f32; 3]) -> f32 { a[0]*b[0] + a[1]*b[1] + a[2]*b[2] }
    pub fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] { [ a[1]*b[2] - a[2]*b[1], a[2]*b[0] - a[0]*b[2], a[0]*b[1] - a[1]*b[0] ] }
    pub fn length(v: [f32; 3]) -> f32 { (v[0]*v[0] + v[1]*v[1] + v[2]*v[2]).sqrt() }
    
    pub fn normalize_or_zero(v: [f32; 3]) -> [f32; 3] {
        let len = length(v);
        if len < 0.00001 { [0.0, 0.0, 0.0] } else { [v[0]/len, v[1]/len, v[2]/len] }
    }
    
    pub fn slerp(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
        let dot = dot(a, b).clamp(-1.0, 1.0);
        if dot > 0.9995 { return normalize_or_zero([a[0] + (b[0] - a[0])*t, a[1] + (b[1] - a[1])*t, a[2] + (b[2] - a[2])*t]); }
        let theta = dot.acos() * t;
        let relative_vec = normalize_or_zero([b[0] - a[0]*dot, b[1] - a[1]*dot, b[2] - a[2]*dot]);
        [
            a[0]*theta.cos() + relative_vec[0]*theta.sin(),
            a[1]*theta.cos() + relative_vec[1]*theta.sin(),
            a[2]*theta.cos() + relative_vec[2]*theta.sin()
        ]
    }

    pub fn rotate_vector(v: [f32; 3], k: [f32; 3], angle: f32) -> [f32; 3] {
        let cos_t = angle.cos(); let sin_t = angle.sin(); let cross_kv = cross(k, v); let dot_kv = dot(k, v);
        [ v[0] * cos_t + cross_kv[0] * sin_t + k[0] * dot_kv * (1.0 - cos_t), v[1] * cos_t + cross_kv[1] * sin_t + k[1] * dot_kv * (1.0 - cos_t), v[2] * cos_t + cross_kv[2] * sin_t + k[2] * dot_kv * (1.0 - cos_t) ]
    }
}