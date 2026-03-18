pub mod mouse;
pub mod keyboard;

pub struct InputController {
    pub mouse: mouse::MouseState,
    pub keyboard: keyboard::InputState,
}

impl InputController {
    pub fn new() -> Self {
        Self {
            mouse: mouse::MouseState::new(),
            keyboard: keyboard::InputState::new(),
        }
    }
}