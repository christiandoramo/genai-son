mod world;
mod entities;
mod graphics;
mod engine;

use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{WindowEvent, DeviceEvent, ElementState},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId, CursorGrabMode},
    keyboard::PhysicalKey,
};
use engine::State;

#[derive(Default)]
struct App<'a> { state: Option<State<'a>> }

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("GenAI Son: Custom Voxel Engine")
                .with_inner_size(winit::dpi::PhysicalSize::new(1280, 720));
            
            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            let _ = window.set_cursor_grab(CursorGrabMode::Confined).or_else(|_| window.set_cursor_grab(CursorGrabMode::Locked));
            window.set_cursor_visible(false);

            self.state = Some(pollster::block_on(State::new(window)));
        }
    }

    fn device_event(&mut self, _event_loop: &ActiveEventLoop, _device_id: winit::event::DeviceId, event: DeviceEvent) {
        if let Some(state) = &mut self.state {
            if let DeviceEvent::MouseMotion { delta } = event {
                state.player.handle_mouse_move(delta.0, delta.1);
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        let state = match &mut self.state { Some(state) => state, None => return };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(physical_size) => state.resize(physical_size),
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(keycode) = event.physical_key {
                    if keycode == winit::keyboard::KeyCode::Escape { event_loop.exit(); }
                    state.player.handle_keyboard(keycode, event.state == ElementState::Pressed);
                }
            }
            WindowEvent::MouseInput { state: mouse_state, button: winit::event::MouseButton::Left, .. } => {
                state.player.handle_mouse_click(mouse_state == ElementState::Pressed);
            }
            WindowEvent::RedrawRequested => {
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.gpu.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(state) = &self.state { state.window.request_redraw(); }
    }
}

pub fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}