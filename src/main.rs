mod engine;
mod game;

use crate::engine::state::State;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

#[derive(Default)]
struct App<'a> {
    state: Option<State<'a>>,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes().with_title("GenAI Son: Voxel Revolution"),
                )
                .unwrap(),
        );
        self.state = Some(pollster::block_on(State::new(window)));
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();
        match event {
            WindowEvent::CloseRequested => _event_loop.exit(),
            WindowEvent::Resized(new_size) => state.device.resize(new_size),

            // TECLADO: Alimenta state.inputs.keyboard
            WindowEvent::KeyboardInput { event, .. } => {
                if let winit::keyboard::PhysicalKey::Code(key) = event.physical_key {
                    state
                        .inputs
                        .keyboard
                        .update_key(key, event.state.is_pressed());
                }
            }

            // CLIQUE DO MOUSE: Alimenta state.inputs.mouse
            WindowEvent::MouseInput {
                state: m_state,
                button: winit::event::MouseButton::Left,
                ..
            } => {
                state.inputs.mouse.left_pressed = m_state.is_pressed();
                state.player.is_shooting = m_state.is_pressed(); // Sincroniza com o player
            }

            WindowEvent::RedrawRequested => {
                state.update();
                let _ = state.render();
                state.window.request_redraw();
            }
            _ => {}
        }
    }

    fn device_event(&mut self, _: &ActiveEventLoop, _: winit::event::DeviceId, event: DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta } = event {
            if let Some(state) = &mut self.state {
                // ACUMULA O DELTA: O update() do state.rs vai consumir isso
                state.inputs.mouse.delta.0 += delta.0;
                state.inputs.mouse.delta.1 += delta.1;
            }
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
