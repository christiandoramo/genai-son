use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::window::CursorGrabMode;

mod camera;
mod player;
mod world;
mod hud;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "PROTÓTIPO 11: O Planeta Otimizado".into(),
                resolution: bevy::window::WindowResolution::new(1024.0, 768.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin) // O Leitor de FPS
        .insert_resource(ClearColor(Color::srgb(0.4, 0.7, 0.9))) 
        .init_resource::<world::VoxelWorld>()
        // A luz agora é instanciada dentro de gerar_mundo
        .add_systems(Startup, (world::gerar_mundo, player::spawn_player, hud::setup_hud))
        .add_systems(Update, (
            gerenciar_cursor,
            player::movimento_e_fisica,
            camera::sync_camera,
            hud::atualizar_hud
        ))
        .run();
}

fn gerenciar_cursor(
    mut windows: Query<&mut Window>,
    input_teclado: Res<ButtonInput<KeyCode>>,
    input_mouse: Res<ButtonInput<MouseButton>>,
) {
    let mut window = windows.single_mut();
    if input_teclado.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
    if input_mouse.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }
}