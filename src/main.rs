use bevy::prelude::*;
use bevy::window::CursorGrabMode;

mod camera;
mod player;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "PROTÓTIPO 9: Terceira Pessoa e Física Real".into(),
                resolution: bevy::window::WindowResolution::new(1024.0, 768.0),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.4, 0.7, 0.9))) // Céu azul
        .init_resource::<world::VoxelWorld>()
        .add_systems(Startup, (world::gerar_mundo, player::spawn_player))
        .add_systems(Update, (
            gerenciar_cursor,
            player::movimento_e_fisica,
            camera::sync_camera
        ))
        .run();
}

// Prende o mouse no centro da tela (Estilo Minecraft/Jogos 3D)
fn gerenciar_cursor(
    mut windows: Query<&mut Window>,
    input_teclado: Res<ButtonInput<KeyCode>>,
    input_mouse: Res<ButtonInput<MouseButton>>,
) {
    let mut window = windows.single_mut();
    
    // Esc para soltar o mouse
    if input_teclado.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
    
    // Clique esquerdo para prender o mouse de volta
    if input_mouse.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }
}