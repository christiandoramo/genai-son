pub struct MouseState {
    pub delta: (f64, f64),
    pub left_pressed: bool,
}

impl MouseState {
    pub fn new() -> Self {
        Self { delta: (0.0, 0.0), left_pressed: false }
    }
}