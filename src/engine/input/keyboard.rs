use winit::keyboard::KeyCode;

pub struct InputState {
    keys: [bool; 512],
}

impl InputState {
    pub fn new() -> Self {
        Self { keys: [false; 512] }
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
}