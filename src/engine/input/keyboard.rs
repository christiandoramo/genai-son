use winit::keyboard::KeyCode;

pub struct InputState {
    keys: [bool; 512],
    keys_prev: [bool; 512],
}

impl InputState {
    pub fn new() -> Self {
        Self {
            keys: [false; 512],
            keys_prev: [false; 512],
        }
    }

    pub fn update_key(&mut self, key: KeyCode, pressed: bool) {
        let code = key as usize;
        if code < 512 {
            self.keys[code] = pressed;
        }
    }

    pub fn is_pressed(&self, key: KeyCode) -> bool {
        self.keys[key as usize]
    }

    pub fn just_pressed(&self, key: KeyCode) -> bool {
        self.keys[key as usize] && !self.keys_prev[key as usize]
    }

    pub fn tick(&mut self) {
        self.keys_prev = self.keys;
    }
}
